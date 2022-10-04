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
        assert_eq!(None, super::search(&[], 1));
    }

    #[test]
    fn return_none_when_items_doesnt_contain_value() {
        assert_eq!(None, super::search(&[2, 3, 4], 1));
    }

    #[test]
    fn return_some_index_when_items_contains_value() {
        assert_eq!(Some(2), super::search(&[3, 2, 1, 0], 1));
    }
}

fn contains(items: &[i32], value: i32) -> bool {
    for item in items.into_iter() {
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
        assert_eq!(false, super::contains(&[], 1));
    }

    #[test]
    fn return_false_when_items_234_dont_contain_1() {
        assert_eq!(false, super::contains(&[2, 3, 4], 1));
    }

    #[test]
    fn return_true_when_items_3210_contain_1() {
        assert!(super::contains(&[3, 2, 1, 0], 1));
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
        assert_eq!(None, super::min(&[]));
    }

    #[test]
    fn return_some_1_when_items_are_1() {
        assert_eq!(Some(1), super::min(&[1]));
    }

    #[test]
    fn return_some_1_when_items_are_23145() {
        assert_eq!(Some(1), super::min(&[2, 3, 1, 4, 5]));
    }
}

use std::ops::AddAssign;
use num::traits::Zero;
use num_traits::ToPrimitive;

fn sum<T>(items: &[T]) -> T
where T: Copy + AddAssign + Zero {
    let mut result: T = Zero::zero();

    for item in items.into_iter() {
        result.add_assign(*item);
    }

    result
}

#[cfg(test)]
mod sum_should {
    #[test]
    fn return_0_when_empty_items() {
        assert_eq!(0, super::sum(&[]));
    }

    #[test]
    fn return_10_when_items_are_1234() {
        assert_eq!(10, super::sum(&[1, 2, 3, 4]));
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
        assert_eq!(1, super::prod(&[]));
    }

    #[test]
    fn return_24_when_items_are_1234() {
        assert_eq!(24, super::prod(&[1, 2, 3, 4]));
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
        assert_eq!(None, super::average(&[]));
    }

    #[test]
    fn return_some_3_when_items_are_12345() {
        assert_eq!(Some(3.0), super::average(&[1.0, 2.0, 3.0, 4.0, 5.0]));
    }
}

mod md5;

fn binary_search(items: &[i32], value: i32) -> Option<usize> {
    let mut left = 0usize;
    let mut right = items.len();

    loop {
        if left == right {
            return None
        }

        let middle = left + (right - left)/2;
        if value == items[middle] {
            return Some(middle)
        }

        if value < items[middle] {
            right = middle;
        } else {
            left = middle + 1;
        }
    }
}

#[cfg(test)]
mod binary_search_should {
    use crate::binary_search;

    #[test]
    fn return_none_when_empty_items() {
        let actual = binary_search(&[], 7);

        assert_eq!(None, actual);
    }

    #[test]
    fn return_some_0_when_items_are_7_and_value_is_7() {
        let actual = binary_search(&[7], 7);

        assert_eq!(Some(0), actual);
    }

    #[test]
    fn return_none_when_items_are_7_and_value_is_4() {
        let actual = binary_search(&[7], 4);

        assert_eq!(None, actual);
    }

    #[test]
    fn return_some_3_when_items_are_13579_and_value_is_7() {
        let actual = binary_search(&[1, 3, 5, 7, 9], 7);

        assert_eq!(Some(3), actual);
    }

    #[test]
    fn return_none_when_items_are_13579_and_value_is_4() {
        let actual = binary_search(&[1, 3, 5, 7, 9], 4);

        assert_eq!(None, actual);
    }
}

fn calculate_t() -> [u32; 65] {
    let mut result = [0; 65];

    for i in 0..65 {
        let f = i.to_f64().unwrap();
        result[i] = (f.sin().abs() * 4294967296.0).to_u32().unwrap();
    }

    result
}

fn main() {
    let empty = [0; 0];
    let items = [0, 12, 6, 18, 3, 4];

    let empty_float = [0.0; 0];
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

    print!("T");
    for t in calculate_t() {
        print!(" {:#10x}", t);
    }
    println!();

    let (a, b, c, d) = md5::md5(&vec![]);
    println!("md5([]) = {:08x}-{:08x}-{:08x}-{:08x}", a, b, c, d);

    let (a, b, c, d) = md5::md5(&[0]);
    println!("md5([0]) = {:08x}-{:08x}-{:08x}-{:08x}", a, b, c, d);

    let (a, b, c, d) = md5::md5(&[1]);
    println!("md5([1]) = {:08x}-{:08x}-{:08x}-{:08x}", a, b, c, d);




    let ordered_items = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    println!("binary_search({:?}, {}) = {:?}", ordered_items, 11, binary_search(&ordered_items, 11));
    println!("binary_search({:?}, {}) = {:?}", ordered_items, 12, binary_search(&ordered_items, 12));
}
