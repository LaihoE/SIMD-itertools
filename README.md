#  SIMD-accelerated iterators

[![crates.io](https://img.shields.io/crates/v/simd-itertools.svg)](https://crates.io/crates/simd-itertools)


### Unmatched flexibility 🤯

```Rust
let needles = [42, 52, 94];
arr.iter().any_simd(|x| needles.contains(x) || x > 156);
```
- Works by letting LLVM do the vectorization (may change in the future).
- Functions are made easy to paste into Godbolt for inspection.


Currently the following are implemented:

```find```
```filter```
```position```
```all```
```any```
```argmin/argmax```


### Tradeoffs
Every piece of software makes tradeoffs. The goal of this library it to provide the *majority* of performance gains gained from going scalar -> vectorized, while staying user-friendly. If you are looking to shave off the last few cycles this might not be what you are looking for.


### ⚠️ Warning ⚠️:
The library makes one extra assumption over the stdlib: The closure may be executed any number of times:

```Rust
arr.iter().simd_position(|x| {
    println!("hello world");
    *x == 42
})
```

May print a different number of times compared to the standard library. This shouldn't be an issue under normal use-cases but something to keep in mind.


### Why is this not part of the standard library
It's tricky. Hopefully one day.