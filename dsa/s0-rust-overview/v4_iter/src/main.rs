pub struct Stepper {
    curr: i32,
    step: i32,
    max: i32,
}

impl Iterator for Stepper {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if self.curr > self.max {
            return None;
        }
        let res = self.curr;
        self.curr += self.step;
        Some(res)
    }
}

fn main() {
    let mut n = 0;
    loop {
        n += 1;
        if n > 3 {
            break;
        }
        println!("Hello, loop {}", n);
    }
    println!("All done");

    n = 0;
    while n < 3 {
        n += 1;
        println!("Hello, while {}", n);
    }
    println!("All done");

    for i in 1..=3 {
        println!("Hello, for {}", i);
    }
    println!("All done");

    let mut st = Stepper {
        curr: 1,
        step: 1,
        max: 3,
    };

    loop {
        match st.next() {
            Some(v) => println!("Stepper loop {}", v),
            None => break,
        }
    }
    println!("All done");

    st = Stepper {
        curr: 1,
        step: 1,
        max: 3,
    };

    while let Some(n) = st.next() {
        println!("Stepper while let {}", n);
    }
    println!("All done");

    st = Stepper {
        curr: 1,
        step: 1,
        max: 3,
    };

    for i in st {
        println!("Stepper for {}", i);
    }
    println!("All done");
}
