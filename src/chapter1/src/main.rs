extern crate num_traits;

fn search(items: &[i32], value: i32) -> Option<usize> {
    for i in 0..items.len() {
        if items[i] == value {
            return Some(i)
        }
    }

    None
}

#[cfg(test)]
mod search_should {
    #[test]
    fn return_none_when_empty_items() {
        assert_eq!(None, super::search(&vec![], 1));
    }

    #[test]
    fn return_none_when_items_doesnt_contain_value() {
        assert_eq!(None, super::search(&vec![2, 3, 4], 1));
    }

    #[test]
    fn return_some_index_when_items_contains_value() {
        assert_eq!(Some(2), super::search(&vec![3, 2, 1, 0], 1));
    }
}

fn contains(items: &[i32], value: i32) -> bool {
    for item in items.iter() {
        if *item == value {
            return true
        }
    }

    false
}

#[cfg(test)]
mod contains_should {
    #[test]
    fn return_false_when_empty_items() {
        assert_eq!(false, super::contains(&vec![], 1));
    }

    #[test]
    fn return_false_when_items_234_dont_contain_1() {
        assert_eq!(false, super::contains(&vec![2, 3, 4], 1));
    }

    #[test]
    fn return_true_when_items_3210_contain_1() {
        assert!(super::contains(&vec![3, 2, 1, 0], 1));
    }
}

fn min(items: &[i32]) -> Option<i32> {
    if items.is_empty() {
        None
    } else {
        let mut result = items[0];

        for i in 1..items.len() {
            if items[i] < result {
                result = items[i]
            }
        }

        Some(result)
    }
}

#[cfg(test)]
mod min_should {
    #[test]
    fn return_none_when_empty_items() {
        assert_eq!(None, super::min(&vec![]));
    }

    #[test]
    fn return_some_1_when_items_are_1() {
        assert_eq!(Some(1), super::min(&vec![1]));
    }

    #[test]
    fn return_some_1_when_items_are_23145() {
        assert_eq!(Some(1), super::min(&vec![2, 3, 1, 4, 5]));
    }
}

use std::ops::AddAssign;
use num::traits::Zero;

fn sum<T>(items: &[T]) -> T
where T: Copy + AddAssign + Zero {
    let mut result: T = Zero::zero();

    for item in items.iter() {
        result.add_assign(*item);
    }

    result
}

#[cfg(test)]
mod sum_should {
    #[test]
    fn return_0_when_empty_items() {
        assert_eq!(0, super::sum(&vec![]));
    }

    #[test]
    fn return_10_when_items_are_1234() {
        assert_eq!(10, super::sum(&vec![1, 2, 3, 4]));
    }
}

fn prod(items: &[i32]) -> i32 {
    let mut result = 1;

    for item in items.iter() {
        result *= *item
    }

    result
}

#[cfg(test)]
mod prod_should {
    #[test]
    fn return_1_when_empty_items() {
        assert_eq!(1, super::prod(&vec![]));
    }

    #[test]
    fn return_24_when_items_are_1234() {
        assert_eq!(24, super::prod(&vec![1, 2, 3, 4]));
    }
}

fn average(items: &[f64]) -> Option<f64> {
    if items.is_empty() {
        None
    } else {
        Some(sum(&items) / items.len() as f64)
    }
}

#[cfg(test)]
mod average_should {
    #[test]
    fn return_none_when_empty_items() {
        assert_eq!(None, super::average(&vec![]));
    }

    #[test]
    fn return_some_3_when_items_are_12345() {
        assert_eq!(Some(3.0), super::average(&vec![1.0, 2.0, 3.0, 4.0, 5.0]));
    }
}

mod md5;

fn main() {
    let empty = Vec::<i32>::new();
    let items = [0, 12, 6, 18, 3, 4];

    let empty_float = Vec::<f64>::new();
    let float_items = [0.0, 12.0, 6.0, 18.0, 3.0, 4.0];

    println!("search({:?}, {}) = {:?}", items, 6, search(&items, 6));
    println!("search({:?}, {}) = {:?}", items, 5, search(&items, 5));

    println!("contains({:?}, {}) = {}", items, 6, contains(&items, 6));
    println!("contains({:?}, {}) = {}", items, 5, contains(&items, 5));

    println!("min({:?}) = {:?}", empty, min(&empty));
    println!("min({:?}) = {:?}", items, min(&items));

    println!("sum({:?}) = {:?}", empty, sum(&empty));
    println!("sum({:?}) = {:?}", items, sum(&items));

    println!("prod({:?}) = {:?}", empty, prod(&empty));
    println!("prod({:?}) = {:?}", items, prod(&items));

    println!("average({:?}) = {:?}", empty_float, average(&empty_float));
    println!("average({:?}) = {:?}", float_items, average(&float_items));

    let (a, b, c, d) = md5::md5(&vec![]);
    println!("md5([]) = {:08x}-{:08x}-{:08x}-{:08x}", a, b, c, d);
}
