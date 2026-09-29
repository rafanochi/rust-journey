use rust_journey::ThumSize;
use rust_journey::util::thumbnail;

fn main() -> anyhow::Result<()> {
    thumbnail("./test/reze.jxl", ThumSize::Normal)?;
    Ok(())
}
