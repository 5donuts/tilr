//! Test Tilr reading/writing multiple popular image formats

mod utils;

use std::error::Error;
use utils::make_mosaic;

#[test]
fn png() -> Result<(), Box<dyn Error>> {
    make_mosaic("png")
}

#[test]
fn jpeg() -> Result<(), Box<dyn Error>> {
    make_mosaic("jpg")?;
    make_mosaic("jpeg")
}

#[test]
fn svg() -> Result<(), Box<dyn Error>> {
    make_mosaic("svg")
}
