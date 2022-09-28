use std::mem::{transmute};
use std::ops::AddAssign;

// MD5 algorithm has four auxiliary functions — f, g, h, and i, that mix three u32 words.
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
        assert_eq!(0x00000000, super::f(0x00000000, 0x00000000, 0x00000000));
        assert_eq!(0xffffffff, super::f(0x00000000, 0x00000000, 0xffffffff));
        assert_eq!(0x00000000, super::f(0x00000000, 0xffffffff, 0x00000000));
        assert_eq!(0xffffffff, super::f(0x00000000, 0xffffffff, 0xffffffff));
        assert_eq!(0x00000000, super::f(0xffffffff, 0x00000000, 0x00000000));
        assert_eq!(0x00000000, super::f(0xffffffff, 0x00000000, 0xffffffff));
        assert_eq!(0xffffffff, super::f(0xffffffff, 0xffffffff, 0x00000000));
        assert_eq!(0xffffffff, super::f(0xffffffff, 0xffffffff, 0xffffffff));
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
        assert_eq!(0x00000000, super::g(0x00000000, 0x00000000, 0x00000000));
        assert_eq!(0x00000000, super::g(0x00000000, 0x00000000, 0xffffffff));
        assert_eq!(0xffffffff, super::g(0x00000000, 0xffffffff, 0x00000000));
        assert_eq!(0x00000000, super::g(0x00000000, 0xffffffff, 0xffffffff));
        assert_eq!(0x00000000, super::g(0xffffffff, 0x00000000, 0x00000000));
        assert_eq!(0xffffffff, super::g(0xffffffff, 0x00000000, 0xffffffff));
        assert_eq!(0xffffffff, super::g(0xffffffff, 0xffffffff, 0x00000000));
        assert_eq!(0xffffffff, super::g(0xffffffff, 0xffffffff, 0xffffffff));
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
        assert_eq!(0x00000000, super::h(0x00000000, 0x00000000, 0x00000000));
        assert_eq!(0xffffffff, super::h(0x00000000, 0x00000000, 0xffffffff));
        assert_eq!(0xffffffff, super::h(0x00000000, 0xffffffff, 0x00000000));
        assert_eq!(0x00000000, super::h(0x00000000, 0xffffffff, 0xffffffff));
        assert_eq!(0xffffffff, super::h(0xffffffff, 0x00000000, 0x00000000));
        assert_eq!(0x00000000, super::h(0xffffffff, 0x00000000, 0xffffffff));
        assert_eq!(0x00000000, super::h(0xffffffff, 0xffffffff, 0x00000000));
        assert_eq!(0xffffffff, super::h(0xffffffff, 0xffffffff, 0xffffffff));
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
        assert_eq!(0xffffffff, super::i(0x00000000, 0x00000000, 0x00000000));
        assert_eq!(0x00000000, super::i(0x00000000, 0x00000000, 0xffffffff));
        assert_eq!(0x00000000, super::i(0x00000000, 0xffffffff, 0x00000000));
        assert_eq!(0xffffffff, super::i(0x00000000, 0xffffffff, 0xffffffff));
        assert_eq!(0xffffffff, super::i(0xffffffff, 0x00000000, 0x00000000));
        assert_eq!(0xffffffff, super::i(0xffffffff, 0x00000000, 0xffffffff));
        assert_eq!(0x00000000, super::i(0xffffffff, 0xffffffff, 0x00000000));
        assert_eq!(0x00000000, super::i(0xffffffff, 0xffffffff, 0xffffffff));
    }
}

/// Contains A, B, C, and D 32-bit words used to compute message digest.
#[derive(Clone, PartialEq, Debug)]
struct Digest {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}

impl AddAssign for Digest {
    fn add_assign(&mut self, rhs: Self) {
        self.a = self.a.wrapping_add(rhs.a);
        self.b = self.b.wrapping_add(rhs.b);
        self.c = self.c.wrapping_add(rhs.c);
        self.d = self.d.wrapping_add(rhs.d);
    }
}

#[cfg(test)]
mod add_assign_should {
    #[test]
    fn set_12_14_16_18_when_abcd_is_1_2_3_4_and_rhs_is_11_12_13_14() {
        let mut digest = super::Digest { a: 1, b: 2, c: 3, d: 4 };

        digest += super::Digest { a: 11, b: 12, c: 13, d: 14 };

        assert_eq!(super::Digest { a: 12, b: 14, c: 16, d: 18}, digest);
    }
}

impl From<Digest> for (u32, u32, u32, u32) {
    fn from(digest: Digest) -> Self {
        (digest.a.swap_bytes(), digest.b.swap_bytes(), digest.c.swap_bytes(), digest.d.swap_bytes())
    }
}

#[cfg(test)]
mod from_should
{
    #[test]
    fn return_swapped_values_of_digest() {
        let actual = <(u32, u32, u32, u32)>::from(super::Digest { a: 1, b: 2, c: 3, d: 4 });

        assert_eq!((1u32 << 24, 2u32 << 24, 3u32 << 24, 4u32 << 24), actual);
    }
}

fn mix_words(_digest: &mut Digest, _x: &[u32]) {
    #[macro_export]
    macro_rules! mix {
        ($f: ident, $a: ident, $b: ident, $c: ident, $d: ident, $k: literal, $s: literal, $t_i: literal) => {
           _digest.$a = _digest.$b.wrapping_add(_digest.$a.wrapping_add($f(_digest.$b, _digest.$c, _digest.$d))
                                                          .wrapping_add(_x[$k])
                                                          .wrapping_add($t_i)
                                                          .rotate_left($s));
        };
    }

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
}

fn as_u32(byte1: u8, byte2: u8, byte3: u8, byte4: u8) -> u32 {
    byte1 as u32 + ((byte2 as u32) << 8) + ((byte3 as u32) << 16) + ((byte4 as u32) << 24)
}

fn convert_bytes_to_words(bytes: &[u8]) -> Vec<u32> {
    const U32_SIZE: usize = std::mem::size_of::<u32>();
    debug_assert!(bytes.len() % U32_SIZE == 0);

    let mut result = Vec::with_capacity(bytes.len() / U32_SIZE);
    for i in 0..bytes.len() / U32_SIZE {
        result.push(as_u32(bytes[U32_SIZE * i], bytes[U32_SIZE * i + 1],
                        bytes[U32_SIZE * i + 2], bytes[U32_SIZE * i + 3]));
    }

    result
}

#[cfg(test)]
mod convert_bytes_to_word_should {
    use crate::md5::convert_bytes_to_words;

    #[test]
    fn return_04030201_when_bytes_are_01_02_03_04() {
        let actual = convert_bytes_to_words(&[0x01, 0x02, 0x03, 0x04]);

        assert_eq!(vec![0x04030201], actual);
    }
}

fn mix_chunk(digest: &mut Digest, bytes: &[u8]) {
    debug_assert!(bytes.len() == 64);
//    let x = unsafe { std::mem::transmute_copy::<&[u8], &[u32]>(&bytes) };
    let x = convert_bytes_to_words(bytes);

    let previous_digest = digest.clone();

    mix_words(digest, &x);

    *digest += previous_digest;
}

pub fn md5(bytes: &[u8]) -> (u32, u32, u32, u32) {
    let mut digest = Digest {
        a: 0x67452301,
        b: 0xefcdab89,
        c: 0x98badcfe,
        d: 0x10325476,
    };

    for block in bytes.chunks_exact(64) {
        mix_chunk(&mut digest, block);
    }

    let bit_length = unsafe { transmute::<u64, [u8; 8]>(bytes.len() as u64 * 8) };
    let tail_block = bytes.chunks_exact(64).remainder();
    let tail_block_length = tail_block.len();

    if tail_block_length < 56 {
        let mut last_block = [0u8; 64];
        last_block[..tail_block_length].clone_from_slice(tail_block);
        last_block[tail_block_length] = 128;
        last_block[tail_block_length + 1..56].fill(0);
        last_block[56..].clone_from_slice(&bit_length);

        mix_chunk(&mut digest, &last_block);
    } else {
        let mut last_block = [0u8; 128];
        last_block[..tail_block_length].clone_from_slice(tail_block);
        last_block[tail_block_length] = 128;
        last_block[tail_block_length + 1..120].fill(0);
        last_block[120..].clone_from_slice(&bit_length);

        mix_chunk(&mut digest, &last_block[..64]);
        mix_chunk(&mut digest, &last_block[64..]);
    }

    digest.into()
}

#[cfg(test)]
mod md5_should {
    // https://www.ietf.org/rfc/rfc1321.txt
    // Appendix A.5 Test suite
    // MD5 ("") = d41d8cd98f00b204e9800998ecf8427e
    // MD5 ("a") = 0cc175b9c0f1b6a831c399e269772661
    // MD5 ("abc") = 900150983cd24fb0d6963f7d28e17f72
    // MD5 ("message digest") = f96b697d7cb7938d525a2f31aaf161d0
    // MD5 ("abcdefghijklmnopqrstuvwxyz") = c3fcd3d76192e4007dfb496cca67e13b
    // MD5 ("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789") = d174ab98d277d9f5a5611c2c9f419d9f
    // MD5 ("12345678901234567890123456789012345678901234567890123456789012345678901234567890") = 57edf4a22be3c955ac49da2e2107b67a
    #[test]
    fn return_d41d8cd9_8f00b204_e9800998_ecf8427e_when_empty_bytes() {
        assert_eq!((0xd41d8cd9, 0x8f00b204, 0xe9800998, 0xecf8427e),
                   super::md5(&"".as_bytes()));
    }

    #[test]
    fn return_0cc175b9_c0f1b6a8_31c399e2_69772661_when_a() {
        assert_eq!((0x0cc175b9, 0xc0f1b6a8, 0x31c399e2, 0x69772661),
                   super::md5(&"a".as_bytes()));
    }

    #[test]
    fn return_90015098_3cd24fb0_d6963f7d_28e17f72_when_abc() {
        assert_eq!((0x90015098, 0x3cd24fb0, 0xd6963f7d, 0x28e17f72),
                   super::md5(&"abc".as_bytes()));
    }

    #[test]
    fn return_f96b697d_7cb7938d_525a2f31_aaf161d0_when_message_digest() {
        assert_eq!((0xf96b697d, 0x7cb7938d, 0x525a2f31, 0xaaf161d0),
                   super::md5(&"message digest".as_bytes()));
    }

    #[test]
    fn return_c3fcd3d7_6192e400_7dfb496c_ca67e13b_when_abcdefghijklmnopqrstuvwxyz() {
        assert_eq!((0xc3fcd3d7, 0x6192e400, 0x7dfb496c, 0xca67e13b),
                   super::md5(&"abcdefghijklmnopqrstuvwxyz".as_bytes()));
    }

    #[test]
    #[allow(non_snake_case)]
    fn return_d174ab98_d277d9f5_a5611c2c_9f419d9f_when_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789() {
        assert_eq!((0xd174ab98, 0xd277d9f5, 0xa5611c2c, 0x9f419d9f),
                   super::md5(&"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".as_bytes()));
    }

    #[test]
    fn return_57edf4a2_2be3c955_ac49da2e_2107b67a_when_12345678901234567890123456789012345678901234567890123456789012345678901234567890() {
        assert_eq!((0x57edf4a2, 0x2be3c955, 0xac49da2e, 0x2107b67a),
                   super::md5(&"12345678901234567890123456789012345678901234567890123456789012345678901234567890".as_bytes()));
    }
}
