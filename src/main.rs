use std::f32::consts::PI;

use bevy::prelude::*;

use leap_input::leap_controller_plugin::{HandsOrigin, LeapControllerPlugin};

mod helpers;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
pub const CAMERA_ORIGIN: Transform = Transform::from_xyz(0., 500., 500.0);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
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
        // .add_system(move_camera)
        // .add_system(orbit_camera)
        .add_system(adjust_hands_origin_to_camera_transform)
        .run();
}

/// Main camera. Hands' Transform is calculated in relation to it
#[derive(Component)]
pub struct PlayerCamera;

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 150.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 50.0 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0, 50., 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 1000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(80.0, 160.0, 80.0),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: CAMERA_ORIGIN.looking_at(Vec3::ZERO, Vec3::Y),
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
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<HandsOrigin>)>,
) {
    let mut hands_origin_transform = hand_origin_query.single_mut();
    let camera_transform = camera_query.single();

    *hands_origin_transform = Transform {
        translation: camera_transform.translation + camera_transform.forward() * 500.,
        rotation: camera_transform.rotation * Quat::from_euler(EulerRot::YXZ, 0., PI / 2., 0.),
        ..default()
    };
}
