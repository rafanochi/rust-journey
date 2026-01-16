use std::fmt::Display;

static BEST_WAIFU: &str = "Reze";

fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Best waifu is: {ann}");
    if x.len() > y.len() { x } else { y }
}

struct Waifu<'a> {
    name: &'a str,
    surname: &'a str,
}
impl<'a> Waifu<'a> {
    fn find_love(self, s: &str) -> &str {
        match s.find("love") {
            Some(i) => &s[i..i + 4],
            _ => s,
        }
    }
}

fn main() {
    let waifu: &'static str = "Hanekawa";
    let x = String::from("asuna");
    let y = String::from("senjougahara");

    let kawori = Waifu {
        name: "Kawori",
        surname: "Miyazono",
    };

    let r = longest(&x, &y, BEST_WAIFU);
}
