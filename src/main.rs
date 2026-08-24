use image::{GenericImage, GenericImageView, Rgba, open};

static SRC: &str = "./assets/reze.png";
// static RESULT: &str = "./assets/reze.png";
static TEXT: &str = "Asuna";

fn main() {
    let text: Vec<_> = TEXT.into();

    // Hide the text inside text
    let mut image = open(SRC).unwrap();
    image
        .clone()
        .pixels()
        .enumerate()
        .take(text.len())
        .for_each(|(i, (x, y, Rgba([r, g, b, a])))| {
            image.put_pixel(x, y, Rgba([r, g + text[i], b, a]));
        });

    image.save("./assets/result.png").unwrap();

    // Read it back
    // let mut binding = open(RESULT).unwrap();
    // let result: Vec<u8> = Vec::new();
    // binding
    //     .clone()
    //     .pixels()
    //     .enumerate()
    //     .take(text.len())
    //     .for_each(|(i, (x, y, Rgba([r, g, b, a])))| {
    //         binding.put_pixel(x, y, Rgba([r, g + text[i], b, a]));
    //     });
    // binding.save("./assets/result.png").unwrap();
}
