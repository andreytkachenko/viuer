use crate::utils;

/// Configuration struct to customize printing behaviour.
pub struct Config {
    /// [resize](crate::resize) the image before printing. Defaults to true.
    pub resize: bool,
    /// Enable true transparency instead of checkerboard background. Defaults to false.
    pub transparent: bool,
    /// Make the x and y offset be relative to the top left terminal corner.
    /// If false, the y offset is relative to the row after the last printed image.
    /// Defaults to true.
    pub absolute_offset: bool,
    /// X offset. Defaults to 0.
    pub x: u16,
    /// Y offset. Can be negative only when `absolute_offset` is `false`. Defaults to 0.
    pub y: i16,
    /// Optional image width. Defaults to None.
    pub width: Option<u32>,
    /// Optional image height. Defaults to None.
    pub height: Option<u32>,
    /// Use truecolor if the terminal supports it. Defaults to true.
    pub truecolor: bool,
    /// Use Kitty protocol if the terminal supports it. Defaults to true.
    pub use_kitty: bool,
}

impl std::default::Default for Config {
    fn default() -> Self {
        Self {
            resize: true,
            transparent: false,
            absolute_offset: true,
            x: 0,
            y: 0,
            width: None,
            height: None,
            truecolor: utils::truecolor_available(),
            use_kitty: true,
        }
    }
}
