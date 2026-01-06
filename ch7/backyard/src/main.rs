pub mod garden;

use crate::garden::vegetables;

fn main() {
    let a = vegetables::Aparagus {};
    println!("a: {:#?}", a);
}
