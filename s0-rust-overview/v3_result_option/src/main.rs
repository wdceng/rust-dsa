/*
// Similar as Result, but without output for error
// Use Option<T> when a value might be missing (like Rust’s safe alternative to null); Some(value) means it's present, None means it's absent — great for return values, optional struct fields, or anything that might not exist.

pub enum Option<T> {
    Some(T),
    None,
}
*/

fn main() {
    let a = divide(10, 5);
    let b = divide(10, 0);

    match a {
        Ok(v) => println!("val = {}", v),
        _ => {}
    }

    if let Ok(v) = a {
        println!("val = {}", v);
    }

    println!("a = {:?}, b =  {:?}", a, b);
}

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Cannot divide by zero".to_string());
    }
    Ok(a / b)
}

/*
// Our own Result enum

#[derive(Debug)]
pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn main() {
    let a = divide(10, 5);
    let b = divide(10, 0);

    match a {
        Res::Thing(v) => println!("val = {}", v),
        _ => {}
    }

    if let Res::Thing(v) = a {
        println!("val = {}", v);
    }

    println!("a = {:?}, b =  {:?}", a, b);
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if b == 0 {
        return Res::Error("Cannot divide by zero".to_string());
    }
    Res::Thing(a / b)
}
 */
