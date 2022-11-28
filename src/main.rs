use bevy::prelude::shape::Icosphere;
use bevy::prelude::*;
use bevy::render::mesh::shape::{Box};

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
        .add_system(analyze_digits_position)
        .run();
}

/// Main camera. Hands' Transform is calculated in relation to it
#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct ObjectBounds;

fn analyze_digits_position(
    hands_res: Res<HandsData>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    meshes: Res<Assets<Mesh>>,
    bounds_query: Query<(&Handle<Mesh>, &Handle<StandardMaterial>), With<ObjectBounds>>,
) {
    if hands_res.hands.len() == 0 {
        return;
    }

    let right_finger_tips = hands_res.hands[0]
        .digits
        .iter()
        .map(|digit| digit.distal.next_joint)
        .collect::<Vec<Vec3>>();

    // if there are at least 3 finger tips within bounds, then change transparency
    let finger_tips_inside_bounds = right_finger_tips
        .iter()
        .filter(|vec3| vec3.distance(Vec3::new(100., 250., 0.)) < 35.)
        .count();

    let (_mesh_handle, material_handle) = bounds_query.single();

    // todo: get this only if value has changed?..
    let some_color = &mut materials.get_mut(material_handle).unwrap().base_color;

    if finger_tips_inside_bounds >= 3 {
        some_color.set_a(0.8);
    } else {
        some_color.set_a(0.4);
    }

    // for (_mesh_handle, material_handle) in bounds_query.iter() {
    //     // let x = meshes.get(mesh_handle).unwrap();
    //
    //     let some_color = &mut materials.get_mut(material_handle).unwrap().base_color;
    //     some_color.set_a(0.8);
    // }
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
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Icosphere {
                radius: 20.,
                subdivisions: 12,
            })),
            transform: Transform::from_xyz(100., 250., 0.),
            material: materials.add(Color::rgb_u8(50, 224, 229).into()),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Icosphere {
                        radius: 35.,
                        subdivisions: 12,
                    })),
                    material: materials.add(Color::rgba_u8(172, 229, 88, 50).into()),
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
    camera_query: Query<&Transform, (With<PlayerCamera>, Without<HandsOrigin>)>,
) {
    let mut hands_origin_transform = hand_origin_query.single_mut();
    let camera_transform = camera_query.single();

    // *hands_origin_transform = Transform {
    //     translation: camera_transform.translation + camera_transform.forward() * HANDS_DISTANCE,// - Vec3::Y * 300.,
    //     rotation: Quat::from_axis_angle(Vec3::X, PI),
    //     // rotation: camera_transform.rotation * Quat::from_euler(EulerRot::YXZ, 0., PI / 4., 0.),
    //     ..default()
    // };
    let transform = Transform::IDENTITY;
    *hands_origin_transform = transform;
}
