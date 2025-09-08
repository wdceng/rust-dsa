fn main() {
    let mut s = "   hello   ".to_string();

    let p = s.trim();
    let p = p.to_string();

    s.push_str("goodbye");

    println!("p == '{}'", p);

    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("ffstr = '{}'", ffstr);
    println!("chosen = '{}'", choser_str(1));
}

fn string_find_f<'a>(s: &'a str) -> &'a str {
    for (n, x) in s.char_indices() {
        if x == 'f' {
            return &s[n..];
        }
    }
    s
}

fn choser_str(n: i32) -> &'static str {
    match n {
        0 => "hello",
        1 => "goodbye",
        _ => "other",
    }
}
