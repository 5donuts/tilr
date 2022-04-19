//! Build a mosaic of a particular image out of a set of smaller images.

// tilr - A program to build an image from a set of image 'tiles'.
// Copyright (C) 2022  Charles German <5donuts@pm.me>
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

#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    broken_intra_doc_links
)]

use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::{DynamicImage, GenericImage, GenericImageView, Pixel, Rgb, RgbImage};
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::{fs, path::PathBuf};
use structopt::StructOpt;

// Struct to describe our command-line arguments
// and generate a parser for them.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Tilr",
    author = "Charles German <5donuts@pm.me>",
    about = env!("CARGO_PKG_DESCRIPTION")
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
    let img = ImageReader::open(opt.image).expect("Unable to read image file.");
    let img = img.decode().expect("Unable to decode image file.");
    let img = img.into_rgb8(); // why does `.as_rgb8()` return `None` here?

    // load the images to use as tiles
    // TODO: replace these "unwrap"s
    let mut tiles = Vec::new();
    fs::read_dir(opt.tile_dir)
        .expect("Error opening tile dir.")
        .for_each(|entry| {
            if let Ok(e) = entry {
                let path = e.path();
                if !path.is_dir() {
                    if let Ok(img) = ImageReader::open(&path) {
                        if let Ok(img) = img.decode() {
                            tiles.push(img);
                        } else {
                            eprintln!(
                                "Unable to decode image {}",
                                path.into_os_string().into_string().unwrap()
                            );
                        }
                    } else {
                        eprintln!(
                            "Could not open file {}",
                            path.into_os_string().into_string().unwrap()
                        );
                    }
                } else {
                    eprintln!(
                        "Skipping directory entry {}",
                        path.into_os_string().into_string().unwrap()
                    );
                }
            } else {
                eprintln!("Error reading dir entry {}", entry.unwrap_err().to_string());
            }
        });

    // build the mosaic
    let mosaic = Mosaic::new(
        DynamicImage::ImageRgb8(img),
        &tiles,
        opt.scale,
        opt.tile_size,
    );

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

/// Generates an image 'mosaic' using a set of image Tiles.
///
/// An image 'mosaic' is an image made up of a number of smaller
/// images in place of pixels. Using the average color of an image
/// Tile, a suitable large image mosaic viewed from far enough
/// away can appear to be a normal image.
#[allow(missing_debug_implementations)]
struct Mosaic {
    /// The original image used to create the mosaic.
    img: RgbImage,
    /// The set of [`Tile`]s to use to build the mosaic.
    ///
    /// Pixels in the original image are mapped to these tiles based
    /// on the Euclidean distance between the RGB pixel values and the
    /// average RGB values in the [`Tile`].
    tiles: TileSet,
    /// An inner member used to build the resulting image mosaic.
    inner: Inner,
}

impl Mosaic {
    /// Initialize a new image mosaic.
    ///
    /// # Arguments
    /// * `img` - The original image used to create the mosaic.
    /// * `tiles` - The set of Tiles to use to build the mosaic.
    /// * `img_scaling` - The scaling factor to apply to the original
    ///                   image for the mosaic. A scaling factor of `1`
    ///                   means no scaling. The scaling performed does
    ///                   _not_ preserve aspect ratio.
    /// * `tile_size` - The desired side length for the Tiles to use
    ///                 to generate this mosaic. If the Tiles are not
    ///                 already squares with this side length, they will
    ///                 be resized (without preserving aspect ratio) to
    ///                 be squares with the given side length.
    ///
    /// # Returns
    /// An empty mosaic. To build the mosaic, call [`to_image`](Mosaic::to_image).
    /// Note that generating the resulting mosaic is an expensive operation and
    /// could take many seconds (or minutes for especially large mosaics).
    ///
    /// # Panics
    /// This function panics if `img_scaling` is less than `0.1`.
    /// Additionally, it will panic if the chosen scaling factor would result
    /// in an image that has zero pixels in any dimension.
    pub fn new(
        img: DynamicImage,
        tiles: &Vec<DynamicImage>,
        img_scaling: f32,
        tile_size: u8,
    ) -> Self {
        if img_scaling < 0.1 {
            panic!("Scaling factor must be at least 0.1.");
        }
        // Scale the source image, if specified
        let img = if img_scaling != 1.0 {
            let (x, y) = img.dimensions();
            let x = (x as f32 * img_scaling) as u32;
            let y = (y as f32 * img_scaling) as u32;
            if x == 0 || y == 0 {
                panic!(
                    "Scaling factor results in an image with at least one dimension with zero px"
                );
            }
            img.resize_exact(x, y, image::imageops::FilterType::Triangle)
        } else {
            img
        }
        .to_rgb8();

        // Build the tileset
        let mut tiles = TileSet::from(tiles);

        // Scale the tiles if they're not already appropriately
        // sized.
        // TODO: just build them the correct size to start with.
        let tile_size = tile_size as u32;
        if tiles.tile_side_len() != tile_size {
            tiles.scale_tiles(tile_size);
        }

        // Initialize the inner image (the output mosaic image)
        let (img_x, img_y) = img.dimensions();
        let (mos_x, mos_y) = (img_x * tile_size, img_y * tile_size);
        let inner = Inner(DynamicImage::new_rgb8(mos_x, mos_y));

        Self { img, tiles, inner }
    }

    /// Get the size (in pixels) of the resulting mosaic based on the input image size,
    /// scale factor, and tile size.
    pub fn output_size(&self) -> (u32, u32) {
        let (img_x, img_y) = self.img.dimensions();
        let tile_size = self.tiles.tile_side_len();
        let (mos_x, mos_y) = (img_x * tile_size, img_y * tile_size);

        (mos_x, mos_y)
    }

    /// Generate the image mosaic and convert it to an [`RgbImage`].
    ///
    /// Depending on the size of the mosaic to build, this function may
    /// take some time to run.
    pub fn to_image(self) -> RgbImage {
        let map = self.tiles.map_to(&self.img);
        let (img_x, img_y) = self.img.dimensions();
        let tile_size = self.tiles.tile_side_len();
        let mut mosaic = self.inner;

        // Build the mosaic
        let mut mos_x = 0;
        for x in 0..img_x {
            let mut mos_y = 0;
            for y in 0..img_y {
                eprintln!(
                    "Adding tile for img px ({}, {}) to mosaic at ({}, {})...",
                    x, y, mos_x, mos_y
                );

                // Add the tile to the mosaic
                let tile_for_px = map.get(&self.img.get_pixel(x, y)).expect("No tile for px");
                mosaic.add_tile(tile_for_px, (mos_x, mos_y));

                // Move to the next pixel in the mosaic
                mos_y += tile_size;
            }

            // Move to the next row in the mosaic
            mos_x += tile_size;
        }

        mosaic.0.into_rgb8()
    }
}

/// A wrapper around a [`DynamicImage`] used to build the resulting
/// image mosaic.
struct Inner(DynamicImage);

impl Inner {
    /// Add a [`Tile`] to the image mosaic.
    ///
    /// More specifically, insert the pixels of a given [`Tile`] into
    /// this image at an offset based on where that [`Tile`] belongs
    /// in the [`Mosaic`].
    pub fn add_tile(&mut self, tile: &Tile, start_coords: (u32, u32)) {
        let s = tile.side_len();
        let (start_x, start_y) = start_coords;
        let mut tile_px = tile.img().pixels();
        for x in start_x..(start_x + s) {
            for y in start_y..(start_y + s) {
                let px = tile_px
                    .next()
                    .expect("Unable to get next tile px")
                    .to_rgba();
                self.0.put_pixel(x, y, px);
            }
        }
    }
}

/// Represents a single tile in a set; used to map
/// between pixels in the original image and images
/// in the [`TileSet`](super::TileSet).
#[derive(Debug)]
struct Tile {
    /// The underlying image to use for this Tile.
    img: RgbImage,
    /// The average pixel in the underlying image.
    ///
    /// This is computed only once when the tile is
    /// first created to handle the case of very large
    /// images being used as tiles and making the mapping
    /// between image pixels and Tiles very slow.
    avg: Rgb<u8>,
}

impl Tile {
    /// Compute the Euclidean distance between the color
    /// of the given pixel and the average pixel color
    /// of this Tile.
    pub fn dist_to(&self, px: &Rgb<u8>) -> f32 {
        // color values for the given px
        let p_r = px.0[0] as i32;
        let p_g = px.0[1] as i32;
        let p_b = px.0[2] as i32;

        // color values for the avg px color of the tile
        let q_r = self.avg.0[0] as i32;
        let q_g = self.avg.0[1] as i32;
        let q_b = self.avg.0[2] as i32;

        // Euclidean distance
        (((p_r - q_r).pow(2) + (p_g - q_g).pow(2) + (p_b - q_b).pow(2)) as f32).sqrt()
    }

    /// Get the underlying image for this Tile.
    pub fn img(&self) -> &RgbImage {
        &self.img
    }

    /// Get the side length of this Tile.
    pub fn side_len(&self) -> u32 {
        self.img.dimensions().0
    }
}

impl From<RgbImage> for Tile {
    /// Build a [`Tile`] from an [`RgbImage`].
    fn from(img: RgbImage) -> Self {
        let avg_px_color = {
            // get total for each color in the image
            let mut tot_r = 0;
            let mut tot_g = 0;
            let mut tot_b = 0;
            for px in img.pixels() {
                tot_r += px.0[0] as usize;
                tot_g += px.0[1] as usize;
                tot_b += px.0[2] as usize;
            }

            // calculate the avg color for the image
            // TODO: to we care about integer division here?
            let num_px = img.pixels().len();
            Rgb([
                (tot_r / num_px) as u8,
                (tot_g / num_px) as u8,
                (tot_b / num_px) as u8,
            ])
        };

        Self {
            img,
            avg: avg_px_color,
        }
    }
}

/// A set of [`Tile`]s to use to build a [`Mosaic`](crate::Mosaic).
///
/// This struct provides methods to map between the pixels in the original
/// image to [`Tile`]s in order to build a [`Mosaic`](crate::Mosaic).
#[derive(Debug)]
struct TileSet {
    /// The [`Tile`]s in this set.
    tiles: Vec<Tile>,
}

impl TileSet {
    /// Get the side length of the [`Tile`]s (which are uniform squares)
    /// in this set.
    pub fn tile_side_len(&self) -> u32 {
        self.tiles[0].side_len()
    }

    /// Create a mapping between pixels in the given image
    /// and [`Tile`]s in the set.
    pub fn map_to<'a>(&self, img: &'a RgbImage) -> HashMap<&'a Rgb<u8>, &Tile> {
        let mut map = HashMap::new();
        for px in img.pixels() {
            if map.contains_key(px) {
                continue; // don't duplicate closest tile calculations
            }
            map.insert(px, self.closest_tile(px));
        }

        map
    }

    /// Scale the [`Tile`]s in this tileset to a new side length.
    pub fn scale_tiles(&mut self, s: u32) {
        self.tiles = self
            .tiles
            .iter()
            .map(|t| {
                let dyn_img = DynamicImage::ImageRgb8(t.img().clone());
                Tile::from(dyn_img.resize_exact(s, s, FilterType::Triangle).to_rgb8())
            })
            .collect();
    }

    /// Given a pixel, find the [`Tile`] in the set that most
    /// closely matches it.
    fn closest_tile(&self, px: &Rgb<u8>) -> &Tile {
        let mut min_idx = 0;
        for (i, t) in self.tiles.iter().enumerate() {
            if t.dist_to(px) < self.tiles[min_idx].dist_to(px) {
                min_idx = i;
            }
        }
        &self.tiles[min_idx]
    }
}

impl From<&Vec<DynamicImage>> for TileSet {
    /// Build a tile set using the given images as [`Tile`]s.
    ///
    /// The images will be scaled to be squares with a
    /// side length equal to the smallest dimension among
    /// the given images.
    ///
    /// NB: Aspect ratio will _not_ be preserved when the
    /// images are resized. Images are scaled using a
    /// triangular linear sampling filter.
    // TODO: look into reducing the memory footprint of this fn
    fn from(imgs: &Vec<DynamicImage>) -> Self {
        // get the smallest dimension of any of the images
        // for the side length of the resulting image tiles
        let s = imgs
            .iter()
            .map(|img| {
                let (w, h) = img.dimensions();
                if w < h {
                    w
                } else {
                    h
                }
            })
            .min()
            .unwrap();

        // scale all of the images to be squares with that side length
        let imgs: Vec<RgbImage> = imgs
            .iter()
            .map(|img| img.resize_exact(s, s, FilterType::Triangle).to_rgb8())
            .collect();

        // build tiles from the resulting images
        Self {
            tiles: imgs.iter().map(|img| Tile::from(img.clone())).collect(),
        }
    }
}
