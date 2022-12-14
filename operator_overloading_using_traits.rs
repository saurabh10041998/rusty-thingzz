use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x  + other.x,
            y: self.y + other.y
        }
    }
}

fn main() {
    let p1 = Point {
        x: 1,
        y: 0
    };

    let p2 = Point {
        x: 2,
        y : 1
    };

    assert_eq!(p1 + p2, Point{x:3, y: 1});

}