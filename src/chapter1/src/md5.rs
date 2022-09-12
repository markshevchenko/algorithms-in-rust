use std::mem::size_of;

static SIZE_OF_U32: usize = size_of::<u32>();

fn f(x: u32, y: u32, z: u32) -> u32 {
    x & y | !x & z
}

#[cfg(test)]
mod f_should {
    // x y z  x & y | !x & z
    // 0 0 0        0
    // 0 0 1        1
    // 0 1 0        0
    // 0 1 1        1
    // 1 0 0        0
    // 1 0 1        0
    // 1 1 0        1
    // 1 1 1        1
    #[test]
    fn match_truth_table() {
        assert_eq!(0b0, super::f(0b0, 0b0, 0b0));
        assert_eq!(0b1, super::f(0b0, 0b0, 0b1));
        assert_eq!(0b0, super::f(0b0, 0b1, 0b0));
        assert_eq!(0b1, super::f(0b0, 0b1, 0b1));
        assert_eq!(0b0, super::f(0b1, 0b0, 0b0));
        assert_eq!(0b0, super::f(0b1, 0b0, 0b1));
        assert_eq!(0b1, super::f(0b1, 0b1, 0b0));
        assert_eq!(0b1, super::f(0b1, 0b1, 0b1));
    }
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    x & z | y & !z
}

#[cfg(test)]
mod g_should {
    // x y z  x & z | y & !z
    // 0 0 0        0
    // 0 0 1        0
    // 0 1 0        1
    // 0 1 1        0
    // 1 0 0        0
    // 1 0 1        1
    // 1 1 0        1
    // 1 1 1        1
    #[test]
    fn match_truth_table() {
        assert_eq!(0b0, super::g(0b0, 0b0, 0b0));
        assert_eq!(0b0, super::g(0b0, 0b0, 0b1));
        assert_eq!(0b1, super::g(0b0, 0b1, 0b0));
        assert_eq!(0b0, super::g(0b0, 0b1, 0b1));
        assert_eq!(0b0, super::g(0b1, 0b0, 0b0));
        assert_eq!(0b1, super::g(0b1, 0b0, 0b1));
        assert_eq!(0b1, super::g(0b1, 0b1, 0b0));
        assert_eq!(0b1, super::g(0b1, 0b1, 0b1));
    }
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[cfg(test)]
mod h_should {
    // x y z  x ^ y ^ z
    // 0 0 0      0
    // 0 0 1      1
    // 0 1 0      1
    // 0 1 1      0
    // 1 0 0      1
    // 1 0 1      0
    // 1 1 0      0
    // 1 1 1      1
    #[test]
    fn match_truth_table() {
        assert_eq!(0b0, super::h(0b0, 0b0, 0b0));
        assert_eq!(0b1, super::h(0b0, 0b0, 0b1));
        assert_eq!(0b1, super::h(0b0, 0b1, 0b0));
        assert_eq!(0b0, super::h(0b0, 0b1, 0b1));
        assert_eq!(0b1, super::h(0b1, 0b0, 0b0));
        assert_eq!(0b0, super::h(0b1, 0b0, 0b1));
        assert_eq!(0b0, super::h(0b1, 0b1, 0b0));
        assert_eq!(0b1, super::h(0b1, 0b1, 0b1));
    }
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}

#[cfg(test)]
mod i_should {
    // x y z  y ^ (x | !z)
    // 0 0 0    1
    // 0 0 1    0
    // 0 1 0    0
    // 0 1 1    1
    // 1 0 0    1
    // 1 0 1    1
    // 1 1 0    0
    // 1 1 1    0
    #[test]
    fn match_truth_table() {
        assert_eq!(0b1, super::i(0b0, 0b0, 0b0) & 0b1);
        assert_eq!(0b0, super::i(0b0, 0b0, 0b1) & 0b1);
        assert_eq!(0b0, super::i(0b0, 0b1, 0b0) & 0b1);
        assert_eq!(0b1, super::i(0b0, 0b1, 0b1) & 0b1);
        assert_eq!(0b1, super::i(0b1, 0b0, 0b0) & 0b1);
        assert_eq!(0b1, super::i(0b1, 0b0, 0b1) & 0b1);
        assert_eq!(0b0, super::i(0b1, 0b1, 0b0) & 0b1);
        assert_eq!(0b0, super::i(0b1, 0b1, 0b1) & 0b1);
    }
}

fn bytes_as_u32(bytes: &[u8], offset: usize) -> u32 {
    debug_assert!(bytes.len() >= offset + SIZE_OF_U32);

    (bytes[offset] as u32)
        + (bytes[offset + 1] as u32).rotate_left(8)
        + (bytes[offset + 2] as u32).rotate_left(16)
        + (bytes[offset + 3] as u32).rotate_left(24)
}

#[cfg(test)]
mod bytes_as_u32_should {
    #[test]
    fn return_128_when_128_0_0_0() {
        assert_eq!(128, super::bytes_as_u32(&vec![128, 0, 0, 0], 0));
    }

    #[test]
    fn return_32768_when_0_128_0_0() {
        // 128 * 256
        assert_eq!(32768, super::bytes_as_u32(&vec![0, 128, 0, 0], 0));
    }

    #[test]
    fn return_8388608_when_0_0_128_0() {
        // 128 * 256 * 256
        assert_eq!(8388608, super::bytes_as_u32(&vec![0, 0, 128, 0], 0));
    }

    #[test]
    fn return_2147483648_when_0_0_0_128() {
        // 128 * 256 * 256 * 256
        assert_eq!(2147483648, super::bytes_as_u32(&vec![0, 0, 0, 128], 0));
    }
}

fn decode(input: &[u8], input_offset: usize, output: &mut [u32]) {
    for i in 0..output.len() {
        output[i] = bytes_as_u32(input, input_offset + SIZE_OF_U32 * i);
    }
}

fn mix_buffer(bytes: &[u8], offset: usize, buffer: (u32, u32, u32, u32)) -> (u32, u32, u32, u32) {
    let (mut a, mut b, mut c, mut d) = buffer;
    let mut x: [u32; 16] = [0; 16];

    #[macro_export]
    macro_rules! mix {
        ($f: ident, $a: ident, $b: ident, $c: ident, $d: ident, $k: literal, $s: literal, $t_i: literal) => {
           $a = $b.wrapping_add($a.wrapping_add($f($b, $c, $d))
                                  .wrapping_add(x[$k])
                                  .wrapping_add($t_i)
                                  .rotate_left($s));
        };
    }

    decode(bytes, offset, &mut x);

    let aa = a;
    let bb = b;
    let cc = c;
    let dd = d;

    mix!(f, a, b, c, d,  0,  7, 0xd76aa478);
    mix!(f, d, a, b, c,  1, 12, 0xe8c7b756);
    mix!(f, c, d, a, b,  2, 17, 0x242070db);
    mix!(f, b, c, d, a,  3, 22, 0xc1bdceee);

    mix!(f, a, b, c, d,  4,  7, 0xf57c0faf);
    mix!(f, d, a, b, c,  5, 12, 0x4787c62a);
    mix!(f, c, d, a, b,  6, 17, 0xa8304613);
    mix!(f, b, c, d, a,  7, 22, 0xfd469501);

    mix!(f, a, b, c, d,  8,  7, 0x698098d8);
    mix!(f, d, a, b, c,  9, 12, 0x8b44f7af);
    mix!(f, c, d, a, b, 10, 17, 0xffff5bb1);
    mix!(f, b, c, d, a, 11, 22, 0x895cd7be);

    mix!(f, a, b, c, d, 12,  7, 0x6b901122);
    mix!(f, d, a, b, c, 13, 12, 0xfd987193);
    mix!(f, c, d, a, b, 14, 17, 0xa679438e);
    mix!(f, b, c, d, a, 15, 22, 0x49b40821);

    mix!(g, a, b, c, d,  1,  5, 0xf61e2562);
    mix!(g, d, a, b, c,  6,  9, 0xc040b340);
    mix!(g, c, d, a, b, 11, 14, 0x265e5a51);
    mix!(g, b, c, d, a,  0, 20, 0xe9b6c7aa);

    mix!(g, a, b, c, d,  5,  5, 0xd62f105d);
    mix!(g, d, a, b, c, 10,  9, 0x02441453);
    mix!(g, c, d, a, b, 15, 14, 0xd8a1e681);
    mix!(g, b, c, d, a,  4, 20, 0xe7d3fbc8);

    mix!(g, a, b, c, d,  9,  5, 0x21e1cde6);
    mix!(g, d, a, b, c, 14,  9, 0xc33707d6);
    mix!(g, c, d, a, b,  3, 14, 0xf4d50d87);
    mix!(g, b, c, d, a,  8, 20, 0x455a14ed);

    mix!(g, a, b, c, d, 13,  5, 0xa9e3e905);
    mix!(g, d, a, b, c,  2,  9, 0xfcefa3f8);
    mix!(g, c, d, a, b,  7, 14, 0x676f02d9);
    mix!(g, b, c, d, a, 12, 20, 0x8d2a4c8a);

    mix!(h, a, b, c, d,  5,  4, 0xfffa3942);
    mix!(h, d, a, b, c,  8, 11, 0x8771f681);
    mix!(h, c, d, a, b, 11, 16, 0x6d9d6122);
    mix!(h, b, c, d, a, 14, 23, 0xfde5380c);

    mix!(h, a, b, c, d,  1,  4, 0xa4beea44);
    mix!(h, d, a, b, c,  4, 11, 0x4bdecfa9);
    mix!(h, c, d, a, b,  7, 16, 0xf6bb4b60);
    mix!(h, b, c, d, a, 10, 23, 0xbebfbc70);

    mix!(h, a, b, c, d, 13,  4, 0x289b7ec6);
    mix!(h, d, a, b, c,  0, 11, 0xeaa127fa);
    mix!(h, c, d, a, b,  3, 16, 0xd4ef3085);
    mix!(h, b, c, d, a,  6, 23, 0x04881d05);

    mix!(h, a, b, c, d,  9,  4, 0xd9d4d039);
    mix!(h, d, a, b, c, 12, 11, 0xe6db99e5);
    mix!(h, c, d, a, b, 15, 16, 0x1fa27cf8);
    mix!(h, b, c, d, a,  2, 23, 0xc4ac5665);

    mix!(i, a, b, c, d,  0,  6, 0xf4292244);
    mix!(i, d, a, b, c,  7, 10, 0x432aff97);
    mix!(i, c, d, a, b, 14, 15, 0xab9423a7);
    mix!(i, b, c, d, a,  5, 21, 0xfc93a039);

    mix!(i, a, b, c, d, 12,  6, 0x655b59c3);
    mix!(i, d, a, b, c,  3, 10, 0x8f0ccc92);
    mix!(i, c, d, a, b, 10, 15, 0xffeff47d);
    mix!(i, b, c, d, a,  1, 21, 0x85845dd1);

    mix!(i, a, b, c, d,  8,  6, 0x6fa87e4f);
    mix!(i, d, a, b, c, 15, 10, 0xfe2ce6e0);
    mix!(i, c, d, a, b,  6, 15, 0xa3014314);
    mix!(i, b, c, d, a, 13, 21, 0x4e0811a1);

    mix!(i, a, b, c, d,  4,  6, 0xf7537e82);
    mix!(i, d, a, b, c, 11, 10, 0xbd3af235);
    mix!(i, c, d, a, b,  2, 15, 0x2ad7d2bb);
    mix!(i, b, c, d, a,  9, 21, 0xeb86d391);

    a = a.wrapping_add(aa);
    b = b.wrapping_add(bb);
    c = c.wrapping_add(cc);
    d = d.wrapping_add(dd);

    (a, b, c, d)
}

pub fn md5(bytes: &[u8]) -> (u32, u32, u32, u32) {
    let mut buffer = (0x67452301u32, 0xefcdab89u32, 0x98badcfeu32, 0x10325476u32);

    if bytes.len() < 56 {
        let mut block : [u8; 64] = [0; 64];
        for i in 0..bytes.len() {
            block[i] = bytes[i]
        }

        block[bytes.len()] = 0x80;

        for i in bytes.len() + 1..block.len() {
            block[i] = 0x00;
        }

        let bit_len = bytes.len() * 8;
        block[56] = (bit_len) as u8;
        block[57] = (bit_len >> 8) as u8;
        block[58] = (bit_len >> 16) as u8;
        block[59] = (bit_len >> 24) as u8;
        block[60] = (bit_len >> 32) as u8;
        block[61] = (bit_len >> 40) as u8;
        block[62] = (bit_len >> 48) as u8;
        block[63] = (bit_len >> 56) as u8;

        buffer = mix_buffer(&block, 0, buffer);
    }

    (buffer.0.swap_bytes(), buffer.1.swap_bytes(), buffer.2.swap_bytes(), buffer.3.swap_bytes())
}

#[cfg(test)]
mod md5_should {
    // google "md5 of empty string"
    #[test]
    fn return_d41d8cd9_8f00b204_e9800998_ecf8427e_when_empty_bytes() {
        assert_eq!((0xd41d8cd9, 0x8f00b204, 0xe9800998, 0xecf8427e), super::md5(&vec![]));
    }
}
