use crate::leap_controller_plugin::{HandPart, HandsData};
use bevy::prelude::*;
use std::f32::consts::PI;
use bevy::ecs::query::QueryIter;

pub fn display_hands(
    hands_res: Res<HandsData>,
    mut bone_meshes_query: Query<(&mut Visibility, &mut Transform), With<HandPart>>,
) {
    let all_hands_bones = hands_res
        .hands
        .iter()
        .flat_map(|hand| hand.digits)
        .map(|x| [x.metacarpal, x.proximal, x.intermediate, x.distal])
        .flat_map(|x1| x1);

    let mut bones_iterator = all_hands_bones.into_iter();
    let mut query_iterator = bone_meshes_query.iter_mut();

    while let Some((mut visibility, mut transform)) = query_iterator.next() {
        let _ = match bones_iterator.next() {
            None => {
                hide_rest_meshes(&mut query_iterator, &mut visibility);
                break;
            }
            Some(bone) => {
                transform.translation = bone.next_joint;
                transform.rotation = bone.rotation * Quat::from_rotation_x(PI / 2.);
                visibility.is_visible = true;
            }
        };
    }
}

fn hide_rest_meshes(
    query_iterator: &mut QueryIter<(&mut Visibility, &mut Transform), With<HandPart>>,
    visibility: &mut Visibility,
) {
    visibility.is_visible = false;

    while let Some((mut visibility, _)) = query_iterator.next() {
        visibility.is_visible = false;
    }
}
