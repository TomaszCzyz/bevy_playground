use bevy::app::{App, Plugin};
use bevy::prelude::shape::Capsule;
use bevy::prelude::*;
use leaprs::{Connection, ConnectionConfig, Digit, Event};

use crate::display::display_hands;
use crate::leap_controller_plugin::hand::{MyHand, MyHandType};

mod bone;
mod digit;
mod hand;
mod palm;

pub struct LeapControllerPlugin;

impl Plugin for LeapControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_connection)
            .add_startup_system(spawn_hands_entities)
            .add_system(update_hand_data)
            .add_system(display_hands.after(update_hand_data))
            .insert_resource(HandsData { hands: default() });
    }
}

#[derive(Component)]
pub struct HandBone;

#[derive(Component)]
pub struct ThumbDigit;

#[derive(Component)]
pub struct IndexDigit;

#[derive(Component)]
pub struct MiddleDigit;

#[derive(Component)]
pub struct RingDigit;

#[derive(Component)]
pub struct PinkyDigit;

#[derive(Component)]
pub struct MetacarpalBone;

#[derive(Component)]
pub struct ProximalBone;

#[derive(Component)]
pub struct IntermediateBone;

#[derive(Component)]
pub struct DistalBone;

#[derive(Component)]
pub struct HandMarker;

pub enum DigitType {
    Unknown,
    Thumb,
    Index,
    Middle,
    Ring,
    Pinky,
}

pub enum BoneType {
    Unknown,
    Metacarpal,
    Proximal,
    Intermediate,
    Distal,
}

#[derive(Component)]
pub struct BoneComponent {
    pub digit_type: DigitType,
    pub bone_type: BoneType,
    pub _m: HandBone,
}

// impl BoneComponent {
//     fn new(digit_type: DigitType, bone_type: BoneType) -> Self {
//         Self {
//             digit_type,
//             bone_type,
//             _m: BoneMarker,
//         }
//     }
// }
//
// #[derive(Bundle)]
// pub struct DigitBundle {
//     pub metacarpal: BoneComponent,
//     pub proximal: BoneComponent,
//     pub intermediate: BoneComponent,
//     pub distal: BoneComponent,
//     pub _m: DigitMarker,
// }
//
// impl DigitBundle {
//     pub fn from_type(digit_type: DigitType) -> Self {
//         Self {
//             metacarpal: BoneComponent::default(),
//             proximal: BoneComponent::default(),
//             intermediate: BoneComponent::default(),
//             distal: BoneComponent::default(),
//             _m: DigitMarker,
//         }
//     }
// }
//
// #[derive(Bundle)]
// pub struct HandBundle {
//     pub type_: MyHandType,
//     pub thumb_digits: DigitBundle,
//     pub index_digits: DigitBundle,
//     pub middle_digits: DigitBundle,
//     pub ring_digits: DigitBundle,
//     pub pinky_digits: DigitBundle,
//     pub arm: BoneComponent,
//     pub _m: HandMarker,
// }
//
// impl HandBundle {
//     fn from_type(type_: MyHandType) -> Self {
//         Self {
//             type_,
//             thumb_digits: DigitBundle::from_type(DigitType::Thumb),
//             index_digits: DigitBundle::from_type(DigitType::Index),
//             middle_digits: DigitBundle::from_type(DigitType::Middle),
//             ring_digits: DigitBundle::from_type(DigitType::Ring),
//             pinky_digits: DigitBundle::from_type(DigitType::Pinky),
//             arm: BoneComponent::default(),
//             _m: HandMarker,
//         }
//     }
// }

#[derive(Resource)]
pub struct HandsData {
    pub hands: Vec<MyHand>,
}

/// Struct to mark SpatialBundle, which is a parent of all [`HandPart`]s.
/// You can use it for to change relative Transform of all digits at once.
#[derive(Component)]
pub struct HandsOrigin;

/// Struct to mark all hands' digits
#[derive(Component)]
pub struct HandPart;

fn create_connection(world: &mut World) {
    let mut connection = Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    world.insert_non_send_resource(connection);
}

fn spawn_hands_entities(mut commands: Commands) {
    commands
        .spawn((SpatialBundle::default(), HandsOrigin))
        .with_children(|parent| {
            for _ in 1..20 {
                parent.spawn(BoneComponent {
                    digit_type: DigitType::Unknown,
                    bone_type: BoneType::Unknown,
                    _m: HandBone,
                });
            }
        });
    // .with_children(|parent| {
    //     for _ in 1..2 {
    //         parent.spawn((HandBone, ThumbDigit, DistalBone));
    //         parent.spawn((HandBone, ThumbDigit, IntermediateBone));
    //         parent.spawn((HandBone, ThumbDigit, ProximalBone));
    //         parent.spawn((HandBone, ThumbDigit, MetacarpalBone));
    //
    //         parent.spawn((HandBone, IndexDigit, DistalBone));
    //         parent.spawn((HandBone, IndexDigit, IntermediateBone));
    //         parent.spawn((HandBone, IndexDigit, ProximalBone));
    //         parent.spawn((HandBone, IndexDigit, MetacarpalBone));
    //
    //         parent.spawn((HandBone, MiddleDigit, DistalBone));
    //         parent.spawn((HandBone, MiddleDigit, IntermediateBone));
    //         parent.spawn((HandBone, MiddleDigit, ProximalBone));
    //         parent.spawn((HandBone, MiddleDigit, MetacarpalBone));
    //
    //         parent.spawn((HandBone, RingDigit, DistalBone));
    //         parent.spawn((HandBone, RingDigit, IntermediateBone));
    //         parent.spawn((HandBone, RingDigit, ProximalBone));
    //         parent.spawn((HandBone, RingDigit, MetacarpalBone));
    //
    //         parent.spawn((HandBone, PinkyDigit, DistalBone));
    //         parent.spawn((HandBone, PinkyDigit, IntermediateBone));
    //         parent.spawn((HandBone, PinkyDigit, ProximalBone));
    //         parent.spawn((HandBone, PinkyDigit, MetacarpalBone));
    //     }
    // });
}

fn add_meshes_and_materials_for_hands(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut bones_query: Query<Entity, With<HandBone>>,
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

    for entity in bones_query.iter_mut() {
        commands.get_entity(entity).unwrap().insert(PbrBundle {
            mesh: meshes.add(capsule.clone().into()),
            visibility: Visibility::INVISIBLE,
            material: debug_material.clone(),
            ..default()
        });
    }
}

fn update_hand_data(
    mut leap_conn: NonSendMut<Connection>,
    mut digits_query: Query<(&mut Transform, &mut Visibility, &mut BoneComponent), With<HandBone>>,
) {
    if let Ok(message) = leap_conn.poll(25) {
        match &message.event() {
            Event::Connection(_) => println!("connection event"),
            Event::Device(_) => println!("device event"),
            Event::Tracking(e) => {
                if e.hands().len() == 0 {
                    for (_transform, mut visibility) in digits_query.iter_mut() {
                        visibility.is_visible = false;
                    }
                    return;
                }

                // for (transform, visibility, bone_type, digit_type) in digits_query.iter_mut() {
                //
                // }

                let mut query_iter = digits_query.iter_mut();

                let hands = e.hands();

                for hand in hands.iter() {
                    let digits = hand.digits();
                    let digits_types_order = [
                        DigitType::Thumb,
                        DigitType::Index,
                        DigitType::Middle,
                        DigitType::Ring,
                        DigitType::Pinky,
                    ];

                    for (bone_type_index, digit) in digits.iter().enumerate() {
                        let bones = [
                            (digit.distal(), BoneType::Distal),
                            (digit.proximal(), BoneType::Proximal),
                            (digit.intermediate(), BoneType::Intermediate),
                            (digit.metacarpal(), BoneType::Metacarpal),
                        ];

                        for (bone, bone_type) in bones {
                            let (&mut transform, visibility, bone_component) = query_iter.next().unwrap();

                            bone_component.digit_type = digits_types_order[&bone_type_index];
                            bone_component.bone_type = bone_type;
                            transform = Transform {
                                translation: Vec3::from_array(bone.prev_joint().array()),
                                rotation: Quat::from_array(bone.rotation().array()),
                                ..default()
                            };
                            visibility.is_visible = true;
                        }
                    }
                }

                let iterator = hands.iter().flat_map(|hand| hand.digits()).collect::<Vec<Digit>>();

                // for hand in e.hands() {
                //     hands_res.hands.push(MyHand::from(hand))
                // }
            }
            _ => {}
        }
    }
}

fn handle_distal_bone() {}

// fn update_hand_data(mut hands_res: ResMut<HandsData>, mut leap_conn: NonSendMut<Connection>) {
//     if let Ok(message) = leap_conn.poll(25) {
//         match &message.event() {
//             Event::Connection(_) => println!("connection event"),
//             Event::Device(_) => println!("device event"),
//             Event::Tracking(e) => {
//                 hands_res.hands.clear();
//
//                 if e.hands().len() == 0 {
//                     return;
//                 }
//
//                 for hand in e.hands() {
//                     hands_res.hands.push(MyHand::from(hand))
//                 }
//             }
//             _ => {}
//         }
//     }
// }
