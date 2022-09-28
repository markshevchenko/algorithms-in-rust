# Arrays as data structures

## How arryas work

Arrays are one of most known data structures. Moreover they are simple enough. So, arrays are frequently used as a base data structure for different algorithms.

We won't discuss how arrays are constructed, it's the topic of the future section. Now we will discuss how arrays work.

First of all, an array is a set of one-typed items.

```rust
let items = vec![3, 1, 4, 1, 5, 9, 2];
```

The variable `items` has the type `[i32, 7]` that means 7 elements of the `i32` type. The word *vec* in the code above is the abbreviation of *vector* that is the alias of the *array*.

What can we do with arrays?

Firstly, we can detect the length (the count of items) of array with the help of the `len()` method.

```rust
println!{"{}", items.len()}; // => 7
```

Secondly, we can get the item by its non-negative index.

```rust
println!("{}", items[2]); // => 4
```

As in the C and many other inheritors of C, the first item of the array has the index 0.

```rust
println!("{}", items[0]); // => 3
```

Thirdly, we can set the item by its index, but the array should be mutable.

```rust
let mut items = vec![2, 7, 1, 8, 2, 8, 1, 8, 2, 8];

items[0] = 1;
items[1] = 6;

println!("{:?}", items); // => [1, 6, 1, 8, 2, 8, 1, 8, 2, 8]
```

Theese three operations are all that we need.

So, let see some simple algorithms that work for arrays.

## Simple search in array

The `search` function looks for the specified item in the array. If the item has found, the function should return the index of the found item.

In languages such C, Java, or C# the function will return -1 (non-valid index value) to show that the item hasn't found. But Rust has the special type `Option` for this. Our function will return `None`, if the item hasn't found, or `Some` index, if it has found.

```rust
fn search(items: &[i32], value: i32) -> Option<usize> {
    for i in 0..items.len() {
        if items[i] == value {
            return Some(i)
        }
    }

    None
}
```

The construction `i in 0..items.len()` means that the `i` variable will get all values from 0 till `items.len()` excluding `items.len()`. So, if the array has 7 elements, `i` will get values 0, 1, 2, 3, 4, 5, and 6, but not 7!

Here we see most popular method to work with arrays since good old C times. We have the `for` loop with the variable `i`, that we use to access to items.

But if we don't need the index, we can iterate though the items using an *iterator*.

Let's compare `search` and `contains` functions. The `contains` detects if the array has the specified vlaue — yes or no. Because we don't need the index, we can skip `i` variable and check items of the array directly.

```rust
fn contains(items: &[i32], value: i32) -> bool {
    for item in items.into_iter() {
        if *item == value {
            return true
        }
    }

    false
}
```

As you have possibly noticed, we use the *star* operator to get access to items inside the `for` loop. It's because the iterator returns references to items instead of items themselves.

If you're not familiar with references you can skip theese details now. We will return to the topic in the Chapter 5.

What we should understand — iterators hide details of implementation. We can use them with all collection types like lists, sets, and arrays of course.

## Finding minimum of array

We have seen two algorithms that iterate through entrire array. But sometimes we need proceed *specific cases*.

For example we can specificly proceed a first or a last item of an array. Implementing the `min` function we compare each next item with the current minimum value, but we haven't this value for the first item of the array.

If the array has only one item, it becomes the result of the `min` function.

So we can write something like this.

```rust
fn min(items: &[i32]) -> Option<i32> {
    if items.is_empty() {
        None
    }

    let mut result = items[0];
    for i in 1..items.len() {
        if items[i] < result {
            result = items[i]
        }
    }

    Some(result)
}
```

We store the first item in the `result` variable and then proceed all remaining items as usually.

Chaning the only character (`<` to `>`) we can turn `min` function to `max`.

```rust
fn max(items: &[i32]) -> Option<i32> {
    if items.is_empty() {
        None
    }

    let mut result = items[0];
    for i in 1..items.len() {
        if items[i] > result {
            result = items[i]
        }
    }

    Some(result)
}
```

## Finding sum and product of items

Unlike `min` function, `sum` and `product` are always have a result. It seems surpising because what can be the sum of the empty array?

Let $s_n$ is the sum of numbers $a_1$, $a_2$, ..., $a_n$.

$$
s_n = a_1 + a_2 + \cdots + a_n
$$

The sum of numbers $a_1$, ..., $a_{n + 1}$ will be:

$$
s_{n + 1} = a_1 + a_2 + \cdots + a_n + a_{n + 1} = s_n + a_{n + 1}
$$

It means that:

$$
s_n = s_{n + 1} - a_{n + 1}
$$

This formula helps us to find marginal values $s_1$ and $s_0$.

$$
\displaylines{
s_1 = a_1 \\
s_0 = 0
}
$$

Thinking same way we can find values $p_1$ and $p_0$ where $p_n$ is the product of $n$ numbers.

$$
\displaylines{
p_1 = a_1 \\
p_0 = 1
}
$$

Now we can calculate the sum and the product of array items.

```rust
fn sum(items: &[i32]) -> i32 {
    let mut result = 0;

    for item in items.iter() {
        result += *item
    }

    result
}

fn prod(items: &[i32]) -> i32 {
    let mut result = 1;

    for item in items.iter() {
        result *= *item
    }

    result
}
```

Rust is the strongly typed language, so we need to have different functions to summarize integer and float numbers.

Fortunately, Rust supports generic types and special traits to summarize everything that can be added.

First we need to append crates `num` and `num-traits` to the **Cargo.toml** file.

```ini
[dependencies]
num = "0.4.0"
num-traits = "0.2.14"
```

Then we should use traits `AddAssing` and `Zero` to make generic version of the `sum` function.

```rust
use std::ops::AddAssign;
use num::traits::Zero;

fn sum<T>(items: &[T]) -> T
where T: Copy + AddAssign + Zero {
    let mut result: T = Zero::zero();

    for item in items.iter() {
        result.add_assign(*item);
    }

    result
}
```

The `product` function can be coded the same way, but you need to use `MulAssign` and `One` traits.

## Finding arithmetic mean of items

Like `min` and `max` functions the `average` can't handle empty arrays, so it should return an optional value.

We can simplify the code using the `sum` function that we have made already.

```rust
fn average(items: &[f64]) -> Option<f64> {
    if items.is_empty() {
        None
    } else {
        Some(sum(&items) / items.len() as f64)
    }
}
```

We need to cast the count of elements `items.len()` from the type `usize` to the type `f64`, because we can't divide a float value by an integer value. Remember: Rust is the strongly typed language.

Otherwise the code is simple enough.

## Finding MD5 checksum of byte array

Let's imagine that you've downloaded a very big file from the internet. You want check if the file hasn't corrupted during the download. How can you check it?

One of the simplest and fastest way is the comparing of *checksums* or *signatures*. Checksums are short numbers simple to compare. They are calculated by mixing all bytes of the source file.

Author of the file can calcualte its signature and publish it together with the link. After the downloading you also calculate your signature and compare it with the original value.

Equality of signatures mean equality of files. Or maybe not.

Because signatures are more shorter than source files sometimes they can be equal even if files are different. To reduce the probability of collisions we can use enough long signatures, for example 128-bit instead of 32-bit. Also we need enough mixing algorithm to avoid cases when `signature("abc")` equals to `signature("bac")` or `signature("cab")`.

Nowdays MD5 considered not so reliable method. But it's well-known and it's enough simple to learn how to implement such kind of algorithms.

### RFC-1321

The MD5 algorightm has been described in the [RFC-1321](https://www.ietf.org/rfc/rfc1321.txt). Although the document has title "Request For Comments" actually it's a kind of the standard having reference implementation of the algorithm on C program language.

So we'll can validate our code.

### Digest

MD5 signature (or *digest*) is the 128 bits value. We can represent it in different forms. Inside the algorithm the signature is stored as four 32 bits unsigned values. We'll call them `a`, `b`, `c`, and `d`.

Their initial values are (lowest byte first):

| Variable | Initial Value |
|:---------|--------------:|
|   `a`    | `01 23 45 67` |
|   `b`    | `08 ab cd ef` |
|   `c`    | `fe dc ba 98` |
|   `d`    | `76 54 32 10` |

Inside most modern computers numbers are stored in reverse order, highest byte first. So we need reverse bytes while initializing.

```rust
struct Digest {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}

pub fn md5(bytes: &[u8]) -> Digest {
    let digest = Digest {
        a: 0x67452301,
        b: 0xefcdab89,
        c: 0x98badcfe,
        d: 0x10325476,
    };

    digest
}
```

### Mixing functions

The MD5 algorithm mixes data with help of four fuctions called `f`, `g`, `h`, and `i`. All of them have three 32-bit parameters and mix them into the one 32-bit value.

```rust
fn f(x: u32, y: u32, z: u32) -> u32 {
    x & y | !x & z
}

fn g(x: u32, y: u32, z: u32) -> u32 {
    x & z | y & !z
}

fn h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

fn i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}
```

### Mixing secret

To make a mixing a bit more unpredictable, the MD5 algorithm uses special table of values, that called `T` in the RFC-1321. This table contains 64 unsigned 32-bit numbers. For each `i` from 1 to 64 `T[i]` is the $|4294967296 \times \sin i|$. Few first values of `T` are 0xd76aa478, 0xe8c7b756, 0x242070db, and 0xc1bdceee.

The reference C implementation doesn't have this table, it uses all values directly instead. We'll do the same.

### Chunks

The algorithm splits source array into 64-byte blocks or *chunks*. Every chunk is proceeded separately in four steps.

1. The 64-byte array is converted to array of unsinged 32-bit integers.
2. Current digest is stored to a temporary variable.
3. The integer array is mixed with digest.
4. The digest from the temporary variable is added to the new digest value.

### Converting bytes to unsigned integers

```rust
fn to_u32(byte1: u8, byte2: u8, byte3: u8, byte4: u8) -> u32 {
    byte1 as u32 + ((byte2 as u32) << 8) + ((byte3 as u32) << 16) + ((byte4 as u32) << 24)
}

fn convert_bytes_to_words(bytes: &[u8]) -> Vec<u32> {
    const U32_SIZE: usize = std::mem::size_of::<u32>();
    debug_assert!(bytes.len() % U32_SIZE == 0);
    let result_length = bytes.len() / U32_SIZE;

    let mut result = Vec::with_capacity(result_length);
    for i in 0..result_length {
        result.push(to_u32(bytes[U32_SIZE * i], bytes[U32_SIZE * i + 1],
                        bytes[U32_SIZE * i + 2], bytes[U32_SIZE * i + 3]));
    }

    result
}
```

Firstly we've implemented the function `to_u32` to convert four bytes to `u32` value. It gets parameters from `byte1` to `byte4`, and let's say their values are 0x11, 0x22, 0x33, and 0x44.

The operator `byte1 as u32` converts 0x11 to 0x00000011, `byte2 as u32` converts 0x22 to 0x00000022, and so on.

`((byte2 as u32) << 8)` shifts 0x00000022 left on eight binary positions that exactly equal to two hexadecimal positions, so 0x00000022 becomes 0x00002200. Following the same reasoning `((byte3 as u32) << 16)` becomes 0x00330000, and `((byte4 as u32) << 24)` becomes 0x44000000. After the addition all four values we will give 0x44332211.

Then we've made the function `convert_bytes_to_words` that fills the array of `u32` with converted values.
Due to the size of `u32` is exactly four bytes (`U32_SIZE` equals to 4) we have some limits, but also we can use some tricks.

Finally we convert every four bytes of source array to one unsigned integer value and return the array of integers.

### Cloning current value of the digest

There are some ways how we can store the value of digest. The simplest and enough obvious is to make `Digest` *cloneable*. The Rust compiler can do it for us, because the strucutre is primitive.

```rust
#[derive(Clone)]
struct Digest {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: u32,
}
```

Now we can call the `clone()` method to make a copy of the structure.

### Mixing values

## Binary search in array