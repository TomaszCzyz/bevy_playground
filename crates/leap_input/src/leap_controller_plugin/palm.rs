use bevy::math::{Vec3, Vec4};
use leaprs::Palm;

#[derive(Copy, Clone, Debug, Default)]
pub struct MyPalm {
    /// The center position of the palm in millimeters from the Ultraleap Tracking camera device origin.
    pub position: Vec3,

    /// The time-filtered and stabilized position of the palm.
    ///
    /// Smoothing and stabilization is performed in order to make
    /// this value more suitable for interaction with 2D content. The stabilized
    /// position lags behind the palm position by a variable amount, depending
    /// primarily on the speed of movement.
    pub stabilized_position: Vec3,

    /// The rate of change of the palm position in millimeters per second.
    pub velocity: Vec3,

    /// The normal vector to the palm. If your hand is flat, this vector wil.
    /// point downward, or \"out\" of the front surface of your palm.
    pub normal: Vec3,

    /// The estimated width of the palm when the hand is in a flat position.
    pub width: f32,

    /// The unit direction vector pointing from the palm position toward the fingers.
    /// pub direction: Vec3,
    /// The quaternion representing the palm's orientation
    /// corresponding to the basis {normal x direction, -normal, -direction}
    pub orientation: Vec4,
}

impl From<Palm<'_>> for MyPalm {
    fn from(leaprs_palm: Palm) -> Self {
        MyPalm {
            position: Vec3::from_array(leaprs_palm.position().array()),
            stabilized_position: Vec3::from_array(leaprs_palm.stabilized_position().array()),
            velocity: Vec3::from_array(leaprs_palm.velocity().array()),
            normal: Vec3::from_array(leaprs_palm.normal().array()),
            width: leaprs_palm.width(),
            orientation: Vec4::from_array(leaprs_palm.orientation().array()),
        }
    }
}
