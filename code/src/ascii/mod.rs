pub fn is_digit(c: u8) -> bool {
    c >= b'0' && c <= b'9'
}

pub fn is_lower(c: u8) -> bool {
    c >= b'a' && c <= b'z'
}

pub fn is_upper(c: u8) -> bool {
    c >= b'A' && c <= b'Z'
}

pub fn is_letter(c: u8) -> bool {
    is_lower(c) || is_upper(c)
}

pub fn to_lower(c: u8) -> u8 {
    if is_upper(c) {
        // c + (b'a' - b'A')
        c + 32
    } else {
        c
    }
}

pub fn to_upper(c: u8) -> u8 {
    if is_lower(c) {
        c - (b'a' - b'A')
    } else {
        c
    }
}

pub fn parse_u32(s: &[u8]) -> Option<u32> {
    if s.len() == 0 || !is_digit(s[0]) {
        return None;
    }

    let mut accumulator: u32 = 0;
    let mut i = 0;

    while i < s.len() && is_digit(s[i]) {
        accumulator = 10 * accumulator + u32::from(s[i] - b'0');
        i += 1;
    }

    Some(accumulator)
}