#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

fn main() {
    let mut x = 34;
    let y = x;
    x += 5;
    println!("y:{} x:{}", y, x);

    let mut p = Person {
        name: String::from("Matt"),
        age: 34,
    };

    let p2 = p.clone();
    p.name.push_str(" Davor");
    println!("{:?}\n{:?}", p, p2);

    let mut pnt = Point::new(3, 4);
    let pn2 = pnt;
    pnt.x += 3;
    println!("{:?}\n{:?}", pnt, pn2);
}
