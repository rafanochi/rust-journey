use rust_journey::util::thumbnail;

fn main() -> anyhow::Result<()> {
    let thumb_path = thumbnail("./test/reze.jxl", rust_journey::ThumSize::Normal)?;
    println!(
        "Thumbnail path: file://{}",
        thumb_path.to_str().unwrap_or_default()
    );
    Ok(())
}
