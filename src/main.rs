use std::ops::BitAnd;

use image::{GenericImage, GenericImageView, Pixel, Rgba, open};

static SRC: &str = "./assets/reze.png";
static RESULT: &str = "./assets/reze.png";
static TEXT: &str = "Asuna";
static PATTERN: &str = "!@#$";

fn main() {
    let text: Vec<_> = format!("{PATTERN}{TEXT}{PATTERN}").into();
    let mut pixels_changed: Vec<(u32, u32)> = Vec::new();

    // Hide the text inside text
    println!("======HIDING=====");
    let mut image = open(SRC).unwrap();
    image
        .clone()
        .pixels()
        .enumerate()
        .take(text.len())
        .for_each(|(i, (x, y, Rgba([r, _g, b, a])))| {
            println!("{}", text[i].clone());
            pixels_changed.push((x, y));
            image.put_pixel(x, y, Rgba([r, text[i], b, a]));
        });

    image.save("./assets/result.png").unwrap();

    // Read it back
    println!("======READING=====");
    let pattern = PATTERN.as_bytes();
    let binding = open(RESULT).unwrap();
    let result = binding.as_bytes().chunks(text.len()).find_map(|x| {
        (x.starts_with(pattern) && x.ends_with(pattern))
            .then(|| x.strip_circumfix(pattern, pattern))
    });

    println!("Hidden text is: {:?}", result.is_some())
}
