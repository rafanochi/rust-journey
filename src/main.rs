use image::{GenericImage, GenericImageView, Rgba, open};

static SRC: &str = "./assets/reze.png";
static RESULT: &str = "./assets/reze.png";
static TEXT: &str = "Asuna";

fn main() {
    let text: Vec<_> = TEXT.into();

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
            image.put_pixel(x, y, Rgba([r, text[i], b, a]));
        });

    image.save("./assets/result.png").unwrap();

    // Read it back
    println!("======READING=====");
    let binding = open(RESULT).unwrap();
    let mut result = String::new();
    binding
        .clone()
        .pixels()
        .take(text.len())
        .for_each(|(_, _, Rgba([_r, g, _b, _a]))| {
            println!("{}", g.clone());
            result.push(g.into());
        });
    println!("Hidden text is: {}", result)
}
