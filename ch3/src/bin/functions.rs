fn main() {
    let a = say_hello("Asuna", "Shahruz");
    println!("{}", &a);
}

fn say_hello(waifu: &str, name: &str) -> String {
    format!("{waifu}: I love you {name}")
}
