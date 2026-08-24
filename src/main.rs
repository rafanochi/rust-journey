use image::{GenericImage, GenericImageView, Rgba, open};

fn main() {
    let mut binding = open("./assets/reze.png").unwrap();

    let text: Vec<_> = "Asuna".into();
    binding
        .clone()
        .pixels()
        .enumerate()
        .take(text.len())
        .for_each(|(i, (x, y, Rgba([r, g, b, a])))| {
            binding.put_pixel(x, y, Rgba([r, g + text[i], b, a]));
        });

    binding.save("./assets/test.png").unwrap();
}
