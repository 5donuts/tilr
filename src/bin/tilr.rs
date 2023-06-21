//! Build a mosaic of a particular image out of a set of smaller images.

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
use image::DynamicImage;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;
use structopt::StructOpt;

use tilr::Mosaic;

// Struct to describe our command-line arguments
// and generate a parser for them.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Tilr",
    about = r#"A program to build a mosaic of an image from a set of smaller image 'tiles'

Copyright (C) 2022 Charles German <5donuts@pm.me>
This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY.
See the GNU General Public License for more details. You should have received a copy of the
GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>."#
)]
struct Opt {
    /// Path to the original image.
    #[structopt(name = "SRC_IMAGE", parse(from_os_str))]
    pub image: PathBuf,

    /// Path to the directory containing the tile set. Each image in this
    /// directory should be squares of the same size for optimal results.
    #[structopt(short, long, default_value = "tiles/", parse(from_os_str))]
    tile_dir: PathBuf,

    /// Path at which to save the resulting image.
    #[structopt(short, long, default_value = "mosaic.png", parse(from_os_str))]
    output: PathBuf,

    /// Scaling to apply to the image before building the mosaic.
    #[structopt(short, long, default_value = "1.0")]
    scale: f32,

    /// The side length to use for the tiles (in pixels). Any tiles which
    /// are not squares with this side length will be resized; this may
    /// introduce some distortion in the resulting mosaic.
    #[structopt(long, default_value = "8")]
    tile_size: u8,
}

fn main() {
    // parse command-line args
    let opt = Opt::from_args();

    // load the image to build a mosaic from
    eprint!("Loading input image...");
    let img = ImageReader::open(opt.image).expect("Unable to read image file.");
    let img = img.decode().expect("Unable to decode image file.");
    let img = img.into_rgb8(); // why does `.as_rgb8()` return `None` here?
    eprintln!("done.");

    // load the images to use as tiles
    eprint!("Loading tiles...");
    let tiles = tilr::utils::load_tiles(&opt.tile_dir).expect("Error loading tiles");
    eprintln!("done.");

    // build the mosaic
    eprint!("Initializing mosaic canvas...");
    let mosaic = Mosaic::new(
        DynamicImage::ImageRgb8(img),
        &tiles,
        opt.scale,
        opt.tile_size,
    );
    eprintln!("done.");

    // get user confirmation to proceed (so we don't start making hilariously huge images
    // w/o asking first).
    let mut s = String::new();
    let (mos_x, mos_y) = mosaic.output_size();
    print!(
        "Resulting mosaic will be a {}px x {}px image. Continue y/n? ",
        mos_x, mos_y
    );
    let _ = stdout().flush();
    stdin().read_line(&mut s).unwrap();

    // ignore newlines
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }

    // we only care about the first character
    let s = s.chars().next().unwrap();
    if s == 'y' {
        let mosaic = mosaic.to_image();

        eprintln!(
            "Saving image to {}...",
            opt.output.clone().into_os_string().into_string().unwrap()
        );
        mosaic.save(opt.output).expect("Error saving mosaic.");
    } else if s != 'n' {
        // quit with warning
        println!("Unrecognized input, expected 'y' or 'n'.");
    }
    // else, the input was 'n' so quit
}
