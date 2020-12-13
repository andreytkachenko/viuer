#![deny(missing_docs)]

//! Library to display images in the terminal.
//!
//! This library contains functionality extracted from the [`viu`](https://github.com/atanunq/viu) crate.
//! It aims to provide an easy to use interface to print images in the terminal. Uses some abstractions
//! provided by the [`image`] crate. Both the [Kitty](https://sw.kovidgoyal.net/kitty/graphics-protocol.html)
//! and [iTerm](https://iterm2.com/documentation-images.html) graphic protocols are supported.
//! By default, they are used if detected. If not, `viuer` will fallback to using regular
//! half blocks instead (▄ and ▀).
//!
//! ## Basic Usage
//! The example below shows how to print the image `img.jpg` in 40x30 terminal cells, with vertical
//! offset of 4 and horizontal of 10, starting from the top left corner. More options are available
//! through the [Config] struct.
//! ```no_run
//! use viuer::{Config, print_from_file};
//! let conf = Config {
//!     width: Some(40),
//!     height: Some(30),
//!     x: 10,
//!     y: 4,
//!     ..Default::default()
//! };
//! // will resize the image to fit in 40x30 terminal cells and print it
//! print_from_file("img.jpg", &conf).expect("Image printing failed.");
//! ```

use crossterm::execute;
use error::ViuResult;
use image::DynamicImage;
use printer::Printer;
use std::io::Write;

mod config;
mod error;
mod printer;
mod utils;

pub use config::Config;
pub use error::ViuError;
pub use printer::{get_kitty_support, is_iterm_supported, resize, KittySupport};
pub use utils::terminal_size;

/// Default printing method. Uses either iTerm or Kitty graphics protocol, if supported,
/// and half blocks otherwise.
///
/// Check the [Config] struct for all customization options.
/// ## Example
/// The snippet below reads all of stdin, decodes it with the [`image`] crate
/// and prints it to the terminal. The image will also be resized to fit in the terminal.
///
/// ```no_run
/// use std::io::{stdin, Read};
/// use viuer::{Config, print};
///
/// let stdin = stdin();
/// let mut handle = stdin.lock();
///
/// let mut buf: Vec<u8> = Vec::new();
/// let _ = handle
///     .read_to_end(&mut buf)
///     .expect("Could not read until EOF.");
///
/// let img = image::load_from_memory(&buf).expect("Data from stdin could not be decoded.");
/// print(&img, &Config::default()).expect("Image printing failed.");
/// ```
pub fn print(img: &DynamicImage, config: &Config) -> ViuResult<(u32, u32)> {
    let mut stdout = std::io::stdout();
    if config.restore_cursor {
        execute!(&mut stdout, crossterm::cursor::SavePosition)?;
    }

    let printer = choose_printer(config);

    let (w, h) = printer.print(img, config)?;

    if config.restore_cursor {
        execute!(&mut stdout, crossterm::cursor::RestorePosition)?;
    };

    Ok((w, h))
}

/// Helper method that reads a file, tries to decode it and prints it.
///
/// ## Example
/// ```no_run
/// use viuer::{Config, print_from_file};
/// let conf = Config {
///     width: Some(30),
///     transparent: true,
///     ..Default::default()
/// };
/// // Image will be scaled down to width 30. Aspect ratio will be preserved.
/// // Also, the terminal's background color will be used instead of checkerboard pattern.
/// print_from_file("img.jpg", &conf).expect("Image printing failed.");
/// ```
pub fn print_from_file(filename: &str, config: &Config) -> ViuResult<(u32, u32)> {
    let mut stdout = std::io::stdout();
    if config.restore_cursor {
        execute!(&mut stdout, crossterm::cursor::SavePosition)?;
    }

    let printer = choose_printer(config);

    let (w, h) = printer.print_from_file(filename, config)?;

    if config.restore_cursor {
        execute!(&mut stdout, crossterm::cursor::RestorePosition)?;
    };

    Ok((w, h))
}

// Choose the appropriate printer to use based on user config and availability
fn choose_printer(config: &Config) -> Box<dyn Printer> {
    if config.use_iterm && is_iterm_supported() {
        Box::new(printer::iTermPrinter {})
    } else if config.use_kitty && get_kitty_support() != KittySupport::None {
        Box::new(printer::KittyPrinter {})
    } else {
        Box::new(printer::BlockPrinter {})
    }
}
