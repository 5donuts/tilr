#![warn(
    // missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    broken_intra_doc_links
)]

mod tileset;

pub use tileset::TileSet;

use image::{DynamicImage, GenericImage, Pixel, RgbImage};
use tileset::Tile;

/// Build a mosaic of an image using the given tile set to map
/// between pixels in the original image and a tile in the set.
/// NB: this may take a hot second to run.
// TODO: implement original image scaling/resulting mosaic scaling
// to reduce resulting image sizes
pub fn make_mosaic(img: &RgbImage, tiles: &TileSet) -> RgbImage {
    // get a mapping of image px to tiles
    let map = tiles.map_to(&img);

    // compute the size of the resulting mosaic
    let tile_sz = tiles.tile_side_len();
    let (img_x, img_y) = img.dimensions();
    let (mos_x, mos_y) = (img_x * tile_sz, img_y * tile_sz);
    eprintln!(
        "Building mosaic ({}, {}) with tiles of size ({}, {}).",
        mos_x, mos_y, tile_sz, tile_sz
    );

    // build the mosaic
    let mut mosaic = DynamicImage::new_rgb8(mos_x, mos_y);
    let mut mos_x = 0;
    for x in 0..img_x {
        let mut mos_y = 0;
        for y in 0..img_y {
            // add the tile to the mosaic
            eprintln!(
                "Adding tile for img px ({}, {}) to mosaic at ({}, {})...",
                x, y, mos_x, mos_y
            );
            let tile_for_px = map.get(img.get_pixel(x, y)).expect("No tile for px");
            add_tile_to_mosaic(&mut mosaic, tile_for_px, (mos_x, mos_y));
            mos_y += tile_sz;
        }
        mos_x += tile_sz;
    }

    // scale the image (TODO: remove, this is for testing)
    let mosaic = mosaic.resize_exact(1024, 1024, image::imageops::FilterType::Nearest);

    // convert to RGB image and return
    mosaic.into_rgb8()
}

fn add_tile_to_mosaic(mosaic: &mut DynamicImage, tile: &Tile, start_coords: (u32, u32)) {
    let s = tile.side_len();
    let (start_x, start_y) = start_coords;
    let mut tile_px = tile.img().pixels();
    for x in start_x..(start_x + s) {
        for y in start_y..(start_y + s) {
            let px = tile_px
                .next()
                .expect("Unable to get next tile px")
                .to_rgba();
            mosaic.put_pixel(x, y, px);
        }
    }
}
