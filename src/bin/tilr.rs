use image::{io::Reader as ImageReader, DynamicImage};
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
    /// Path to the original image.
    #[structopt(name = "IMAGE", parse(from_os_str))]
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
    /// are not squares with this side length will be resized, which may
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
    let mosaic = tilr::Mosaic::new(
        DynamicImage::ImageRgb8(img),
        &tiles,
        opt.scale,
        opt.tile_size,
    )
    .generate();

    // save the image to the path specified
    eprintln!(
        "Saving image to {}...",
        opt.output.clone().into_os_string().into_string().unwrap()
    );
    mosaic.save(opt.output).expect("Error saving mosaic.");
}
