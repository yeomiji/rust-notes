/// Adds one to the number given.
/// This is generated by running `cargo doc --bin ch14`.
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = ch14::add_one(arg);
///
/// assert_eq!(1, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

// re-export at the top layer for convenient api usage/documentation
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}