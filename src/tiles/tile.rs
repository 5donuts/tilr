use image::{Rgb, RgbImage};

/// Represents a single tile in a set; used to map
/// between pixels in the original image and images
/// in the [`TileSet`](super::TileSet).
#[derive(Debug)]
pub struct Tile {
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
