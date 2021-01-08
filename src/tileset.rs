use image::{Rgb, RgbImage};
use std::collections::HashMap;

#[derive(Debug)]
pub struct TileSet<'a> {
    tiles: Vec<Tile<'a>>,
}

impl<'a> TileSet<'a> {
    /// Build a tile set using the given images as tiles
    pub fn new(tiles: &'a Vec<RgbImage>) -> Self {
        Self {
            tiles: tiles.iter().map(|img| Tile::from(img)).collect(),
        }
    }

    /// Create a mapping between pixels in the given image
    /// and tiles in the set
    pub fn map_to(&self, img: &'a RgbImage) -> HashMap<&Rgb<u8>, &Tile<'a>> {
        let mut map = HashMap::new();
        for px in img.pixels() {
            // TODO: see about skipping pixels that already have a
            // closest tile computed for them
            map.insert(px, self.closest_tile(px));
        }

        map
    }

    /// Given a pixel, find the tile in the set that most
    /// closely matches it
    fn closest_tile(&self, px: &Rgb<u8>) -> &Tile<'a> {
        let mut min_idx = 0;
        for (i, t) in self.tiles.iter().enumerate() {
            if t.dist_to(px) < self.tiles[min_idx].dist_to(px) {
                min_idx = i;
            }
        }
        &self.tiles[min_idx]
    }
}

/// Represents a single tile in a set; used to map
/// between pixels in the original image and images
/// in the tile set.
#[derive(Debug)]
pub struct Tile<'a> {
    /// The underlying image to use for this tile
    img: &'a RgbImage,
    /// The average pixel in the underlying image
    avg: Rgb<u8>,
}

impl<'a> Tile<'a> {
    /// Compute the Euclidean distance between the color
    /// of the given pixel and the average pixel color
    /// of this tile.
    pub fn dist_to(&self, px: &Rgb<u8>) -> f32 {
        // color values for the given px
        let p_r = px.0[0];
        let p_g = px.0[1];
        let p_b = px.0[2];

        // color values for the avg px color of the tile
        let q_r = self.avg.0[0];
        let q_g = self.avg.0[1];
        let q_b = self.avg.0[2];

        // Euclidean distance
        (((p_r - q_r).pow(2) + (p_g - q_g).pow(2) + (p_b - q_b).pow(2)) as f32).sqrt()
    }
}

impl<'a> From<&'a RgbImage> for Tile<'a> {
    fn from(img: &'a RgbImage) -> Self {
        let avg_px_color = {
            // get total for each color in the image
            let mut tot_r = 0;
            let mut tot_g = 0;
            let mut tot_b = 0;
            for px in img.pixels() {
                tot_r += px.0[0];
                tot_g += px.0[1];
                tot_b += px.0[2];
            }

            // calculate the avg color for the image
            // TODO: to we care about integer division here?
            let num_px = img.pixels().len();
            Rgb([
                tot_r / num_px as u8,
                tot_g / num_px as u8,
                tot_b / num_px as u8,
            ])
        };

        Self {
            img,
            avg: avg_px_color,
        }
    }
}
