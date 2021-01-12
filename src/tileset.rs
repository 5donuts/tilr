use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, Rgb, RgbImage};
use std::collections::HashMap;

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
            // TODO: see about skipping pixels that already have a
            // closest tile computed for them
            map.insert(px, self.closest_tile(px));
        }

        map
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

/// Represents a single tile in a set; used to map
/// between pixels in the original image and images
/// in the TileSet.
#[derive(Debug)]
pub struct Tile {
    /// The underlying image to use for this Tile
    img: RgbImage,
    /// The average pixel in the underlying image
    avg: Rgb<u8>,
}

impl Tile {
    /// Compute the Euclidean distance between the color
    /// of the given pixel and the average pixel color
    /// of this Tile.
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

    /// Get the underlying image for this Tile
    pub fn img(&self) -> &RgbImage {
        &self.img
    }

    /// Get the side length of this Tile
    pub fn side_len(&self) -> u32 {
        self.img.dimensions().0
    }
}

impl From<RgbImage> for Tile {
    fn from(img: RgbImage) -> Self {
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
