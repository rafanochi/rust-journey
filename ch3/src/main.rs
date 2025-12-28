fn main() {
    let x: i8 = 8; // can store -2^7...2^7-1 -> -127...127
    let t: (f64, u8, char) = (0.0001, 124, '$');

    let _st = t.0;
    println!("x: {x}, t: {:?}", t);

    let haha = if x == 5 { 10 } else { 20 };
    println!("haha: {haha}");

    let mut counter = 0;
    'outer_loop: loop {
        let mut i = 10;
        println!("{i}");
        loop {
            if i == 0 {
                break;
            }
            i -= 1;
            println!("{i}")
        }
        counter += 1;
        if counter == 5 {
            break 'outer_loop;
        }
    }

    let _arr = [234, 123, 12, 31, 31, 23, 124, 1243, 0];
    for j in (0..=10).rev() {
        println!("{}", j)
    }
}
