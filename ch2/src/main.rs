const WAIFU: &str = "Asuna";

fn main() {
    let x = 34;
    {
        let x = x * x;
        println!("x inside scope: {x}");
    }
    let x = 900_000;
    println!("x OUTside scope: {x}");
    println!("My waifu is {WAIFU}");
}
