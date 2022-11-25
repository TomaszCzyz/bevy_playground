use std::f32::consts::{PI, TAU};
use std::num::FpCategory::Zero;
use std::ops::Add;
use std::time::Duration;

use bevy::prelude::*;

use leap_input::display::display_hands;
#[allow(unused_imports)]
use leap_input::leap_controller_plugin::{HandPart, HandsData, HandsOrigin, LeapControllerPlugin};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

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
        .add_system(display_hands)
        .add_system(move_camera)
        .add_system(orbit_camera)
        .add_system(adjust_hands_origin_to_camera_transform)
        .run();
}

#[derive(Component)]
pub struct PlayerCamera {
    speed: f32,
}

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
            transform: Transform::from_xyz(0., 500., 500.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PlayerCamera { speed: 0.3 },
    ));
}

fn move_camera(mut camera_query: Query<(&mut Transform, &PlayerCamera)>, timer: Res<Time>) {
    for (mut transform, camera) in &mut camera_query {
        let forward = transform.left();
        transform.translation += forward * 150.0 * timer.delta_seconds();
    }
}

fn orbit_camera(mut camera_query: Query<(&mut Transform, &PlayerCamera)>, timer: Res<Time>) {
    for (mut transform, _) in &mut camera_query {
        let look_at_center = transform.looking_at(Vec3::ZERO, transform.local_y());
        let incremental_turn_weight = 20. * timer.delta_seconds();
        let old_rotation = transform.rotation;

        transform.rotation = old_rotation.lerp(look_at_center.rotation, incremental_turn_weight);
    }
}

fn adjust_hands_origin_to_camera_transform(
    mut hand_origin_query: Query<&mut Transform, With<HandsOrigin>>,
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<HandsOrigin>)>,
) {
    for mut hands_origin_transform in hand_origin_query.iter_mut() {
        for camera_transform in camera_query.iter() {
            *hands_origin_transform = Transform {
                // translation: camera_transform.translation.lerp(Vec3::ZERO, 0.8),
                translation: camera_transform.translation + camera_transform.forward() * 500.,
                rotation: camera_transform.rotation * Quat::from_euler(EulerRot::YXZ, 0., PI / 2., 0.),
                ..default()
            };
        }
    }
}
