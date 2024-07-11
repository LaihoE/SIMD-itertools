##  SIMD alterinatives for common Rust operations

Change:
```Rust
arr.iter().min()
```
To:
```Rust
arr.iter().min_simd()
```

Requires nightly for now:
```
rustup toolchain install nightly
rustup run nightly cargo run
```
Or set nightly as default: 
```
rustup default nightly
cargo run
```

Implements the following:

- ```min/max```
- ```position```
- ```find```
- ```contains```
- ```equal```
- ```all_equal```
- ```is_sorted```







