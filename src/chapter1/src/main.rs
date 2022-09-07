fn search(items: &[i32], value: i32) -> Option<usize> {
    for i in 0..items.len() {
        if items[i] == value {
            return Some(i);
        }
    }

    return None;
}

fn main() {
    let items = [0, 12, 6, 18, 3, 4];
    let index1 = search(&items, 6);
    println!("find({:?}, {}) = {:?}", items, 6, index1);

    let index2 = search(&items, 5);
    println!("find({:?}, {}) = {:?}", items, 5, index2);
}
