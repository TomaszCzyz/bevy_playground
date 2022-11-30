use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::prelude::shape::Icosphere;
use bevy::render::mesh::shape::Box;
use bevy_editor_pls::prelude::*;
use mac::unwrap_or_return;

use leap_input::leap_controller_plugin::{HandsData, HandsOrigin, LeapControllerPlugin};

use crate::grab_gesture::{detect_obj_grabbing, GrabData, ObjectBounds, update_grabbed_obj_transform, update_grabbed_obj_transparency};

mod helpers;
mod shape;
mod grab_gesture;

pub const HEIGHT: f32 = 1080.;
pub const WIDTH: f32 = 1920.;
pub const TABLE_SIZE: [f32; 3] = [1600., 700., 1600.];
pub const HANDS_DISTANCE: f32 = 800.;
pub const CAMERA_ORIGIN: Transform = Transform::from_xyz(0., 350., 500.);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(GrabData::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(EntityCountDiagnosticsPlugin::default())
        .add_plugin(EditorPlugin)
        .add_plugin(LeapControllerPlugin)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_basic_scene)
        .add_system(detect_obj_grabbing)
        .add_system(adjust_hands_origin_to_camera_transform)
        .add_system(update_grabbed_obj_transform)
        .add_system(update_grabbed_obj_transparency)
        // .add_system(print_grab_strength)
        .run();
}

/// Main camera. Hands' Transform is calculated in relation to it
#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct MainGizmo;

fn print_grab_strength(hands_res: Res<HandsData>) {
    let hand = unwrap_or_return!(hands_res.hands.get(0), ());

    info!(
        "grab strength: {} \t\t pinch strength: {}",
        hand.grab_strength, hand.pinch_strength
    )
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
