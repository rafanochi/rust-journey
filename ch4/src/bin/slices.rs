use std::io;

fn first_word(s: &String) -> String {
    let mut temp = String::from("");
    for item in s.chars() {
        if item == ' ' || item == '\n' {
            break;
        }
        let c = item.to_string();
        temp.push_str(&c);
    }

    temp
}

fn main() {
    println!("enter a string: ");
    let mut s = String::from("");

    io::stdin().read_line(&mut s).expect("Couldn't read line");

    let w = first_word(&s);
    println!("FIRST WORD: {w}");
}
