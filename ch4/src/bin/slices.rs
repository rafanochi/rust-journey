use std::io;

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s
}

fn main() {
    println!("enter a string: ");
    let mut s = String::from("");

    io::stdin().read_line(&mut s).expect("Couldn't read line");

    let w = first_word(&s);
    println!("FIRST WORD: {w}");

    let ss = String::from("Hitagi Senjougahara");
    let hitagi = &ss[..6];
    let senjougahara = &ss[7..];

    println!("{hitagi} {senjougahara}");
}
