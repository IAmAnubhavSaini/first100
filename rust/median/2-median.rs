fn odd(list: Vec<i32>, len: usize) -> f64 {
    list[len/2] as f64
}

fn even(list: Vec<i32>, len: usize) -> f64 {
    ((list[len/2-1] + list[len/2])  as f64) / 2.0
}

const MEDIANS: [fn(Vec<i32>, usize) -> f64; 2] = [even, odd];

fn median(v: Vec<i32>) -> f64 {
    let mut list = v.to_vec();
    list.sort();
    let len = list.len();
    MEDIANS[len%2](list, len)
}

fn main() {
    let mut v = vec![5, 4, 3, 2, 1];
    println!("{}", median(v));

    v = vec![3, 2, 1];
    println!("{}", median(v));

    v = vec![4, 3, 2, 1];
    println!("{}", median(v));

    v = vec![1, 2, 8, 6, 4, 2, 1, 2, 54, 43, 46];
    println!("{}", median(v));
}
