//
// Copyright (c) 2024 Nathan Fiedler
//
use magick_rust::{MagickError, MagickWand, magick_wand_genesis};
use rust_journey::ThumSize;
use std::fs;
use std::sync::Once;

// Used to make sure MagickWand is initialized exactly once. Note that we do not
// bother shutting down, we simply exit when we're done.
static START: Once = Once::new();

// Read the named file and create a thumbnail bound by a rectangle that is 240
// by 240 pixels (for snow-covered-cat.jpg it will be 240x191 pixels).
fn thumbnail(filepath: &str, size: ThumSize) -> Result<Vec<u8>, MagickError> {
    START.call_once(|| {
        magick_wand_genesis();
    });
    let wand = MagickWand::new();
    wand.read_image(filepath)?;
    wand.fit(size.into(), size.into());
    wand.set_image_property("Thumb::URI", filepath)?;
    // wand.set_image_property("Thumb::MTime", wand.get_image_properties(pattern))?;
    wand.set_image_property("Description", "alksjf;laksjdf;alksjdf;laksjdf;alkjsef")?;

    wand.write_image_blob("png")
}

fn main() {
    match thumbnail("test/reze.jxl", ThumSize::Normal) {
        Ok(bytes) => {
            fs::write("dog.png", bytes).expect("write failed");
        }
        Err(err) => println!("error: {err}"),
    }
}
