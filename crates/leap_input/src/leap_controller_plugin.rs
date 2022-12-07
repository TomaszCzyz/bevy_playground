use std::f32::consts::PI;

use bevy::app::{App, Plugin};
use bevy::prelude::shape::Capsule;
use bevy::prelude::*;
use leaprs::{Connection, ConnectionConfig, Event};

use crate::constant::LEAP_DIGITS_TYPES_ORDER;
use crate::leap_controller_plugin::hand::MyHand;

mod bone;
mod digit;
mod hand;
mod palm;

pub struct LeapControllerPlugin;

impl Plugin for LeapControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_connection)
            .add_startup_system(spawn_hands_entities)
            .add_system(update_hand_data);
    }
}

/// Struct to mark SpatialBundle, which is a parent of all [`BoneComponent`]s.
/// You can use it for to change relative Transform of all digits at once.
#[derive(Component)]
pub struct HandsOrigin;

#[derive(Component)]
pub struct BoneComponent {
    pub digit_type: DigitType,
    pub bone_type: BoneType,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DigitType {
    Unknown,
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoneType {
    Unknown,
    Metacarpal,
    Proximal,
    Intermediate,
    Distal,
}

#[derive(Resource)]
pub struct HandsData {
    pub hands: Vec<MyHand>,
}

fn create_connection(world: &mut World) {
    let mut connection = Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    world.insert_non_send_resource(connection);
}

fn spawn_hands_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let capsule = Capsule {
        radius: 5.,
        rings: 0,
        depth: 10.0,
        ..default()
    };

    let debug_material = materials.add(StandardMaterial {
        base_color: Color::rgb_u8(192, 191, 187),
        metallic: 0.3,
        perceptual_roughness: 0.8,
        reflectance: 0.2,
        ..default()
    });

    commands
        .spawn((SpatialBundle::default(), HandsOrigin))
        .with_children(|parent| {
            for _ in 0..40 {
                parent
                    .spawn(BoneComponent {
                        digit_type: DigitType::Unknown,
                        bone_type: BoneType::Unknown,
                    })
                    .insert(PbrBundle {
                        mesh: meshes.add(capsule.into()),
                        visibility: Visibility::INVISIBLE,
                        material: debug_material.clone(),
                        ..default()
                    });
            }
        });
}

fn update_hand_data(
    mut leap_conn: NonSendMut<Connection>,
    mut digits_query: Query<(&mut Transform, &mut Visibility, &mut BoneComponent)>,
) {
    if let Ok(message) = leap_conn.poll(25) {
        match &message.event() {
            Event::Connection(_) => println!("connection event"),
            Event::Device(_) => println!("device event"),
            Event::Tracking(e) => {
                let mut query_iter = digits_query.iter_mut();

                for hand in e.hands().iter() {
                    let digits = hand.digits();

                    for (bone_type_index, digit) in digits.iter().enumerate() {
                        let bones = [
                            (digit.distal(), BoneType::Distal),
                            (digit.proximal(), BoneType::Proximal),
                            (digit.intermediate(), BoneType::Intermediate),
                            (digit.metacarpal(), BoneType::Metacarpal),
                        ];

                        for (bone, bone_type) in bones {
                            let (mut transform, mut visibility, mut bone_component) = query_iter.next().unwrap();

                            bone_component.digit_type = LEAP_DIGITS_TYPES_ORDER[bone_type_index];
                            bone_component.bone_type = bone_type;
                            *transform = Transform {
                                translation: Vec3::from_array(bone.prev_joint().array()),
                                rotation: Quat::from_array(bone.rotation().array()) * Quat::from_rotation_x(PI / 2.),
                                ..default()
                            };
                            visibility.is_visible = true;
                        }
                    }
                }

                while let Some((_, mut visibility, _)) = query_iter.next() {
                    visibility.is_visible = false;
                }
            }
            _ => {}
        }
    }
}
