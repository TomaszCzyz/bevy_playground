use bevy::math::Vec3;
use bevy::prelude::{Query, Res, Time, Transform};
use crate::PlayerCamera;

#[allow(dead_code)]
fn move_camera(mut camera_query: Query<(&mut Transform, &PlayerCamera)>, timer: Res<Time>) {
    for (mut transform, _) in &mut camera_query {
        let forward = transform.left();
        transform.translation += forward * 150.0 * timer.delta_seconds();
    }
}

#[allow(dead_code)]
fn orbit_camera(mut camera_query: Query<(&mut Transform, &PlayerCamera)>, timer: Res<Time>) {
    for (mut transform, _) in &mut camera_query {
        let look_at_center = transform.looking_at(Vec3::ZERO, transform.local_y());
        let incremental_turn_weight = 20. * timer.delta_seconds();
        let old_rotation = transform.rotation;

        transform.rotation = old_rotation.lerp(look_at_center.rotation, incremental_turn_weight);
    }
}