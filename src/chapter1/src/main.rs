extern crate num_traits;

fn contains(items: &[i32], value: i32) -> bool {
    for i in 0..items.len() {
        if items[i] == value {
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

#[derive(PartialEq, Debug)]
struct User<'a> {
    pub id: u32,
    pub login: &'a str,
}

fn search_user_by_id<'a, 'b>(users: &'a [User<'b>], id: u32) -> Option<&'a User<'b>> {
    for i in 0..users.len() {
        if users[i].id == id {
            return Some(&users[i])
        }
    }

    return None
}

#[cfg(test)]
mod search_user_by_id_should {
    use super::{search_user_by_id, User};

    #[test]
    fn return_none_when_missed_id() {
        let users = vec![
            User { id: 1, login: "John Smith" },
            User { id: 2, login: "Jack Brown" },
            User { id: 3, login: "Ronald Tolkien" }];

        let non_existing_id = 4;

        assert!(search_user_by_id(&users, non_existing_id).is_none());
    }

    #[test]
    fn return_login_jack_brown_when_id_is_2() {
        let users = vec![
            User { id: 1, login: "John Smith" },
            User { id: 2, login: "Jack Brown" },
            User { id: 3, login: "Ronald Tolkien" }];

        let jack_brown_id = 2;

        assert_eq!("Jack Brown", search_user_by_id(&users, jack_brown_id).unwrap().login);
    }
}

fn search_user_by_login<'a, 'b, 'c>(users: &'a [User<'b>], login: &'c str) -> Option<&'a User<'b>> {
    for i in 0..users.len() {
        if users[i].login == login {
            return Some(&users[i])
        }
    }

    return None
}

#[cfg(test)]
mod search_user_by_login_should {
    use super::{search_user_by_login, User};

    #[test]
    fn return_none_when_missed_login() {
        let users = vec![
            User { id: 1, login: "John Smith" },
            User { id: 2, login: "Jack Brown" },
            User { id: 3, login: "Ronald Tolkien" }];

        assert!(search_user_by_login(&users, &"John Doe").is_none());
    }

    #[test]
    fn return_id_2_when_login_is_jack_brown() {
        let users = vec![
            User { id: 1, login: "John Smith" },
            User { id: 2, login: "Jack Brown" },
            User { id: 3, login: "Ronald Tolkien" }];

        assert_eq!(2, search_user_by_login(&users, &"Jack Brown").unwrap().id);
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

    let users = vec![User { id: 1, login: &"foo" },
        User { id: 2, login: &"boo" }];

    println!("contains({:?}, {}) = {}", items, 6, contains(&items, 6));
    println!("contains({:?}, {}) = {}", items, 5, contains(&items, 5));

    println!("search_user_by_id({:?}, {}) = {:?}", users, 3, search_user_by_id(&users, 3));
    println!("search_user_by_id({:?}, {}) = {:?}", users, 1, search_user_by_id(&users, 1));

    println!("search_user_by_login({:?}, \"{}\") = {:?}", users, "baz", search_user_by_login(&users, "baz"));
    println!("search_user_by_login({:?}, \"{}\") = {:?}", users, "foo", search_user_by_login(&users, "foo"));

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
