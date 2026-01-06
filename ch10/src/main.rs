fn largest<T: PartialOrd>(xs: &[T]) -> &T {
    let mut largest = &xs[0];

    for x in xs {
        if x > largest {
            largest = x;
        }
    }

    largest
}

struct Point<T, U> {
    x: T,
    y: U,
}
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn mixup<TT, UU>(self, other: Point<TT, UU>) -> Point<T, UU> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Either<T, E> {
    Right(T),
    Left(E),
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
