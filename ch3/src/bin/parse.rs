use std::io;

fn main() {
    let a = [5; 3]; // a = [5,5,5,5]

    println!("okay now enter array index: ");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("couldn't read the line");

    let u_index: usize = index.trim().parse().expect("asdfasdf");

    println!("a[{u_index}] = {}", a[u_index]);
}
