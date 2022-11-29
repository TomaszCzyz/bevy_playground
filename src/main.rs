use bevy::prelude::shape::Icosphere;
use bevy::prelude::*;
use bevy::render::mesh::shape::Box;
use mac::unwrap_or_return;

use leap_input::leap_controller_plugin::{HandsData, HandsOrigin, LeapControllerPlugin};

mod helpers;
mod shape;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
pub const TABLE_SIZE: [f32; 3] = [1600., 700., 1600.];
pub const HANDS_DISTANCE: f32 = 800.;
pub const CAMERA_ORIGIN: Transform = Transform::from_xyz(0., 350., 500.);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(GrabData::default())
        .add_plugin(LeapControllerPlugin)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_system_to_stage(CoreStage::PreUpdate, detect_obj_grabbing)
        .add_system(adjust_hands_origin_to_camera_transform)
        .add_system(update_grabbed_obj_transform)
        .add_system(update_grabbed_obj_transparency)
        .run();
}

/// Main camera. Hands' Transform is calculated in relation to it
#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct ObjectBounds;

#[derive(Component)]
pub struct MainGizmo;

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

fn update_grabbed_obj_transparency(
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

fn update_grabbed_obj_transform(
    grab_res: Res<GrabData>,
    hands_res: Res<HandsData>,
    mut transform_query: Query<&mut Transform>,
) {
    let hand = unwrap_or_return!(hands_res.hands.get(0), ());
    let grabbed_entity = unwrap_or_return!(grab_res.entity, ());

    let mut transform = transform_query.get_mut(grabbed_entity).unwrap();
    let hand_move_delta = hand.palm.position - grab_res.start_hands_transform.translation;

    transform.translation = grab_res.start_obj_transform.translation + hand_move_delta;
}

fn detect_obj_grabbing(
    mut grab_res: ResMut<GrabData>,
    hands_res: Res<HandsData>,
    main_gizmo_query: Query<(Entity, &Transform), With<MainGizmo>>,
) {
    let hand = unwrap_or_return!(hands_res.hands.get(0), ());

    let (entity, transform): (Entity, &Transform) = main_gizmo_query.single();

    let right_finger_tips = hand
        .digits
        .iter()
        .map(|digit| digit.distal.next_joint)
        .collect::<Vec<Vec3>>();

    let finger_tips_inside_bounds = right_finger_tips
        .iter()
        .filter(|vec3| vec3.distance(transform.translation) < 40.)
        .count();

    match grab_res.clone().entity {
        None => {
            if finger_tips_inside_bounds >= 3 {
                // start new grabbing
                grab_res.update(
                    entity,
                    Transform::from_translation(hand.palm.position),
                    Transform::from_translation(transform.translation),
                )
            }
        }
        Some(_) => {
            if finger_tips_inside_bounds < 3 {
                // end of a grabbing; clear resource
                grab_res.clear();
            }
        }
    }
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // main table
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Box::new(TABLE_SIZE[0], TABLE_SIZE[1], TABLE_SIZE[2]))),
        transform: Transform::from_xyz(0., -TABLE_SIZE[1], 0.),
        material: materials.add(Color::rgb_u8(50, 224, 229).into()),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 2500.0,
            range: 2000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., TABLE_SIZE[1] + 200., 0.),
        ..default()
    });

    // main gizmo
    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(Icosphere {
                    radius: 20.,
                    subdivisions: 12,
                })),
                transform: Transform::from_xyz(100., 250., 0.),
                material: materials.add(Color::rgb_u8(50, 224, 229).into()),
                ..default()
            },
            MainGizmo,
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Icosphere {
                        radius: 35.,
                        subdivisions: 12,
                    })),
                    material: materials.add(Color::rgba_u8(172, 229, 88, 102).into()),
                    ..default()
                },
                ObjectBounds,
            ));
        });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: CAMERA_ORIGIN.looking_at(Vec3::Y * 200., Vec3::Y),
            ..default()
        },
        PlayerCamera,
    ));
}

/// Sets Transform of [`HandsOrigin`] forward from [`PlayerCamera`].
/// System also rotates the [`HandsOrigin`].
///
/// The distance from the camera is constant, but we can use
/// ```
/// translation: camera_transform.translation.lerp(Vec3::ZERO, 0.8),
/// ```
/// to adjust distance as percent of length from e.g. center of the scene
fn adjust_hands_origin_to_camera_transform(
    mut hand_origin_query: Query<&mut Transform, With<HandsOrigin>>,
    // camera_query: Query<&Transform, (With<PlayerCamera>, Without<HandsOrigin>)>,
) {
    let mut hands_origin_transform = hand_origin_query.single_mut();
    // let camera_transform = camera_query.single();

    // *hands_origin_transform = Transform {
    //     translation: camera_transform.translation + camera_transform.forward() * HANDS_DISTANCE,// - Vec3::Y * 300.,
    //     rotation: Quat::from_axis_angle(Vec3::X, PI),
    //     // rotation: camera_transform.rotation * Quat::from_euler(EulerRot::YXZ, 0., PI / 4., 0.),
    //     ..default()
    // };
    let transform = Transform::IDENTITY;
    *hands_origin_transform = transform;
}
