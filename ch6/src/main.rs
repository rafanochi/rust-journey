#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Maybe<T> {
    Nothing,
    Just(T),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Technology {
    PC(u8),
    Laptop,
    Phone,
}

impl Technology {
    fn created_in(&self) -> u16 {
        match self {
            Technology::PC(_) => 1930,
            Technology::Laptop => 1998,
            Technology::Phone => 2009,
        }
    }
}

fn cost(t: Technology) -> usize {
    match t {
        Technology::PC(discount) => {
            println!("{discount}% OFFFFFFF");
            2000
        }
        Technology::Laptop => 1000,
        Technology::Phone => 500,
    }
}

fn discount(t: Technology) -> Option<u8> {
    let Technology::PC(d) = t else {
        return None;
    };
    Some(d)
}

fn increment(x: Option<i32>) -> Option<i32> {
    x.map(|i| i + 1)
}

fn main() {
    let some_number = Maybe::Just(5);
    let some_char = Maybe::Just('c');
    let absent_number: Maybe<i32> = Maybe::Nothing;

    let (home, loopback) = (
        IpAddrKind::V4(127, 0, 0, 1),
        IpAddrKind::V6("::1".to_string()),
    );

    println!("home: {:#?},\nloopback: {:#?}", home, loopback);

    let five = increment(Some(4));
    let none = increment(None);

    let map_pro = Technology::PC(50);

    cost(Technology::PC(90));
    discount(map_pro);
}
