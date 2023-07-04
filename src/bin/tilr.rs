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

use clap::Parser;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

use tilr::Mosaic;

// Struct to describe our command-line arguments
// and generate a parser for them.
#[derive(Debug, Parser)]
#[clap(
    author,
    version,
    about,
    long_about = r#"A program to build a mosaic of an image from a set of smaller image 'tiles'

Copyright (C) 2022 Charles German <5donuts@pm.me>
This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY.
See the GNU General Public License for more details. You should have received a copy of the
GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>."#
)]
struct Args {
    /// Path to the original image.
    #[clap(value_parser)]
    src_image: PathBuf,

    /// Path to the directory containing the tile set. Each image in this
    /// directory should be squares of the same size for optimal results.
    #[clap(short, long, default_value = "tiles/", value_parser)]
    tile_dir: PathBuf,

    /// Path at which to save the resulting image.
    #[clap(short, long, default_value = "mosaic.png", value_parser)]
    output: PathBuf,

    /// Scaling to apply to the image before building the mosaic.
    #[clap(short, long, default_value = "1.0")]
    scale: f32,

    /// The side length to use for the tiles (in pixels). Any tiles which
    /// are not squares with this side length will be resized; this may
    /// introduce some distortion in the resulting mosaic.
    #[clap(long, default_value = "8")]
    tile_size: u8,
}

fn main() {
    // fetch the CLI args
    let args = Args::parse();
    let src_image = args.src_image;
    let tile_dir = args.tile_dir;
    let scale = args.scale;
    let tile_size = args.tile_size;
    let output = args.output;

    // load the image to build a mosaic from
    eprint!("Loading input image...");
    let img = ImageReader::open(&src_image).expect("Unable to read image file.");
    let img = img.decode().expect("Unable to decode image file.");
    let img = img.into_rgb8(); // why does `.as_rgb8()` return `None` here?
    eprintln!("done.");

    // load the images to use as tiles
    eprint!("Loading tiles...");
    let tiles = tilr::load_tiles(&tile_dir).expect("Error loading tiles");
    eprintln!("done.");

    // build the mosaic
    eprint!("Initializing mosaic canvas...");
    let mosaic = Mosaic::new(DynamicImage::ImageRgb8(img), &tiles, scale, tile_size);
    eprintln!("done.");

    // get user confirmation to proceed (so we don't start making hilariously huge images
    // w/o asking first).
    let (mos_x, mos_y) = mosaic.output_size();
    if user_confirm(&*format!(
        "Resulting mosaic will be a {}px x {}px image. Continue y/N? ",
        mos_x, mos_y
    )) {
        let mosaic = mosaic.to_image();
        eprint!("Saving image to {}...", &output.display());
        mosaic.save(output).expect("Error saving mosaic.");
        eprintln!("done.");
    }
}

/// Get user confirmation for the given prompt
fn user_confirm(prompt: &str) -> bool {
    print!("{}", prompt);
    let _ = stdout().flush();

    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    // we only care about the first character
    let s = s.to_lowercase().chars().next().unwrap();
    if s != 'y' && s != 'n' {
        eprintln!("Unrecognized input; expected 'y' or 'n'.");
    }

    s == 'y'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        use clap::IntoApp;
        Args::into_app().debug_assert()
    }
}
