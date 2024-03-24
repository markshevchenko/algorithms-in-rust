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