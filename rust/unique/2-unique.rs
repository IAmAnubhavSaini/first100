fn unique(list: &mut Vec<i32>) -> Vec<i32> {
    list.sort();
    list.dedup();
    list.to_vec()
}


fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6];
    println!("original: {:?}, unique: {:?}", list, unique(&mut list.clone()));
    list = vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6];
    println!("original: {:?}, unique: {:?}", list, unique(&mut list.clone()));
    list = vec![1, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6];
    println!("original: {:?}, unique: {:?}", list, unique(&mut list.clone()));
}
