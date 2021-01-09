use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

// Struct to describe our command-line arguments
// and generate a parser for them.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "Tilr",
    author = "Charles German <5donuts@protonmail.com>",
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Opt {
    /// Enable verbose logging
    #[structopt(short, long)]
    verbose: bool,

    /// Path to the original image
    #[structopt(name = "IMAGE", parse(from_os_str))]
    pub image: PathBuf,

    /// Path to the directory containing the tile set. Each image in this
    /// directory should be squares of the same size for optimal results.
    /// Otherwise, all the images will be scaled to squares with side length
    /// equal to the shortest side length of any image in the directory.
    #[structopt(short, long, default_value = "tiles/", parse(from_os_str))]
    tile_dir: PathBuf,

    /// Path to the output image
    #[structopt(short, long, default_value = "tiled.png", parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    // TODO: setup logging (& verbose modes)
    // TODO: improve error messages (esp for I/O stuff)

    // parse command-line args
    let opt = Opt::from_args();

    let tileset = tilr::load_tiles(&opt.tile_dir);

    let img = ImageReader::open(opt.image)
        .expect("Unable to read image file.")
        .decode()
        .expect("Unable to decode image file.");

    tilr::stitch_mosaic(
        &img.as_rgb8().expect("Error reading image as an RGB image."),
        &tileset,
    );
}
