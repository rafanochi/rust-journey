use image::{GenericImage, GenericImageView, Rgba, open};

static SRC: &str = "./assets/reze.png";
static DEST: &str = "./assets/result.png";
static TEXT: &str = "Asuna";
static PATTERN: &str = "!@#$";

fn main() -> anyhow::Result<()> {
    let text: Vec<_> = format!("{PATTERN}{TEXT}{PATTERN}").into();

    // Hide the text inside text
    println!("======HIDING=====");
    let mut image = open(SRC)?;
    image
        .clone()
        .pixels() // take pixels
        .enumerate() // add indexing behaviour
        .take(text.len()) // only take starting pixels
        .for_each(|(i, (x, y, Rgba([r, _g, b, a])))| {
            // replace green channel with hidden text
            image.put_pixel(x, y, Rgba([r, text[i], b, a]));
        });

    image.save(DEST)?;

    // Read it back
    println!("======READING=====");
    let pattern = PATTERN.as_bytes();
    let image = open(DEST)?;

    // getting only green channels of pixels
    let pixels = image
        .pixels()
        .map(|(_, _, Rgba([_r, g, _b, _a]))| g.clone())
        .collect::<Vec<u8>>();

    // creating text-length chunks from pixel array, and searching for 
    // a chunk that satisfies our hidden data
    let result = pixels
        .chunks(text.len())
        .find_map(|x| x.strip_circumfix(pattern, pattern));

    println!(
        "Hidden text is: {:?}",
        String::from_utf8_lossy(result.unwrap_or_default())
    );

    Ok(())
}
