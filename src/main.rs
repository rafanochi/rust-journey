//
// Copyright (c) 2024 Nathan Fiedler
//
use magick_rust::{MagickError, MagickWand, magick_wand_genesis};
use rust_journey::ThumSize;
use rust_journey::util::thumbnail;
use std::fs::{self, File};
use std::sync::Once;
use std::time::SystemTime;

// Used to make sure MagickWand is initialized exactly once. Note that we do not
// bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

// Read the named file and create a thumbnail bound by a rectangle that is 240
// by 240 pixels (for snow-covered-cat.jpg it will be 240x191 pixels).

fn main() {
    match thumbnail(&START, "test/reze.jxl", ThumSize::Normal) {
        Ok(bytes) => {
            fs::write("test/thumbnail_result.png", bytes).expect("write failed");
        }
        Err(err) => println!("error: {err}"),
    }
}
