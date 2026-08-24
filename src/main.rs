use image::{GenericImage, GenericImageView, Pixel, Rgba, open};

static SRC: &str = "./assets/reze.png";
static RESULT: &str = "./assets/reze.png";
static TEXT: &str = "Asuna";

fn main() {
    let text: Vec<_> = TEXT.into();
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
    let binding = open(RESULT).unwrap();
    let result: String = pixels_changed
        .iter()
        .map(|(x, y)| {
            let pixel = binding.get_pixel(*x, *y);
            let z = char::from(pixel[1]);
            println!("{:?}", pixel);
            z
        })
        .collect();

    println!("Hidden text is: {}", result)
}
