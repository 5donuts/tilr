use image::io::Reader as ImageReader;
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
    /// Path to the original image
    #[structopt(name = "IMAGE", parse(from_os_str))]
    pub image: PathBuf,

    /// Path to the directory containing the tile set. Each image in this
    /// directory should be squares of the same size for optimal results.
    /// Otherwise, all the images will be scaled to squares with side length
    /// equal to the shortest side length of any image in the directory.
    #[structopt(short, long, default_value = "tiles/", parse(from_os_str))]
    tile_dir: PathBuf,

    /// Path at which to save the resulting image
    #[structopt(short, long, default_value = "mosaic.png", parse(from_os_str))]
    save_path: PathBuf,
    // /// Enable verbose logging
    // #[structopt(short, long)]
    // verbose: bool,
}

fn main() {
    // parse command-line args
    let opt = Opt::from_args();

    // load the image to build a mosaic from
    let img = ImageReader::open(opt.image).expect("Unable to read image file.");
    let img = img.decode().expect("Unable to decode image file.");
    let img = img.into_rgb8(); // why does `.as_rgb8()` return `None` here?

    // load the images to use as tiles
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

    // build the tileset
    let tileset = tilr::TileSet::from(&tiles);

    // build the mosaic
    let mosaic = tilr::make_mosaic(&img, &tileset);

    // save the image to the path specified
    eprintln!(
        "Saving image to {}...",
        opt.save_path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap()
    );
    mosaic.save(opt.save_path).expect("Error saving mosaic.");
}
