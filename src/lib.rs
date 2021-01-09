#![warn(
    // missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    broken_intra_doc_links
)]

mod tileset;

use image::RgbImage;
use std::path::PathBuf;

pub use tileset::TileSet;

pub fn load_tiles(tile_dir: &PathBuf) -> TileSet<'_> {
    todo!("load all of the images in the dir");
    todo!("scale them to squares with the smallest side length of any image");
    todo!("build a TileSet from those images")
}

pub fn stitch_mosaic(img: &RgbImage, tiles: &TileSet<'_>) -> RgbImage {
    todo!()
}
