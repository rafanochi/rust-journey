enum DistinctTypes {
    Name(String),
    Count(i32),
}
fn match_enum_types(enum_types: &DistinctTypes) {
    match enum_types {
        DistinctTypes::Name(name) => println!("name={name}"),
        DistinctTypes::Count(count) => println!("count={count}"),
    }
}

enum CatColor {
    Black,
    Red,
    Chocolate,
    Cinnamon,
    Blue,
    Cream,
    Cheshire,
}

struct Cat {
    name: String,
    color: CatColor,
}

fn match_on_black_cats(cat: &Cat) {
    match cat {
        Cat {
            name,
            color: CatColor::Black,
        } => println!("This is a black cat named {name}"),
        Cat { name, color: _ } => println!("{name} is not a black cat"),
    }
}

fn write_to_file_io_result() -> std::io::Result<()> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("filename")?;
    file.write_all(b"File contents")?;
    Ok(())
}

fn try_to_write_to_file_io_result() {
    match write_to_file_io_result() {
        Ok(()) => println!("Write succeeded"),
        Err(err) => println!("Write failed: {}", err.to_string()),
    }
}

enum ErrorTypes {
    IoError(std::io::Error),
    FormatError(std::fmt::Error),
}
struct ErrorWrapper {
    source: ErrorTypes,
    message: String,
}
impl From<std::io::Error> for ErrorWrapper {
    fn from(source: std::io::Error) -> Self {
        Self {
            source: ErrorTypes::IoError(source),
            message: "there was an IO error!".into(),
        }
    }
}

fn write_to_file() -> Result<(), ErrorWrapper> {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::create("filename")?;
    file.write_all(b"File contents")?;
    Ok(())
}
fn try_to_write_to_file() {
    match write_to_file() {
        Ok(()) => println!("Write succeeded"),
        Err(err) => {
            println!("Write failed: {}", err.message)
        }
    }
}

fn main() {}
