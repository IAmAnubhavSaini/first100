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


fn median(v:Vec<i32>) -> f64 {
    let mut list = v.to_vec();
    list.sort();
    let len = list.len();
    return if len % 2 == 0 {
        let mid = len / 2;
        (list[mid] + list[mid - 1]) as f64 / 2.0
    } else {
        list[len / 2] as f64
    };
}

