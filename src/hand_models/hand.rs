use std::ops::Deref;

use leaprs::{Digit, Hand, HandType};

use crate::hand_models::bone::MyBone;
use crate::hand_models::digit::MyDigit;
use crate::hand_models::palm::Palm;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyHandType {
    Left,
    Right,
}

impl Default for MyHandType {
    fn default() -> Self {
        MyHandType::Left
    }
}

impl From<HandType> for MyHandType {
    fn from(hand_type: HandType) -> Self {
        match hand_type {
            HandType::Left => MyHandType::Left,
            HandType::Right => MyHandType::Right,
        }
    }
}

#[derive(Copy, Clone, Default)]
pub struct MyHand {
    /// Identifies the chirality of this hand.
    pub type_: MyHandType,

    /// How confident we are with a given hand pose. Not currently used (always 1.0).
    pub confidence: f32,

    /// The total amount of time this hand has been tracked, in microseconds.
    pub visible_time: u64,

    /// The distance between index finger and thumb.
    pub pinch_distance: f32,

    /// The average angle of fingers to palm.
    pub grab_angle: f32,

    /// The normalized estimate of the pinch pose.
    /// Zero is not pinching; one is fully pinched.
    pub pinch_strength: f32,

    /// The normalized estimate of the grab hand pose.
    /// Zero is not grabbing; one is fully grabbing.
    pub grab_strength: f32,

    /// Additional information associated with the palm. @since 3.0.0
    pub palm: Palm,

    /// Fingers representations
    pub digits: [MyDigit; 5usize],

    /// The arm to which this hand is attached.
    /// An arm consists of a single LEAP_BONE struct.
    pub arm: MyBone,
}

impl From<Hand<'_>> for MyHand {
    fn from(hand: Hand) -> Self {
        let digits = hand.digits();
        let my_digits: [MyDigit; 5] =
            [
                MyDigit::from(&digits[0]),
                MyDigit::from(&digits[1]),
                MyDigit::from(&digits[2]),
                MyDigit::from(&digits[3]),
                MyDigit::from(&digits[4]),
            ];

        MyHand {
            type_: MyHandType::from(hand.hand_type()),
            confidence: hand.confidence(),
            visible_time: hand.visible_time(),
            pinch_distance: hand.pinch_distance(),
            grab_angle: hand.grab_angle(),
            pinch_strength: hand.pinch_strength(),
            grab_strength: hand.grab_strength(),
            palm: Palm::from(hand.palm()),
            digits: my_digits,
            arm: MyBone::from(hand.arm()),
        }
    }
}