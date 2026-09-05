use image::{GenericImage, GenericImageView, Rgba, open};

static SRC: &str = "./assets/reze.png";
static RESULT: &str = "./assets/result.png";
static TEXT: &str = "Asuna";
static PATTERN: &str = "!@#$";

fn main() {
    let text: Vec<_> = format!("{PATTERN}{TEXT}{PATTERN}").into();

    // Hide the text inside text
    println!("======HIDING=====");
    let mut image = open(SRC).unwrap();
    image
        .clone()
        .pixels()
        .enumerate()
        .take(text.len())
        .for_each(|(i, (x, y, Rgba([r, _g, b, a])))| {
            image.put_pixel(x, y, Rgba([r, text[i], b, a]));
        });

    image.save("./assets/result.png").unwrap();

    // Read it back
    println!("======READING=====");
    let pattern = PATTERN.as_bytes();
    let binding = open(RESULT).unwrap();
    let pixels = binding
        .pixels()
        .map(|(_, _, Rgba([_r, g, _b, _a]))| g.clone())
        .collect::<Vec<u8>>();
    let result = pixels.chunks(text.len()).find_map(|x| {
        println!("the pixel: {:?}", String::from_utf8(x.to_vec()));
        x.strip_circumfix(pattern, pattern)
    });

    println!(
        "Hidden text is: {:?}",
        String::from_utf8_lossy(result.unwrap())
    )
}
