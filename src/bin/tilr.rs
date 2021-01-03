use std::path::PathBuf;
use structopt::StructOpt;

/// Struct to describe our command-line arguments
/// and generate a parser for them.
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
    image: PathBuf,

    /// Path to the directory containing the tile set
    #[structopt(short, long, default_value = "tiles/", parse(from_os_str))]
    tile_dir: PathBuf,

    /// Path to the output image
    #[structopt(short, long, default_value = "tiled.png", parse(from_os_str))]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
}
