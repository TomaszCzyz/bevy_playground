use leaprs::Digit;

use crate::hand_models::bone::MyBone;

#[derive(Copy, Clone, Debug, Default)]
pub struct MyDigit {
    /// The finger bone wholly inside the hand.
    /// For thumbs, this bone is set to have zero length and width, an identity basis matrix,
    /// and its joint positions are equal.
    /// Note that this is anatomically incorrect; in anatomical terms, the intermediate phalange
    /// is absent in a real thumb, rather than the metacarpal bone. In the Ultraleap Tracking model,
    /// however, we use a \"zero\" metacarpal bone instead for ease of programming.
    pub metacarpal: MyBone,

    /// The phalange extending from the knuckle.
    pub proximal: MyBone,

    /// The bone between the proximal phalange and the distal phalange.
    pub intermediate: MyBone,

    /// The distal phalange terminating at the finger tip.
    pub distal: MyBone,

    is_extended: bool,
}

impl From<&Digit<'_>> for MyDigit {
    fn from(digit: &Digit) -> Self {
        MyDigit {
            metacarpal: MyBone::from(digit.metacarpal()),
            proximal: MyBone::from(digit.proximal()),
            intermediate: MyBone::from(digit.intermediate()),
            distal: MyBone::from(digit.distal()),
            is_extended: digit.is_extended(),
        }
    }
}
