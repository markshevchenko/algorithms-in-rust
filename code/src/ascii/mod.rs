pub fn is_digit(c: u8) -> bool {
    // c >= b'0' && c <= b'9'
    c >= 48 && c <= 57
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
        // let next_digit = u32::from(s[i] - b'0');
        // accumulator = 10 * accumulator + next_digit;
        if accumulator > u32::MAX / 10 {
            return Some(accumulator);
        }

        let accumulator10 = accumulator * 10;
        
        let next_digit = u32::from(s[i] - b'0');
        if accumulator10 > u32::MAX - next_digit {
            return Some(accumulator);
        }
        
        accumulator = accumulator10 + next_digit;
        
        i += 1;
    }

    Some(accumulator)
}

pub fn caesar_encrypt(s: &[u8], shift: u8) -> Vec<u8> {
    let mut result = Vec::new();
    
    for c in s {
        let out_c = if is_lower(*c) {
            (*c - b'a' + shift) % 26 + b'a'
        } else if is_upper(*c) {
            (*c - b'A' + shift) % 26 + b'A'
        } else {
            *c
        };

        result.push(out_c)
    }

    result
}


pub fn caesar_decrypt(s: &[u8], shift: u8) -> Vec<u8> {
    let mut result = Vec::new();
    
    for c in s {
        let out_c = if is_lower(*c) {
            (*c - b'a' - shift) % 26 + b'a'
        } else if is_upper(*c) {
            (*c - b'A' - shift) % 26 + b'A'
        } else {
            *c
        };

        result.push(out_c)
    }

    result
}