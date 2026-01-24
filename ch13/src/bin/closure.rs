use std::{thread, vec};

#[derive(Debug, Clone, Copy)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

enum Maybe<T> {
    Just(T),
    Nothing,
}
impl<T> Maybe<T> {
    fn unwrap_or_else<F>(self, closure: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self {
            Self::Just(x) => x,
            Self::Nothing => closure(),
        }
    }
}

#[derive(Debug)]
struct Waifu<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Blue);
    let giveaway = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let confess = |waifu: &str, name: &str| println!("{waifu}: love you {name}");
    confess("Rem", "Rafa");

    let square = |x: u32| x * x;

    let mut example = vec![4, 1, 6, 2];
    let mut borrows_mutably = || example.push(10000);
    borrows_mutably();

    thread::spawn(move || println!("From thread: {example:?}"))
        .join()
        .unwrap();

    let mut operations = vec![];
    let s = "Called from closure";

    let mut waifues = vec![
        Waifu {
            name: "Asuna",
            age: 18,
        },
        Waifu {
            name: "Mizuhara",
            age: 20,
        },
    ];

    waifues.sort_by_key(|w| {
        operations.push(s);
        w.age
    });
    println!("{waifues:?}");
}
