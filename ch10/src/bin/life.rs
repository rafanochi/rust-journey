fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

struct Waifu<'a> {
    name: &'a mut str,
    surname: &'a str,
}

fn main() {
    let x = String::from("asuna");
    let y = String::from("senjougahara");

    let mut kawori = Waifu {
        name: "Kawori".as_mut(),
        surname: "Miyazono",
    };

    kawori.name = "Kaori".as_mut();

    let r = longest(&x, &y);
}
