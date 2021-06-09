//! Build a mosaic of a particular image out of a set of smaller images.

#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    broken_intra_doc_links
)]

use image::{DynamicImage, GenericImage, GenericImageView, Pixel, RgbImage};
use tiles::{Tile, TileSet};

mod tiles;

/// Generates an image 'mosaic' using a set of image Tiles.
///
/// An image 'mosaic' is an image made up of a number of smaller
/// images in place of pixels. Using the average color of an image
/// Tile, a suitable large image mosaic viewed from far enough
/// away can appear to be a normal image.
#[allow(missing_debug_implementations)]
pub struct Mosaic {
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
        eprintln!(
            "Building mosaic with size ({}px, {}px) from tiles of side length {}px.",
            mos_x, mos_y, tile_size
        );

        Self { img, tiles, inner }
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
