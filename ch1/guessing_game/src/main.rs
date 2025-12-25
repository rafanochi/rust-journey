use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let random_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess the number: ");

        let mut str = String::new();
        io::stdin().read_line(&mut str).expect("couldn't read");

        let guess: i32 = str.trim().parse().expect("error in parsing");

        match guess.cmp(&random_num) {
            Ordering::Less => println!("Lesser"),
            Ordering::Equal => {
                println!("YOU WOOOONN");
                break;
            }
            Ordering::Greater => println!("Greater"),
        };
    }
}
