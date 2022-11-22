use std::f32::consts::PI;
use std::ops::Deref;

use bevy::asset::Asset;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::query::QueryIter;
use bevy::math::vec3;
use bevy::prelude::shape::Capsule;
use bevy::prelude::*;
use bevy::render::render_resource::ShaderRef::Handle;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use leaprs::{Connection, ConnectionConfig, Event};

use crate::hand_models::bone::MyBone;
use crate::hand_models::hand::MyHand;
use crate::hand_models::palm::Palm;

mod hand_models;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;
const X_EXTENT: f32 = 80.;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(create_connection)
        .add_startup_system(spawn_basic_scene)
        .add_startup_system(setup_for_hands)
        .add_startup_system(spawn_camera)
        .add_system(update_hand_data)
        .add_system(display_hands.after(update_hand_data))
        .insert_resource(HandsData { hands: default() })
        .run();
}

fn create_connection(world: &mut World) {
    let mut connection = Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    world.insert_non_send_resource(connection);
}

#[derive(Resource)]
struct HandsData {
    hands: Vec<MyHand>,
}

#[derive(Component)]
struct HandPart;

fn update_hand_data(mut hands_res: ResMut<HandsData>, mut leap_conn: NonSendMut<Connection>) {
    if let Ok(message) = leap_conn.poll(25) {
        match &message.event() {
            Event::Connection(_) => println!("connection event"),
            Event::Device(_) => println!("device event"),
            Event::Tracking(e) => {
                hands_res.hands.clear();

                if e.hands().len() == 0 {
                    return;
                }

                for hand in e.hands() {
                    hands_res.hands.push(MyHand::from(hand))
                }
            }
            _ => {}
        }
    }
}

fn setup_for_hands(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let capsule = Capsule {
        radius: 5.,
        rings: 0,
        depth: 10.0,
        ..default()
    };

    let mut shapes = Vec::new();

    for _ in 1..40 {
        shapes.push(meshes.add(capsule.clone().into()));
    }

    // let debug_material = materials.add(StandardMaterial {
    //     base_color_texture: Some(images.add(uv_debug_texture())),
    //     ..default()
    // });

    let debug_material = materials.add(StandardMaterial {
        base_color: Color::rgb_u8(192, 191, 187),
        metallic: 0.3,
        perceptual_roughness: 0.8,
        reflectance: 0.2,
        ..default()
    });

    for shape in shapes.into_iter() {
        commands.spawn((
            PbrBundle {
                mesh: shape,
                visibility: Visibility::INVISIBLE,
                material: debug_material.clone(),
                ..default()
            },
            HandPart,
        ));
    }
}

fn display_hands(
    hands_res: Res<HandsData>,
    mut bone_meshes_query: Query<(&mut Visibility, &mut Transform), With<HandPart>>,
) {
    let all_hands_bones = hands_res
        .hands
        .iter()
        .enumerate()
        .flat_map(|x2| x2.1.digits)
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

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.67, 0.84, 0.92).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 9000.0,
            range: 100.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(8.0, 16.0, 8.0),
        ..default()
    });
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-200.0, 500., 100.0).looking_at(Vec3::ZERO, Vec3::Y),

        ..default()
    });
}

fn uv_debug_texture() -> Image {
    const TEXTURE_SIZE: usize = 8;

    let mut palette: [u8; 32] = [
        255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255, 198, 255, 102, 198,
        255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
    ];

    let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
    for y in 0..TEXTURE_SIZE {
        let offset = TEXTURE_SIZE * y * 4;
        texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
        palette.rotate_right(4);
    }

    Image::new_fill(
        Extent3d {
            width: TEXTURE_SIZE as u32,
            height: TEXTURE_SIZE as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
    )
}
