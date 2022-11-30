use bevy::prelude::*;
use mac::unwrap_or_return;

use leap_input::leap_controller_plugin::HandsData;

use crate::MainGizmo;

#[derive(Component)]
pub struct ObjectBounds;

#[derive(Clone, Default, Resource)]
pub struct GrabData {
    entity: Option<Entity>,
    previous_entity: Option<Entity>,
    start_hands_transform: Transform,
    start_obj_transform: Transform,
}

impl GrabData {
    fn clear(&mut self) {
        self.previous_entity = self.entity;
        self.entity = None;
    }

    fn update(&mut self, entity: Entity, start_hands_transform: Transform, start_obj_transform: Transform) {
        self.previous_entity = self.entity;
        self.entity = Some(entity);
        self.start_hands_transform = start_hands_transform;
        self.start_obj_transform = start_obj_transform;
    }
}

pub fn detect_obj_grabbing(
    mut grab_res: ResMut<GrabData>,
    hands_res: Res<HandsData>,
    main_gizmo_query: Query<(Entity, &Transform), With<MainGizmo>>,
) {
    let hand = unwrap_or_return!(hands_res.hands.get(0), ());

    let (entity, transform) = main_gizmo_query.single();

    let right_finger_tips = hand
        .digits
        .iter()
        .map(|digit| digit.distal.next_joint)
        .collect::<Vec<Vec3>>();

    let finger_tips_inside_bounds = right_finger_tips
        .iter()
        .filter(|vec3| vec3.distance(transform.translation) < 40.)
        .collect::<Vec<_>>();

    match grab_res.clone().entity {
        None => {
            if finger_tips_inside_bounds.len() >= 3 {
                // start new grabbing

                let mut fingers_center: Vec3 = finger_tips_inside_bounds.iter().copied().sum();

                fingers_center /= finger_tips_inside_bounds.len() as f32;

                grab_res.update(
                    entity,
                    Transform::from_translation(hand.palm.position),
                    Transform::from_translation(transform.translation),
                )
            }
        }
        Some(_) => {
            if finger_tips_inside_bounds.len() < 3 {
                // end of a grabbing; clear resource
                grab_res.clear();
            }
        }
    }
}

pub fn update_grabbed_obj_transform(
    grab_res: Res<GrabData>,
    hands_res: Res<HandsData>,
    mut transform_query: Query<&mut Transform>,
) {
    let hand = unwrap_or_return!(hands_res.hands.get(0), ());
    let grabbed_entity = unwrap_or_return!(grab_res.entity, ());

    // let mut fingers_center: Vec3 = finger_tips_inside_bounds
    //     .iter()
    //     .copied()
    //     .sum();

    let mut transform = transform_query.get_mut(grabbed_entity).unwrap();
    let hand_move_delta = hand.palm.position - grab_res.start_hands_transform.translation;

    transform.translation = grab_res.start_obj_transform.translation + hand_move_delta;
}

pub fn update_grabbed_obj_transparency(
    grab_res: Res<GrabData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // todo: if we could query parent entity directly (or store child entity) then we could use
    // shorter and more performant 'query.get_mut(entity)'
    mut material_query: Query<(&Parent, &Handle<StandardMaterial>), With<ObjectBounds>>,
) {
    if !grab_res.is_changed() {
        return;
    }

    if let Some(grab_entity) = grab_res.entity {
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
