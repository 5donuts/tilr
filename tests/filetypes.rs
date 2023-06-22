//! Test Tilr reading/writing multiple popular image formats

use std::sync::Once;

mod utils;

static SETUP: Once = Once::new();

// This function will only call `utils::setup()` once.
fn setup() {
    let mut res = Ok(());
    SETUP.call_once(|| {
        res = utils::setup();
    });
    res.unwrap();
}

#[test]
fn png() {
    setup();
    todo!()
}

#[test]
fn jpeg() {
    setup();
    todo!()
}

#[test]
fn svg() {
    setup();
    todo!()
}
