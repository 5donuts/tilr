use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use std::collections::HashMap;

use super::Tile;

#[derive(Debug)]
pub struct TileSet {
    tiles: Vec<Tile>,
}

impl TileSet {
    /// Build a tile set using the given images as tiles.
    /// The images will be scaled to be squares with a
    /// side length equal to the smallest dimension among
    /// the given images.
    /// NB: Aspect ratio will _not_ be preserved when the
    /// images are resized. Images are scaled using a
    /// triangular linear sampling filter.
    pub fn new(tiles: &Vec<DynamicImage>) -> Self {
        Self::from(tiles)
    }

    /// Get the side length of the tiles (which are square)
    /// in this set.
    pub fn tile_side_len(&self) -> u32 {
        self.tiles[0].side_len()
    }

    /// Create a mapping between pixels in the given image
    /// and tiles in the set
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

    /// Scale the tiles in this tileset to a new side length
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

    /// Given a pixel, find the tile in the set that most
    /// closely matches it
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
