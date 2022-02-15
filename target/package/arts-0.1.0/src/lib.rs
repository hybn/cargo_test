//! # Art
//!
//! 一个用户艺术颜色的库

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondColor;
pub use self::utils::mix;

pub mod kinds {

    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    pub enum SecondColor {
        Orange,
        Green,
        Purple
    }
}

pub mod utils {
    use crate::kinds::{PrimaryColor, SecondColor};

    pub fn mix(one: PrimaryColor, two: PrimaryColor) -> SecondColor {
        SecondColor::Orange
    }
}