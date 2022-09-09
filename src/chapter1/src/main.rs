fn search(items: &[i32], value: i32) -> Option<usize> {
    for i in 0..items.len() {
        if items[i] == value {
            return Some(i);
        }
    }

    return None;
}

#[test]
fn search_should_return_none_when_empty_items() {
    assert_eq!(None, search(&vec![], 1));
}

#[test]
fn search_should_return_none_when_items_doesnt_contain_value() {
    assert_eq!(None, search(&vec![2, 3, 4], 1));
}

#[test]
fn search_should_return_index_when_items_contains_value() {
    assert_eq!(Some(2), search(&vec![3, 2, 1, 0], 1));
}

fn contains(items: &[i32], value: i32) -> bool {
    for item in items.iter() {
        if *item == value {
            return true;
        }
    }

    return false;
}

#[test]
fn contains_should_return_false_when_empty_items() {
    assert_eq!(false, contains(&vec![], 1));
}

#[test]
fn contains_should_return_false_when_items_doesnt_contain_value() {
    assert_eq!(false, contains(&vec![2, 3, 4], 1));
}

#[test]
fn contains_should_return_true_when_items_contains_value() {
    assert!(contains(&vec![3, 2, 1, 0], 1));
}

fn main() {
    let items = [0, 12, 6, 18, 3, 4];

    println!("search({:?}, {}) = {:?}", items, 6, search(&items, 6));
    println!("search({:?}, {}) = {:?}", items, 5, search(&items, 5));

    println!("contains({:?}, {}) = {}", items, 6, contains(&items, 6));
    println!("contains({:?}, {}) = {}", items, 5, contains(&items, 5));
}
