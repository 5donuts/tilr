//! Test Tilr reading/writing multiple popular image formats

mod utils;

use std::error::Error;
use utils::make_mosaic;

#[test]
fn png() -> Result<(), Box<dyn Error>> {
    make_mosaic("png")
}

#[test]
fn gif() -> Result<(), Box<dyn Error>> {
    make_mosaic("gif")
}

#[test]
fn tiff() -> Result<(), Box<dyn Error>> {
    make_mosaic("tiff")
}

#[test]
fn bmp() -> Result<(), Box<dyn Error>> {
    make_mosaic("bmp")
}

#[test]
fn svg() -> Result<(), Box<dyn Error>> {
    make_mosaic("svg")
}

#[test]
fn jpeg() -> Result<(), Box<dyn Error>> {
    make_mosaic("jpg")?;
    make_mosaic("jpeg")
}

#[test]
fn jpeg2000() -> Result<(), Box<dyn Error>> {
    make_mosaic("jp2")?;
    make_mosaic("jpx")
}

#[test]
fn jpeg_xl() -> Result<(), Box<dyn Error>> {
    make_mosaic("jxl")
}
