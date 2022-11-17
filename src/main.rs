use std::ops::Deref;
use bevy::prelude::*;
use leaprs::{Connection, ConnectionConfig, Event};

use crate::hand_models::hand::MyHand;
use crate::hand_models::palm::Palm;

mod hand_models;

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

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
        .add_startup_system(spawn_camera)
        .add_system(update_hand_data)
        .add_system(print_hand_position.after(update_hand_data))
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

fn update_hand_data(
    mut hands_res: ResMut<HandsData>,
    mut leap_conn: NonSendMut<Connection>,
) {
    if let Ok(message) = leap_conn.poll(25) {
        match &message.event() {
            Event::Connection(_) => println!("connection event"),
            Event::Device(_) => println!("device event"),
            Event::Tracking(e) => {
                hands_res.hands.clear();

                if e.hands().len() == 0 {
                    println!("no hands...");
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

fn print_hand_position(hands_data: Res<HandsData>) {
    if hands_data.hands.len() == 0 {
        return;
    }

    let x: MyHand = hands_data.hands[0];

    println!("hands data - confidence: {}", x.digits[1].proximal.rotation)
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
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
