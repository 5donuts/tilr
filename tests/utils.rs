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

use image::{DynamicImage, GenericImage, Pixel, Rgb};
use std::error::Error;
use std::path::Path;
use std::{fs, io};

// Directory constants
pub const TILE_DIR: &'static str = "images/tiles";
pub const INPUT_DIR: &'static str = "images/input";
pub const OUTPUT_DIR: &'static str = "images/output";

// Tile constants
const WIDTH: u32 = 25;
const HEIGHT: u32 = 25;
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
        let mut img = DynamicImage::new_rgb8(WIDTH, HEIGHT);
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let px = Rgb([c.0, c.1, c.2]);
                img.put_pixel(x, y, px.to_rgba());
            }
        }

        let img = img.to_rgb8();
        img.save(format!("{}/tile-{}.png", TILE_DIR, i))?;
    }

    Ok(())
}

pub fn setup() -> Result<(), Box<dyn Error>> {
    make_dirs()?;
    make_tiles()?;
    Ok(())
}

// pub static fn gradient()
