# Ent &emsp; ![Build] ![Crate]

[Build]: https://github.com/nkconnor/ent/workflows/build/badge.svg
[Crate]: https://img.shields.io/crates/v/ent


**_Ent is an abstract object library_**. Using Rust's [monomorphization](https://doc.rust-lang.org/book/ch10-01-syntax.html),
you can write zero cost interfaces across multiple object types including [JSON](https://github.com/serde-rs/json) 
and [Python dictionaries](https://pyo3.rs). If you are interested in contributing to Ent, please do! We would welcome 
any help including additional object implementations.

**Warning: Ent is in early development and depends on Rust nightly!** 

## Getting Started

```toml
[dependencies]

# Specify supported implementations using feature keys:
#  - python
#  - json
ent = { version = "0.1", features = ["derive"] }
```

## Examples

```rust
use ent::Ent;

fn get_name<E: Ent>(e: Ent) -> &str {
    e.as_str()
}

fn main() {
    let json: Value = serde_json::from_str(..).unwrap();
    println!(get_name(json))
}
```

## License

Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in Ent by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
