defer-rs
========

golang-like defer provider

usage
-----

1. add dependency into Cargo.toml

```toml
[dependencies]
defer = { git = "https://github.com/0x75960/defer-rs", branch = "master" }
```

2. import and use in your code

```rust
#[macro_use(defer)]
extern crate defer;

 ```rust
#[macro_use(defer)]
extern crate defer;

use defer::Defer;

fn main() {
    defer!({
        println!("this will appear 4th!");
    });
    defer!({
        println!("this will appear 2nd!");
        println!("this will appear 3rd!");
    });
    println!("this will appear 1st!");
}
```
