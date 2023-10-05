use std::fmt::Display;

fn info<T:Display>(input: &T) {
    println!("{}", input);
}

fn main() {
    let x = String::from("what");
    info(&x);
    info(&"abc");
    info(&"abc".to_string());
    info(&"abc".to_ascii_lowercase());
}
