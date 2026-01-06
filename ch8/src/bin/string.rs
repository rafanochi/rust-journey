fn main() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let mut hello = String::from("Hola");

    hello.push_str(" WORLD");

    let x = String::from("Asuna ");
    let y = String::from("Yuuki");

    let w = format!("{x}{y}");

    let waifu = x + &y + " is MY WAIFU";

    // let a = &waifu[0];

    let hello = "HEEEELLOOOO";
    let h = &hello[0..5];

    for i in hello.chars() {
        println!("{i}");
    }
}
