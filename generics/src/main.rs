#![allow(unused)]
#[derive(Debug)]
struct Point<X, Y> {
    x: X,
    y: Y,
}

impl<X, Y> Point<X, Y> {
    fn get_x(&self) -> &X {
        &self.x
    }

    fn get_y(&self) -> &Y {
        &self.y
    }
}

fn mixup<X1, Y1, X2, Y2>(p1: Point<X1, Y1>, p2: Point<X2, Y2>) -> Point<X1, Y2> {
    Point { x: p1.x, y: p2.y }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let num_list = vec![69, 420, 1337];
    let res = largest(&num_list);
    assert_eq!(*res, 1337);
    println!("largest: {}", *res);

    let char_list = vec!['a', 'z'];
    let res = largest(&char_list);
    assert_eq!(*res, 'z');
    println!("largest: {}", *res);

    let p1 = Point { x: 69, y: 420 };
    let p2 = Point { x: 69.0, y: 420.0 };
    let p3 = Point { x: 69, y: 420.0 };
    println!("p1.x = {}", p1.get_x());
    println!("p3.x = {}", p3.get_y());
    println!("p2 distance from origin: {}", p2.distance_from_origin());

    let p4 = mixup(p2, p3);
    println!("p4: {:#?}", p4);
}

fn largest<T: std::cmp::Ord>(list: &[T]) -> &T {
    list.iter().max().unwrap()
}
