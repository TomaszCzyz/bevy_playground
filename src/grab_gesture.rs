use bevy::prelude::*;
use mac::unwrap_or_return;

use leap_input::leap_controller_plugin::{BoneComponent, BoneType, DigitType};

use crate::MainGizmo;

#[derive(Component)]
pub struct ObjectBounds;

#[derive(Clone, Default, Resource)]
pub struct GrabData {
    current_entity: Option<Entity>,
    previous_entity: Option<Entity>,
    start_hands_transform: Transform,
    start_obj_transform: Transform,
    digits_involved: Vec<DigitType>,
}

impl GrabData {
    fn clear(&mut self) {
        self.previous_entity = self.current_entity;
        self.current_entity = None;
        self.digits_involved.clear();
    }

    fn update(&mut self, entity: Entity, start_hands_transform: Transform, start_obj_transform: Transform) {
        self.previous_entity = self.current_entity;
        self.current_entity = Some(entity);
        self.start_hands_transform = start_hands_transform;
        self.start_obj_transform = start_obj_transform;
    }
}

pub fn detect_obj_grabbing(
    mut grab_res: ResMut<GrabData>,
    main_gizmo_query: Query<(Entity, &Transform), With<MainGizmo>>,
    digits_query: Query<(&Transform, &BoneComponent)>,
) {
    let (entity, main_gizmo_transform) = main_gizmo_query.single();

    let digits_inside_bounds = digits_query
        .iter()
        .filter(|(_, bone)| bone.bone_type == BoneType::Distal)
        .filter(|(t, _)| t.translation.distance(main_gizmo_transform.translation) < 35.)
        .collect::<Vec<_>>();

    update_grab_resource(&mut grab_res, entity, main_gizmo_transform, digits_inside_bounds)
}

fn update_grab_resource(
    grab_res: &mut ResMut<GrabData>,
    entity: Entity,
    main_gizmo_transform: &Transform,
    digits_inside_bounds: Vec<(&Transform, &BoneComponent)>,
) {
    match grab_res.current_entity {
        None => {
            if digits_inside_bounds.len() < 3 {
                return;
            }

            // start new grabbing
            let mut fingers_center = Vec3::ZERO;
            for (t, b) in digits_inside_bounds.iter() {
                fingers_center += t.translation;
                grab_res.digits_involved.push(b.digit_type);
            }

            fingers_center /= digits_inside_bounds.len() as f32;

            grab_res.update(
                entity,
                Transform::from_translation(fingers_center),
                Transform::from_translation(main_gizmo_transform.translation),
            )
        }
        Some(_) => {
            if digits_inside_bounds.len() >= 3 {
                // grabbing in progress
                return;
            }

            // end of a grabbing; clear resource
            grab_res.clear();
        }
    }
}

pub fn update_grabbed_obj_transform(
    grab_res: Res<GrabData>,
    digits_query: Query<(&Transform, &BoneComponent)>,
    mut transform_query: Query<&mut Transform, (With<MainGizmo>, Without<BoneComponent>)>,
) {
    let grabbed_entity = unwrap_or_return!(grab_res.current_entity, ());
    let mut grabbed_entity_transform = transform_query.get_mut(grabbed_entity).unwrap();

    let mut involved_digits_center = Vec3::ZERO;
    for (t, b) in digits_query.iter() {
        if b.bone_type == BoneType::Distal && grab_res.digits_involved.contains(&b.digit_type) {
            involved_digits_center += t.translation;
        }
    }

    involved_digits_center /= grab_res.digits_involved.len() as f32;

    grabbed_entity_transform.translation = grab_res.start_obj_transform.translation
        + (involved_digits_center - grab_res.start_hands_transform.translation);
}

pub fn update_grabbed_obj_transparency(
    grab_res: Res<GrabData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // todo: if we could query parent entity directly (or store child entity) then we could use
    // todo: shorter and more performant 'query.get_mut(entity)'
    mut material_query: Query<(&Parent, &Handle<StandardMaterial>), With<ObjectBounds>>,
) {
    if !grab_res.is_changed() {
        return;
    }

    if let Some(grab_entity) = grab_res.current_entity {
        for (parent_entity, material_handle) in material_query.iter_mut() {
            if parent_entity.get() != grab_entity {
                continue;
            }

            let some_color = &mut materials.get_mut(material_handle).unwrap().base_color;
            some_color.set_a(0.8);
        }
    } else {
        match grab_res.previous_entity {
            None => {
                return;
            }
            Some(previous_entity) => {
                for (parent_entity, material_handle) in material_query.iter_mut() {
                    if parent_entity.get() != previous_entity {
                        continue;
                    }

                    let some_color = &mut materials.get_mut(material_handle).unwrap().base_color;
                    info!("updating transparency");
                    some_color.set_a(0.4);
                }
            }
        }
    }
}
