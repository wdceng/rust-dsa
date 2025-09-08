#[derive(Debug, Clone)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Person { name, age }
    }

    // self        - Takes ownership of the instance
    // mut self    - Takes ownership and allows mutation of the instance
    // &self       - Borrows the instance immutably
    // &mut self   - Borrows the instance mutably

    fn greet(&self) -> String {
        format!("Hi my name is {}", self.name)
    }

    fn age_up(&mut self, n: i32) {
        self.age += n
    }

    fn dropme(self) {}
}
fn main() {
    let mut p = Person::new(String::from("matt"), 35);
    p.age_up(3);
    let s = p.greet();
    println!("{}", s);

    let a = get_age(&p);
    println!("{}", a);
    p.age_up(2);

    // p.dropme();

    let s2 = p.greet();
    println!("{}", s2);
}

fn get_age(s: &Person) -> &i32 {
    &s.age
}
