// tilr - A program to build an image from a set of image 'tiles'.
// Copyright (C) 2023  Charles German <5donuts@pm.me>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, Pixel, Rgb};
use std::error::Error;
use std::ops::{Add, Div, Sub};
use std::path::Path;
use std::sync::Once;
use std::{fs, io};

// Directory constants
pub const TILE_DIR: &'static str = "images/tiles";
pub const INPUT_DIR: &'static str = "images/input";
pub const OUTPUT_DIR: &'static str = "images/output";

// Tile constants
const TILE_WIDTH: u32 = 25;
const TILE_HEIGHT: u32 = 25;
const COLORS: [(u8, u8, u8); 12] = [
    (0, 0, 0),       // black
    (255, 255, 255), // white
    (208, 35, 35),   // red
    (209, 136, 192), // pink
    (125, 36, 209),  // purple
    (52, 36, 209),   // blue
    (36, 167, 209),  // lighter blue
    (36, 209, 136),  // blue-ish green-ish
    (42, 209, 36),   // green
    (159, 209, 36),  // green-ish yellow-ish
    (209, 207, 36),  // yellow
    (209, 108, 36),  // orange
];

// Mosaic constants
const PURPLE: (u8, u8, u8) = (125, 36, 209);
const YELLOW: (u8, u8, u8) = (209, 207, 36);
const WIDTH: u32 = 250;
const HEIGHT: u32 = 250;
const SCALE_FACTOR: f32 = 0.25;
const TILE_SCALE_SIZE: u8 = 8; // scale the tiles to be 8px x 8px images

// Some trickery to only generate the directories/tiles once
static SETUP: Once = Once::new();

// Only call setup_inner() once
fn setup() {
    let mut res = Ok(());
    SETUP.call_once(|| {
        res = setup_inner();
    });
    res.unwrap();
}

fn setup_inner() -> Result<(), Box<dyn Error>> {
    make_dirs()?;
    make_tiles()?;
    Ok(())
}

/// Create the various testing directories
fn make_dirs() -> io::Result<()> {
    let paths = vec![
        Path::new(TILE_DIR),
        Path::new(INPUT_DIR),
        Path::new(OUTPUT_DIR),
    ];

    for p in paths {
        fs::create_dir_all(p)?;
    }

    Ok(())
}

/// Create the solid-color tile images
fn make_tiles() -> Result<(), Box<dyn Error>> {
    for (i, c) in COLORS.iter().enumerate() {
        let mut img = DynamicImage::new_rgb8(TILE_WIDTH, TILE_HEIGHT);
        for x in 0..TILE_WIDTH {
            for y in 0..TILE_HEIGHT {
                let px = Rgb([c.0, c.1, c.2]);
                img.put_pixel(x, y, px.to_rgba());
            }
        }

        let img = img.to_rgb8();
        img.save(format!("{}/tile-{}.png", TILE_DIR, i))?;
    }

    Ok(())
}

/// The core logic of these tests
///
/// # Arguments
/// * `extension` - the file extension (e.g., `png` for a `.png` file) to test encoding & decoding
///
/// # Returns
/// `Ok(())` if no errors were encountered while creating the mosaic (you may want to check
/// `images/output/` for a sanity check).
///
/// `Err(_)` if any error was encountered while creating the mosaic. Any error encountered will
/// cause the test calling this function to fail if it has `-> Result<(), Box<dyn Error>>`.
pub fn make_mosaic(extension: &str) -> Result<(), Box<dyn Error>> {
    // init all the directories & tiles
    setup();
    // create the src image
    let img_path = format!("{}/gradient.{}", INPUT_DIR, extension);
    let img = gradient(&PURPLE, &YELLOW, WIDTH, HEIGHT);
    img.save(&img_path)?;

    // create the mosaic
    let img = ImageReader::open(&img_path)?.decode()?.into_rgb8();
    let tiles = tilr::load_tiles(Path::new(TILE_DIR))?;
    let mosaic = tilr::Mosaic::new(
        DynamicImage::ImageRgb8(img),
        &tiles,
        SCALE_FACTOR,
        TILE_SCALE_SIZE,
    );
    let mosaic = mosaic.to_image();
    Ok(mosaic.save(format!("{}/mosaic.{}", OUTPUT_DIR, extension))?)
}

/// Generate a gradient from one color to another
///
/// # Arguments
/// * `c1` - the start color of the gradient (i.e., top left color)
/// * `c2` - the stop color of the gradient (i.e., bottom right color)
/// * `w` - the width of the resulting image
/// * `h` - the height of the resulting image
fn gradient(c1: &(u8, u8, u8), c2: &(u8, u8, u8), w: u32, h: u32) -> DynamicImage {
    let mut img = DynamicImage::new_rgb8(w, h);
    // let start_px = Rgb([c1.0, c1.1, c1.2]);
    // let end_px = Rgb([c2.0, c2.1, c2.2]);

    // Convert u8 to f32 for easier subtraction & division, plus use the convenience type
    let c1 = Color((c1.0 as f32, c1.1 as f32, c1.2 as f32));
    let c2 = Color((c2.0 as f32, c2.1 as f32, c2.2 as f32));

    // Change in the x- or y-directions per pixel
    let d_x = &(&c2 - &c1) / w as f32;
    let d_y = &(&c2 - &c1) / h as f32;

    // set the color of the start & end px
    // img.put_pixel(0, 0, start_px.to_rgba());
    // img.put_pixel(w - 1, h - 1, end_px.to_rgba());

    let mut c_x = c1;
    for x in 0..w {
        let mut c_y = c1;
        for y in 0..h {
            let color = &(&c_x + &c_y) / 2.0;
            let px = Rgb(color.to_u8_array());
            img.put_pixel(x, y, px.to_rgba());

            c_y = &c_y + &d_y;
        }
        c_x = &c_x + &d_x;
    }

    img
}

/// A wrapper type so we can use some convenient arithmetic operators when
/// doing the math to generate a gradient.
///
/// Out of the box, Rust does not allow you to use arithmetic operators on
/// tuples because they are heterogeneous.
/// See: https://doc.rust-lang.org/reference/types/tuple.html
///
/// To add & subtract colors,
/// ```
/// let c1 = Color((1, 2, 3));
/// let c2 = Color((4, 5, 6));
/// assert_eq!(c1 + c2, Color((5, 7, 9)));
/// assert_eq!(c2 - c1, Color((3, 3, 3)));
/// ```
/// To divide by a scalar,
/// ```
/// let c = Color((2, 4, 6));
/// let d = 2.0;
/// assert_eq!(c / d, Color((1, 2, 3)));
#[derive(Debug, PartialEq, Clone, Copy)]
struct Color<T>((T, T, T));

impl<T> Color<T>
where
    T: Into<f32> + Copy,
{
    fn to_u8_array(&self) -> [u8; 3] {
        let r: f32 = (self.0).0.into();
        let g: f32 = (self.0).1.into();
        let b: f32 = (self.0).2.into();

        [r.floor() as u8, g.floor() as u8, b.floor() as u8]
    }
}

impl<T> Add<&Color<T>> for &Color<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Color<T>;

    fn add(self, other: &Color<T>) -> Self::Output {
        Color((
            (self.0).0 + (other.0).0,
            (self.0).1 + (other.0).1,
            (self.0).2 + (other.0).2,
        ))
    }
}

impl<T> Sub<&Color<T>> for &Color<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Color<T>;

    fn sub(self, other: &Color<T>) -> Self::Output {
        Color((
            (self.0).0 - (other.0).0,
            (self.0).1 - (other.0).1,
            (self.0).2 - (other.0).2,
        ))
    }
}

impl<T> Div<f32> for &Color<T>
where
    T: Div<Output = f32> + Div<f32> + Copy,
    <T as Div<f32>>::Output: Into<f32>,
{
    type Output = Color<f32>;

    fn div(self, other: f32) -> Self::Output {
        Color((
            ((self.0).1 / other).into(),
            ((self.0).1 / other).into(),
            ((self.0).2 / other).into(),
        ))
    }
}
