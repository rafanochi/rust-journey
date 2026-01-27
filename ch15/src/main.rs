enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    use List::{Cons, Nil};

    let b = Box::new(8);
    println!("{b}");

    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = &x;
    let z = Box::new(1000);

    println!("{}", x);
    println!("{}", *y);
    println!("{}", *z);
}
