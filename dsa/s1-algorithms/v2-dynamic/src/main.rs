fn main() {
    for i in 0..10 {
        print!("Naive: {}, iter: {} ", fibonacci(i), fibonacci_iter(i));
        println!(
            "dynamic: {}, tail: {}",
            fibonacci_dynamic(i).0,
            fibonacci_tail(i)
        );
    }
}

// O(2^n)
// fib(5) = fib(4) + fib(3) = fib(3) + fib(2) + fib(2) + fib(1)
// = fib(2) + fib(1) + fib(1) + fib(0) + fib(1) + fib(0) + fib(1) = 8

pub fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }

    // Fibonacci(n-1) will call fibonacci(n-2) so we do that function twice
    // fibonacci(n - 2) is computed twice: once directly, and once through fibonacci(n - 1)
    fibonacci(n - 1) + fibonacci(n - 2)
}

// O(n)
pub fn fibonacci_iter(n: i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut res = 1;

    for _ in 1..n {
        res = a + b;
        a = b;
        b = res;
    }

    res
}

// Dynamic
// return(res, prev), prev -> previous result
// If you are going to use the same function more than once, store the result somewhere
pub fn fibonacci_dynamic(n: i32) -> (i32, i32) {
    if n == 0 {
        return (1, 0);
    }

    let (a, b) = fibonacci_dynamic(n - 1);
    (a + b, a)
}

// Fibonacci number using a tail-recursive approach.
// Tail recursion avoids redundant work by passing the accumulated state
// through each recursive call. This is efficient and runs in O(n) time.
pub fn fibonacci_tail(n: i32) -> i32 {
    // Internal helper function:
    // - `n` is the countdown to zero (how many steps left)
    // - `a` holds F(n - 1)
    // - `b` holds F(n)
    fn fib_tail(n: i32, a: i32, b: i32) -> i32 {
        // Base case: when n reaches 0, return the accumulated result
        if n == 0 {
            return a;
        }
        // Tail-recursive step: slide the (a, b) window forward
        fib_tail(n - 1, b, a + b)
    }

    // Start with F(0) = 0, F(1) = 1
    fib_tail(n, 1, 1)
}
