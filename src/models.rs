use bevy::utils::default;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MyHandType {
    Left,
    Right,
}

#[derive(Copy, Clone)]
pub struct MyHand {
    id: u32,

    /// Reserved for future use.
    flags: u32,

    /// Identifies the chirality of this hand.
    type_: MyHandType,

    /// How confident we are with a given hand pose. Not currently used (always 1.0).
    confidence: f32,

    /// The total amount of time this hand has been tracked, in microseconds.
    visible_time: u64,

    /// The distance between index finger and thumb.
    pinch_distance: f32,

    /// The average angle of fingers to palm.
    grab_angle: f32,

    /// The normalized estimate of the pinch pose.
    /// Zero is not pinching; one is fully pinched.
    pinch_strength: f32,

    /// The normalized estimate of the grab hand pose.
    /// Zero is not grabbing; one is fully grabbing.
    grab_strength: f32,

    /// Additional information associated with the palm. @since 3.0.0
    palm: Palm,

    // __bindgen_anon_1: _LEAP_HAND__bindgen_ty_1,

    /// The arm to which this hand is attached.
    /// An arm consists of a single LEAP_BONE struct.
    arm: Bone,
}

#[derive(Copy, Clone)]
struct Bone {
    /// The base of the bone, closer to the heart. The bones origin
    pub prev_joint: [f32; 3usize],

    /// The end of the bone, further from the heart.
    pub next_joint: [f32; 3usize],

    /// The average width of the flesh around the bone in millimeters.
    pub width: f32,

    /// Rotation in world space from the forward direction.
    /// Convert the quaternion to a matrix to derive the basis vectors.
    pub rotation: [f32; 4usize],
}

#[derive(Copy, Clone)]
pub struct Palm {
    /// The center position of the palm in millimeters from the Ultraleap Tracking camera device origin.
    pub position: [f32; 3usize],

    /// The time-filtered and stabilized position of the palm.
    ///
    /// Smoothing and stabilization is performed in order to make
    /// this value more suitable for interaction with 2D content. The stabilized
    /// position lags behind the palm position by a variable amount, depending
    /// primarily on the speed of movement.
    pub stabilized_position: [f32; 3usize],

    /// The rate of change of the palm position in millimeters per second.
    pub velocity: [f32; 3usize],

    /// The normal vector to the palm. If your hand is flat, this vector wil.
    /// point downward, or \"out\" of the front surface of your palm.
    pub normal: [f32; 3usize],

    /// The estimated width of the palm when the hand is in a flat position.
    pub width: f32,

    // /// The unit direction vector pointing from the palm position toward the fingers.
    // pub direction: [f32; 3usize],

    /// The quaternion representing the palm's orientation
    /// corresponding to the basis {normal x direction, -normal, -direction}
    pub orientation: [f32; 4usize],
}

impl Default for Palm {
    fn default() -> Self {
        Palm {
            position: default(),
            stabilized_position: default(),
            velocity: default(),
            normal: default(),
            width: 0.0,
            orientation: default(),
        }
    }
}

impl From<leaprs::Palm<'_>> for Palm {
    fn from(leaprs_palm: leaprs::Palm) -> Self {
        Palm {
            position: leaprs_palm.position().array(),
            stabilized_position: leaprs_palm.stabilized_position().array(),
            velocity: leaprs_palm.velocity().array(),
            normal: leaprs_palm.normal().array(),
            width: leaprs_palm.width(),
            orientation: leaprs_palm.orientation().array(),
        }
    }
}