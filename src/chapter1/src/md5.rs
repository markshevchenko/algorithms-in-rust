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

struct MD5Data {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
    pub x: [u32; 16]
}

fn decode_bytes(bytes: &[u8], x: &mut [u32]) {
    if x.len() > usize::MAX/4 {
        panic!("Too long u32 array to convert from byte array.")
    }

    if bytes.len() != x.len() * 4 {
        panic!("Array of u32 must be exactly four times shorter than array of u8.")
    }

    let mut offset = 0;
    for i in 0..x.len() {
        let byte0 = bytes[offset] as u32;
        let byte1 = bytes[offset + 1] as u32;
        let byte2 = bytes[offset + 2] as u32;
        let byte3 = bytes[offset + 3] as u32;

        x[i] = byte0 + (byte1 << 8) + (byte2 << 16) + (byte3 << 24);
        offset += 4;
    }
}

#[cfg(test)]
mod decode_bytes_should {
    use crate::md5::decode_bytes;

    #[test]
    #[should_panic(expected = "Array of u32 must be exactly four times shorter than array of u8")]
    fn panic_when_bytes_remain() {
        let bytes: Vec<u8> = vec![128, 0, 0];
        let mut x = [0; 1];

        decode_bytes(&bytes, &mut x);
    }

    #[test]
    fn set_128_when_128_0_0_0() {
        let bytes: Vec<u8> = vec![128, 0, 0, 0];
        let mut x = [0; 1];

        decode_bytes(&bytes, &mut x);

        assert_eq!(128, x[0]);
    }

    #[test]
    fn set_32768_when_0_128_0_0() {
        let bytes: Vec<u8> = vec![0, 128, 0, 0];
        let mut x = [0; 1];

        decode_bytes(&bytes, &mut x);

        // 128 * 256
        assert_eq!(32768, x[0]);
    }

    #[test]
    fn set_8388608_when_0_0_128_0() {
        let bytes: Vec<u8> = vec![0, 0, 128, 0];
        let mut x = [0; 1];

        decode_bytes(&bytes, &mut x);

        // 128 * 256 * 256
        assert_eq!(8388608, x[0]);
    }

    #[test]
    fn set_2147483648_when_0_0_0_128() {
        let bytes: Vec<u8> = vec![0, 0, 0, 128];
        let mut x = [0; 1];

        decode_bytes(&bytes, &mut x);

        // 128 * 256 * 256 * 256
        assert_eq!(2147483648, x[0]);
    }
}

fn get_abcd(md5data: &MD5Data) -> (u32, u32, u32, u32) {
    (md5data.a, md5data.b, md5data.c, md5data.d)
}

#[cfg(test)]
mod get_abcd_should {
    #[test]
    fn return_1_2_3_4_when_md5data_contains_1_2_3_4() {
        let md5data = super::MD5Data { a: 1, b: 2, c: 3, d: 4, x: [0; 16] };

        let actual = super::get_abcd(&md5data);

        assert_eq!((1, 2, 3, 4), actual);
    }
}

fn wrapping_add_abcd(md5data: &mut MD5Data, buffer: (u32, u32, u32, u32)) {
    md5data.a = md5data.a.wrapping_add(buffer.0);
    md5data.b = md5data.b.wrapping_add(buffer.1);
    md5data.c = md5data.c.wrapping_add(buffer.2);
    md5data.d = md5data.d.wrapping_add(buffer.3);
}

#[cfg(test)]
mod wrapping_add_abcd_should {
    #[test]
    fn set_12_14_16_18_when_md5data_is_1_2_3_4_and_buffer_is_11_12_13_14() {
        let mut md5data = super::MD5Data { a: 1, b: 2, c: 3, d: 4, x: [0; 16] };

        super::wrapping_add_abcd(&mut md5data, (11, 12, 13, 14));

        assert_eq!(12, md5data.a);
        assert_eq!(14, md5data.b);
        assert_eq!(16, md5data.c);
        assert_eq!(18, md5data.d);
    }
}

fn get_result(md5data: &MD5Data) -> (u32, u32, u32, u32) {
    (md5data.a.swap_bytes(),
     md5data.b.swap_bytes(),
     md5data.c.swap_bytes(),
     md5data.d.swap_bytes())
}

#[cfg(test)]
mod get_result_should
{
    #[test]
    fn return_swapped_values_of_abcd() {
        let md5data = super::MD5Data { a: 1, b: 2, c: 3, d: 4, x: [0; 16] };

        let actual = super::get_result(&md5data);

        assert_eq!(1u32.swap_bytes(), actual.0);
        assert_eq!(2u32.swap_bytes(), actual.1);
        assert_eq!(3u32.swap_bytes(), actual.2);
        assert_eq!(4u32.swap_bytes(), actual.3);
    }
}

fn u64_to_bytes(value: u64) -> [u8; 8] {
    // unsafe { std::mem::transmute(value) }
    [value as u8, (value >> 8) as u8, (value >> 16) as u8, (value >> 24) as u8,
        (value >> 32) as u8, (value >> 40) as u8, (value >> 48) as u8, (value >> 56) as u8]
}

#[cfg(test)]
mod u64_to_bytes_should {
    #[test]
    fn return_128_0_0_0_0_0_0_0_when_value_128() {
        let actual = super::u64_to_bytes(128);

        assert_eq!([128, 0, 0, 0, 0, 0, 0, 0], actual);
    }

    #[test]
    fn return_0_128_0_0_0_0_0_0_when_value_32768() {
        let actual = super::u64_to_bytes(32768);

        assert_eq!([0, 128, 0, 0, 0, 0, 0, 0], actual);
    }
}

fn mix_next_64_bytes(md5data: &mut MD5Data, bytes: &[u8]) {
    #[macro_export]
    macro_rules! mix2 {
        ($f: ident, $a: ident, $b: ident, $c: ident, $d: ident, $k: literal, $s: literal, $t_i: literal) => {
           md5data.$a = md5data.$b.wrapping_add(md5data.$a.wrapping_add($f(md5data.$b, md5data.$c, md5data.$d))
                                  .wrapping_add(md5data.x[$k])
                                  .wrapping_add($t_i)
                                  .rotate_left($s));
        };
    }

    decode_bytes(bytes, &mut md5data.x);
    let abcd = get_abcd(&md5data);

    mix2!(f, a, b, c, d,  0,  7, 0xd76aa478);
    mix2!(f, d, a, b, c,  1, 12, 0xe8c7b756);
    mix2!(f, c, d, a, b,  2, 17, 0x242070db);
    mix2!(f, b, c, d, a,  3, 22, 0xc1bdceee);

    mix2!(f, a, b, c, d,  4,  7, 0xf57c0faf);
    mix2!(f, d, a, b, c,  5, 12, 0x4787c62a);
    mix2!(f, c, d, a, b,  6, 17, 0xa8304613);
    mix2!(f, b, c, d, a,  7, 22, 0xfd469501);

    mix2!(f, a, b, c, d,  8,  7, 0x698098d8);
    mix2!(f, d, a, b, c,  9, 12, 0x8b44f7af);
    mix2!(f, c, d, a, b, 10, 17, 0xffff5bb1);
    mix2!(f, b, c, d, a, 11, 22, 0x895cd7be);

    mix2!(f, a, b, c, d, 12,  7, 0x6b901122);
    mix2!(f, d, a, b, c, 13, 12, 0xfd987193);
    mix2!(f, c, d, a, b, 14, 17, 0xa679438e);
    mix2!(f, b, c, d, a, 15, 22, 0x49b40821);

    mix2!(g, a, b, c, d,  1,  5, 0xf61e2562);
    mix2!(g, d, a, b, c,  6,  9, 0xc040b340);
    mix2!(g, c, d, a, b, 11, 14, 0x265e5a51);
    mix2!(g, b, c, d, a,  0, 20, 0xe9b6c7aa);

    mix2!(g, a, b, c, d,  5,  5, 0xd62f105d);
    mix2!(g, d, a, b, c, 10,  9, 0x02441453);
    mix2!(g, c, d, a, b, 15, 14, 0xd8a1e681);
    mix2!(g, b, c, d, a,  4, 20, 0xe7d3fbc8);

    mix2!(g, a, b, c, d,  9,  5, 0x21e1cde6);
    mix2!(g, d, a, b, c, 14,  9, 0xc33707d6);
    mix2!(g, c, d, a, b,  3, 14, 0xf4d50d87);
    mix2!(g, b, c, d, a,  8, 20, 0x455a14ed);

    mix2!(g, a, b, c, d, 13,  5, 0xa9e3e905);
    mix2!(g, d, a, b, c,  2,  9, 0xfcefa3f8);
    mix2!(g, c, d, a, b,  7, 14, 0x676f02d9);
    mix2!(g, b, c, d, a, 12, 20, 0x8d2a4c8a);

    mix2!(h, a, b, c, d,  5,  4, 0xfffa3942);
    mix2!(h, d, a, b, c,  8, 11, 0x8771f681);
    mix2!(h, c, d, a, b, 11, 16, 0x6d9d6122);
    mix2!(h, b, c, d, a, 14, 23, 0xfde5380c);

    mix2!(h, a, b, c, d,  1,  4, 0xa4beea44);
    mix2!(h, d, a, b, c,  4, 11, 0x4bdecfa9);
    mix2!(h, c, d, a, b,  7, 16, 0xf6bb4b60);
    mix2!(h, b, c, d, a, 10, 23, 0xbebfbc70);

    mix2!(h, a, b, c, d, 13,  4, 0x289b7ec6);
    mix2!(h, d, a, b, c,  0, 11, 0xeaa127fa);
    mix2!(h, c, d, a, b,  3, 16, 0xd4ef3085);
    mix2!(h, b, c, d, a,  6, 23, 0x04881d05);

    mix2!(h, a, b, c, d,  9,  4, 0xd9d4d039);
    mix2!(h, d, a, b, c, 12, 11, 0xe6db99e5);
    mix2!(h, c, d, a, b, 15, 16, 0x1fa27cf8);
    mix2!(h, b, c, d, a,  2, 23, 0xc4ac5665);

    mix2!(i, a, b, c, d,  0,  6, 0xf4292244);
    mix2!(i, d, a, b, c,  7, 10, 0x432aff97);
    mix2!(i, c, d, a, b, 14, 15, 0xab9423a7);
    mix2!(i, b, c, d, a,  5, 21, 0xfc93a039);

    mix2!(i, a, b, c, d, 12,  6, 0x655b59c3);
    mix2!(i, d, a, b, c,  3, 10, 0x8f0ccc92);
    mix2!(i, c, d, a, b, 10, 15, 0xffeff47d);
    mix2!(i, b, c, d, a,  1, 21, 0x85845dd1);

    mix2!(i, a, b, c, d,  8,  6, 0x6fa87e4f);
    mix2!(i, d, a, b, c, 15, 10, 0xfe2ce6e0);
    mix2!(i, c, d, a, b,  6, 15, 0xa3014314);
    mix2!(i, b, c, d, a, 13, 21, 0x4e0811a1);

    mix2!(i, a, b, c, d,  4,  6, 0xf7537e82);
    mix2!(i, d, a, b, c, 11, 10, 0xbd3af235);
    mix2!(i, c, d, a, b,  2, 15, 0x2ad7d2bb);
    mix2!(i, b, c, d, a,  9, 21, 0xeb86d391);

    wrapping_add_abcd(md5data, abcd);
}

pub fn md5(bytes: &[u8]) -> (u32, u32, u32, u32) {
    let mut md5data = MD5Data {
        a: 0x67452301u32,
        b: 0xefcdab89u32,
        c: 0x98badcfeu32,
        d: 0x10325476u32,
        x: [0; 16]
    };

    for block in bytes.chunks_exact(64) {
        mix_next_64_bytes(&mut md5data, block);
    }

    let bit_length = u64_to_bytes(bytes.len() as u64 * 8);
    let tail_block = bytes.chunks_exact(64).remainder();
    let tail_block_length = tail_block.len();

    if tail_block_length < 56 {
        let mut last_block = [0u8; 64];
        last_block[..tail_block_length].clone_from_slice(tail_block);
        last_block[tail_block_length] = 128;
        last_block[tail_block_length + 1..56].fill(0);
        last_block[56..].clone_from_slice(&bit_length);

        mix_next_64_bytes(&mut md5data, &last_block);
    } else {
        let mut last_block = [0u8; 128];
        last_block[..tail_block_length].clone_from_slice(tail_block);
        last_block[tail_block_length] = 128;
        last_block[tail_block_length + 1..120].fill(0);
        last_block[120..].clone_from_slice(&bit_length);

        mix_next_64_bytes(&mut md5data, &last_block[..64]);
        mix_next_64_bytes(&mut md5data, &last_block[64..]);
    }

    get_result(&md5data)
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
