use std::cell::RefCell;
use std::rc::Rc;

type ItemData<T> = Rc<RefCell<T>>;
type ListItemPtr<T> = Rc<RefCell<ListItem<T>>>;

struct Iter<T> {
    next: Option<ListItemPtr<T>>,
}
struct IterMut<T> {
    next: Option<ListItemPtr<T>>,
}
struct IntoIter<T> {
    next: Option<ListItemPtr<T>>,
}

impl<T> Iterator for Iter<T> {
    type Item = ItemData<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            None => None,
            Some(ptr) => {
                self.next.clone_from(&ptr.as_ref().borrow().next);
                Some(ptr.as_ref().borrow().data.clone())
            }
        }
    }
}

impl<T> Iterator for IterMut<T> {
    type Item = ItemData<T>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            None => None,
            Some(ptr) => {
                self.next.clone_from(&ptr.as_ref().borrow().next);
                Some(ptr.as_ref().borrow().data.clone())
            }
        }
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            None => None,
            Some(ptr) => {
                self.next = ptr.as_ref().borrow().next.clone();
                let list_item = Rc::try_unwrap(ptr).map(|refcell| refcell.into_inner());
                match list_item {
                    Ok(x) => Rc::try_unwrap(x.data)
                        .map(|refcell| refcell.into_inner())
                        .ok(),
                    Err(_) => None,
                }
            }
        }
    }
}

#[derive(Debug)]
struct ListItem<T> {
    data: ItemData<T>,
    next: Option<ListItemPtr<T>>,
}

impl<T> ListItem<T> {
    fn new(t: T) -> Self {
        Self {
            data: Rc::new(RefCell::new(t)),
            next: None,
        }
    }
}

struct LinkedList<T> {
    head: ListItemPtr<T>,
    cur_iter: Option<ListItemPtr<T>>,
}
impl<T> LinkedList<T> {
    fn new(x: T) -> Self {
        Self {
            head: Rc::new(RefCell::new(ListItem::new(x))),
            cur_iter: None,
        }
    }
    fn append(&mut self, x: T) {
        let mut next = self.head.clone();
        while next.as_ref().borrow().next.is_some() {
            let n = next.as_ref().borrow().next.as_ref().unwrap().clone();
            next = n;
        }
        next.as_ref().borrow_mut().next = Some(Rc::new(RefCell::new(ListItem::new(x))))
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            next: Some(self.head.clone()),
        }
    }
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: Some(self.head.clone()),
        }
    }
    fn into_iter(self) -> IntoIter<T> {
        IntoIter {
            next: Some(self.head.clone()),
        }
    }
}

fn main() {
    let one = || 1;
    let two = || 2;

    let adder = |left: fn() -> i32, right: fn() -> i32| {
        let (x, y) = (left(), right());
        println!("{x} + {y} = {}", x + y)
    };

    adder(one, two);

    let mut dinosaurs = LinkedList::new("Tyrannosaurus Rex");
    dinosaurs.append("Triceratops");
    dinosaurs.append("Velociraptor");
    dinosaurs.append("Stegosaurus");
    dinosaurs.append("Spinosaurus");
    dinosaurs.into_iter().for_each(|x| println!("{}", x));
}
