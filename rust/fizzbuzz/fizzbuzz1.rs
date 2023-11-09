fn fizz(n:i32) -> String {
    return (if n%3 == 0 { "fizz" } else { "" }).to_string();
}

fn sizz(n:i32)-> String {
    return (if n%4 == 0 { "sizz" } else { "" }).to_string();
}

fn buzz(n:i32)-> String {
    return (if n%5 == 0 { "buzz" } else { "" }).to_string();
}

fn run_fizzbuzz(n: i32) {
    let fns = vec![fizz, sizz, buzz];
    for i in 1..(n+1){
        let val = fns
            .iter()
            .fold(String::new(), |mut a, fx| { a.push_str(&fx(i)); a });
        println!("{i}: {}", if val.len() == 0 { i.to_string() } else {val});
    }
}

fn main() {
    run_fizzbuzz(100);
}
