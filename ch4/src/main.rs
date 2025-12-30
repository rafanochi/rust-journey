fn main() {
    let mut s1 = String::from("Bir narsaaaa");
    {
        let s2 = &mut s1;
        println!("s2: {s2}")
    }

    let r = &s1;
    println!("r: {r}");

    println!("r after r3: {r}");

    let s1 = some_fun(&s1);

    println!("{:?}", s1);
}

// write way to write a function!!! PURE FUNCTION
fn some_fun(s: &String) -> String {
    format!("{} EMAS!", s)
}
