#  SIMD-accelerated iterators

Change:
```Rust
arr.iter().min()
```
To:
```Rust
arr.iter().min_simd()
```


Currently the following are implemented:

```find```
```filter```
```position```
```contains```
```eq```
```min/max```
```is_sorted```
```all_equal```



###  Performance gain of compared to std implementation (u32) 🔥
![Performance gain of compared to std implementation (u32)](benchmark.png)


Requires nightly for now :/:
```Rust
rustup toolchain install nightly
rustup default nightly
// revert back to stable: rustup default stable
```
