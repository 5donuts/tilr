#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    broken_intra_doc_links
)]

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, RgbImage};
use tiles::{Tile, TileSet};

mod tiles;

pub struct Mosaic {
    img: RgbImage,
    tiles: TileSet,
    inner: Inner,
}

impl Mosaic {
    pub fn new(
        img: DynamicImage,
        tiles: &Vec<DynamicImage>,
        img_scaling: f32,
        tile_size: u8,
    ) -> Self {
        // Scale the source image, if specified
        let img = if img_scaling != 1.0 {
            let (x, y) = img.dimensions();
            let x = (x as f32 * img_scaling) as u32;
            let y = (y as f32 * img_scaling) as u32;
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
        eprintln!(
            "Building mosaic with size ({}px, {}px) from tiles of side length {}px.",
            mos_x, mos_y, tile_size
        );

        Self { img, tiles, inner }
    }

    pub fn generate(self) -> RgbImage {
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

struct Inner(DynamicImage);

impl Inner {
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
