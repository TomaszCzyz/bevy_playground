use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy::prelude::shape::Capsule;
use leaprs::{Connection, ConnectionConfig, Event};
use crate::display::display_hands;

use crate::leap_controller_plugin::hand::MyHand;

mod bone;
mod digit;
mod hand;
mod palm;

pub struct LeapControllerPlugin;

impl Plugin for LeapControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(create_connection)
            .add_startup_system(setup_for_hands)
            .add_system(update_hand_data)
            .add_system(display_hands.after(update_hand_data))
            .insert_resource(HandsData { hands: default() });
    }
}

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
            for shape in shapes.into_iter() {
                parent.spawn((
                    PbrBundle {
                        mesh: shape,
                        visibility: Visibility::INVISIBLE,
                        material: debug_material.clone(),
                        ..default()
                    },
                    HandPart,
                ));
            }
        });
}
