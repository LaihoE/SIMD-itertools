##  SIMD alterinatives for common Rust operations


Implements the following:

- ```min/max```
- ```position```
- ```find```
- ```contains```
- ```equal```
- ```all_equal```
- ```is_sorted```


Change:
```Rust
arr.iter().min()
```
To:
```Rust
arr.iter().min_simd()
```




