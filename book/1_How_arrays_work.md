# How arryas work

Arrays are one of most known data structures. Moreover they are simple enough. So, arrays are frequently used as a base data structure for different algorithms.

We won't discuss how arrays are constructed, it's the topic of the future section. Now we will discuss how arrays work.

First of all, an array is a set of one-typed items.

```rust
let items = vec![3, 1, 4, 1, 5, 9, 2];
```

The variable `items` has the type `[i32, 7]` that means 7 elements of the `i32` type. The word *vec* in the code above is the abbreviation of *vector* that is the alias of the *array*.

What can we do with arrays?

Firstly, we can detect the length (the count of elements) of array with the help of the `len()` method.

```rust
println!{"{}", items.len()}; // => 7
```

Secondly, we can get elements by their non-negative index.

```rust
println!("{}", items[2]); // => 4
```

As in the C and many other inheritors of C, first element of an array has the index 0.

```rust
println!("{}", items[0]); // => 3
```

Thirdly, we can set elements by index, but the array should be mutable.

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

In languages such C, Java, or C# the function will return -1 (non-valid index value) to show that the item hasn't found. But Rust has special type for this — `Option`. Our function will return `None`, if the item hasn't found, or `Some` index, if it has found.

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

Here we see most popular method to work with arrays since good old C times. We have the `for` loop with the variable `i`, that we use to access to elements.

But if we don't need the index, we can iterate though the elements using an *iterator*.

Let compare `search` and `contains` functions. The `contains` detects if the array has specified element — yes or no. Because we don't need the index, we can skip `i` variable and use values from the array directly.

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

As you have possibly noticed, inside the `for` loop we use the *star* operator to get access to elements. It's because the iterator returns references to elements instead of elements themselves.

If you're not familiary with references you can skip theese details now. We will return to refernces in the Chapter 5.

What we should understand — iterators hide details of implementation. We can use them with all collection types like lists, sets, and arrays of course.

## Finding minimum of array

We have seen two algorithms that iterate through entrire array. But sometimes we need proceed *specific cases*.

For example we can specificly proceed a first or a last element of an array. Implementing the `min` function we compare each next item with the current minimum value, but we haven't this value for the first element of array.

If the array has only one element then this element becomes the result of the `min` function.

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

We store first element into the `result` variable and then proceed all remaining items as usually.

Chaning the only character (`<` to `>`) we can turn `min` to `max` function.

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
