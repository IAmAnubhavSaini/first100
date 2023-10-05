fn unique(list: Vec<i32>) -> Vec<i32> {
    let mut new_list: Vec<i32> = vec![];
    for i in 0..list.len() {
        let num = list[i];
        if !new_list.contains(&num) {
            new_list.push(num);
        }
    }
    return new_list;
}

fn main() {
    let mut list = vec![1, 2, 3, 4, 5, 6];
    println!("original: {:?}, unique: {:?}", list, unique(list.clone()));
    list = vec![1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6];
    println!("original: {:?}, unique: {:?}", list, unique(list.clone()));
    list = vec![1, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6];
    println!("original: {:?}, unique: {:?}", list, unique(list.clone()));
}

/*
./1-unique       
original: [1, 2, 3, 4, 5, 6], unique: [1, 2, 3, 4, 5, 6]
original: [1, 2, 3, 4, 5, 6, 1, 2, 3, 4, 5, 6], unique: [1, 2, 3, 4, 5, 6]
original: [1, 6, 5, 4, 3, 2, 1, 2, 3, 4, 5, 6], unique: [1, 6, 5, 4, 3, 2]
*/
