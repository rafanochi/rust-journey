fn main() {
    let mut waifues: Vec<&str> = Vec::new();
    let mut waifues = vec!["Asuna", "Rem"];
    let mut v = vec![1, 2, 3]; // i32 -> default int

    waifues.push("Asuna");
    waifues.push("Rem");

    let fst: &str = &waifues[0];
    let st = waifues.get(0);
    match st {
        Some(x) => println!("the value is {x}"),
        _ => println!("element doesn't exist in the vector"),
    }

    v.push(10);

    let v0 = &v[0];

    waifues.push("Senjougahara");

    for i in &waifues {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    println!("{:#?}", v);
}

