#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    fave_color: Color,
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

impl Person {
    pub fn print(self) -> String {
        format!(
            "name = {}, age = {} has {} children and color {:?}",
            self.name, self.age, self.children, self.fave_color,
        )
    }
}

fn main() {
    let p = Person {
        name: "matt".to_string(),
        age: 35,
        children: 4,
        fave_color: Color::Green,
    };

    let c = Color::Red("hello".to_string());

    match c {
        Color::Red(s) => println!("It's red {}", s),
        Color::Green => println!("It's red"),
        Color::Blue => println!("It's red"),
    }

    println!("Hello, people from {:?}", p);
}
