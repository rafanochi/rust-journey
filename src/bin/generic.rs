struct Waifu<T> {
    child: T,
}

impl<T> Waifu<T> {
    fn new(child: T) -> Self {
        Self { child }
    }
}

#[derive(Clone)]
struct Node<T>
where
    T: Clone,
{
    data: Box<T>,
    next: Option<Box<Node<T>>>,
}

enum Recursive<T> {
    Next(Box<Recursive<T>>),
    Boxed(Box<T>),
    Optional(Option<T>),
}

enum NextNode<T> {
    Next(Box<ListNode<T>>),
    End,
}

struct ListNode<T> {
    data: Box<T>,
    next: NextNode<T>,
}

fn main() {
    let is_child = Waifu { child: false };
    let her_child: Waifu<Option<String>> = Waifu { child: None };
    let her_and_my_child = Waifu::<Option<String>>::new(None); // idomatic
}
