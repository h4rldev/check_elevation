# check_elevation

[![license](https://img.shields.io/github/license/h4rldev/check_elevation?style=flat-square)](https://crates.io/crates/check_elevation)
[![version](https://img.shields.io/crates/v/check_elevation?style=flat-square)](https://crates.io/crates/check_elevation)

A tool to check the elevation status through a simple function.

Successor to [is_elevated](https://crates.io/crates/is_elevated).

#### Example

```rust
use check_elevation::is_elevated;

fn main() {
    let elevation_status = is_elevated().expect("Failed to check elevation");
    println!("Elevation Status: {}" elevation_status);
}
```

#### Dependencies

- [windows](https://crates.io/crates/windows)
