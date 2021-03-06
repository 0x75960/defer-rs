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

**macro(recommended)**

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

**Basic**

```rust
extern crate defer;

use defer::Defer;

fn main() {
    let defer1 = Defer::register(|| -> () {
        println!("this will appear 4th!");
    });

    let defer2 = Defer::register(|| -> () {
        println!("this will appear 2nd!");
        println!("this will appear 3rd!");
    });
    println!("this will appear 1st!");
}
```

### attention

* there are least one difference between go and this.

|              # | in this library   | go(original)         |
|---------------:|:------------------|:---------------------|
| 1. executed at | **leaving scope** | **leaving function** |
