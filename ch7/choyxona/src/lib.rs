mod front_of_house;

use front_of_house::hosting;
use front_of_house::hosting::serving;

fn eat_at_choyxona() {
    front_of_house::hosting::add_to_waitlist();
    serving::cook();
}

fn some_func() {
    println!("Helloooo")
}
