pub fn percent(n: i32, percent: i32) -> i32 {
    n * percent / 100
}

pub fn min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}


// pub fn min2<T>(a: T, b: T) -> T where T: Ord {
//     if a < b {
//         a
//     } else {
//         b
//     }
// }


pub fn min2<T>(a: T, b: T) -> T where T: Ord {
    use std::cmp::Ordering;

    match a.cmp(&b) {
        Ordering::Less => a,
        Ordering::Equal => b,
        Ordering::Greater => b,
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


pub fn gdc(a: u32, b: u32) -> u32 {
    let (mut a, mut b) =
        if a > b {
            (a, b)
        } else {
            (b, a)
        };

    while b != 0 {
        (a, b) = (b, a % b);
    }

    a
}

pub fn sqrt(x: f64) -> f64 {
    if x < 0.0 {
        return f64::NAN;
    }

    let error = x * f64::EPSILON;
    let mut a = x / 2.0;

    while (a * a - x).abs() > error {
        a = (a + x / a) / 2.0;
    }

    a
}