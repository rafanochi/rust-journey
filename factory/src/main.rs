use relm4::prelude::DynamicIndex;

#[derive(Debug)]
struct Counter {
    value: u8,
}

#[derive(Debug)]
enum CounterMsg {
    Increment,
    Decrement,
}

enum CounterOutput {
    MoveUp(DynamicIndex),
    MoveDown(DynamicIndex),
    MoveTop(DynamicIndex),
}

fn main() {
    todo!();
}
