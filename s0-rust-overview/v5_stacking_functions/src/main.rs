// =======================================================
// Demonstration of Regular vs Tail Recursion in Rust
// Highlights how regular recursion builds up call stack frames,
// while tail recursion avoids stacking by doing all work up front.
// =======================================================

fn main() {
    println!("Factorial with regular recursion: {}", factorial_regular(5));
    println!("Factorial with tail recursion: {}", factorial_tail(5, 1));
}

// Recursive calls with large input (e.g. 1,000,000) can cause a stack overflow
fn factorial_regular(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    n * factorial_regular(n - 1) // ← More work after the call (multiplication)
}

// Tail-recursive: allows manual conversion to a loop to avoid stack overflow
fn factorial_tail(n: i32, r: i32) -> i32 {
    if n <= 1 {
        return r;
    }
    factorial_tail(n - 1, n * r) // ← No work after the call → this is tail-recursive
}
