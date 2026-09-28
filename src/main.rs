use rust_journey::ThumSize;
use rust_journey::util::thumbnail;

fn main() {
    thumbnail("test/reze.jxl", ThumSize::Normal).unwrap();
}
