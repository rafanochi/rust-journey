use std::{fmt::Display, marker::PhantomData};
// use std::fmt::;

#[derive(Debug)]
struct KB {}
// impl ToString for KB {
//     fn to_string(&self) -> String {
//         String::from("Kilobytes")
//     }
// }
// impl Display for KB {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("Kilobytes")
//     }
// }

#[derive(Debug)]
struct MB {}

#[derive(Debug)]
struct GB {}

#[derive(Debug)]
struct TB {}

#[derive(Debug)]
struct Size<SizeType> {
    bytes: u64,
    size_type: PhantomData<SizeType>,
}

impl Size<KB> {
    fn new(x: u64) -> Self {
        Self {
            bytes: x * 1000,
            size_type: PhantomData,
        }
    }
    fn type_name(&self) -> &str {
        "Kilobyte"
    }
}
impl Display for Size<KB> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (data, tip) = (self.bytes / 1000, self.type_name());

        f.write_str(&format!("{data} {tip}"))
    }
}

fn main() {
    let x: Size<KB> = Size::new(1);
    println!("{:?}", x.to_string())
}
