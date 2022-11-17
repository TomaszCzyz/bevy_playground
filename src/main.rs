mod models;

use bevy::prelude::*;
use bevy::reflect::erased_serde::__private::serde::__private::ser::serialize_tagged_newtype;
use leaprs::{Connection, ConnectionConfig, Event, Hand, HandType};
use crate::models::{MyHand, Palm};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1280.0;

fn main() {
    App::new()
        // .init_resource::<MyConnection>()
        // .insert_resource(Connection::create(ConnectionConfig::default()).expect("Failed to create connection"))
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
        .insert_resource(HandsData { palm: default() })
        .run();
}

fn create_connection(world: &mut World) {
    let mut connection = Connection::create(ConnectionConfig::default()).expect("Failed to create connection");
    connection.open().expect("Failed to open the connection");

    world.insert_non_send_resource(connection);
}


// impl<'a> From<&Hand<'a>> for MyHand<'a> {
//     fn from(hand: &Hand) -> Self {
//         MyHand {
//             hand: Hand {
//
//         }
//         }
//     }
// }

#[derive(Resource)]
struct HandsData {
    // hands: Vec<&'a Hand<'a>>,
    palm: Palm,
    // hands: Vec<f32>,
}

fn update_hand_data(
    mut hands_res:  ResMut<HandsData>,
    mut leap_conn: NonSendMut<Connection>,
) {
    if let Ok(message) = leap_conn.poll(25) {
        match &message.event() {
            Event::Connection(_) => println!("connection event"),
            Event::Device(_) => println!("device event"),
            Event::Tracking(e) => {
                // hands_res.hands.clear();

                if e.hands().len() == 0 {
                    println!("no hands...");
                    return;
                }

                let vec = e.hands();
                let temp = vec.first().unwrap();
                let new_palm = temp.palm();
                // hands_res.hands.push(new_palm);
                hands_res.palm = Palm::from(new_palm);


                for hand in e.hands() {
                    let index_finger = hand.index();
                    let bones = index_finger.bones();
                    for bone in bones {
                        println!("bone width: {}", bone.width());
                    }
                    // let hand_s = &hand;

                    // let my_hand = MyHand::from(hand_s);
                    //
                    // hands_res.hands.push(my_hand);
                }
            }
            _ => {}
        }
    }
}

// #[derive(Resource, Default, Clone, Debug)]
// struct MyConnection {
//     connection: Arc<Mutex<Connection>>,
// }
//
// unsafe impl Send for MyConnection {}

fn print_hand_position() {
    // println!("test")
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
