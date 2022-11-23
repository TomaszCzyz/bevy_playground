use bevy::math::Vec3;
use bevy::prelude::Quat;
use leaprs::Bone;

#[derive(Copy, Clone, Debug, Default)]
pub struct MyBone {
    /// The base of the bone, closer to the heart. The bones origin
    pub prev_joint: Vec3,

    /// The end of the bone, further from the heart.
    pub next_joint: Vec3,

    /// The average width of the flesh around the bone in millimeters.
    pub width: f32,

    /// Rotation in world space from the forward direction.
    /// Convert the quaternion to a matrix to derive the basis vectors.
    pub rotation: Quat,
}

impl From<Bone<'_>> for MyBone {
    fn from(bone: Bone) -> Self {
        MyBone {
            prev_joint: Vec3::from_array(bone.prev_joint().array()),
            next_joint: Vec3::from_array(bone.next_joint().array()),
            width: bone.width(),
            rotation: Quat::from_array(bone.rotation().array()),
        }
    }
}
