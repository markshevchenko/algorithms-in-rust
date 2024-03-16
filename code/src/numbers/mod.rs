pub fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}


pub fn min2<T>(a: T, b: T) -> T where T: Ord {
    if a < b {
        a
    } else {
        b
    }
}


pub fn min3<T>(a: T, b: T, c: T) -> T where T: Ord {
    min2(a, min2(b, c))
}


pub fn partial_min2<T>(a: T, b: T) -> Option<T> where T: PartialOrd {
    match a.partial_cmp(&b) {
        Some(std::cmp::Ordering::Less) => Some(a),
        None => None,
        _ => Some(b),
    }
}