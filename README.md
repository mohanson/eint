# Eint

Extended precision integer Rust library. Provides signed/unsigned integer 256 to 2048.

```text
[dependencies]
eint = "0.1"
```

# Usage

```rs
use eint::{E256, Eint};

fn main() {
    let a = E256::from(u128::MAX);
    let b = E256::from(u128::MAX);
    println!("    a = {:?}", a);
    println!("    b = {:?}", a);
    println!("a + b = {:?}", a + b);
    println!("a - b = {:?}", a - b);
    println!("a * b = {:?}", a * b);
    println!("a / b = {:?}", a / b);
    println!("a.ctz = {:?}", a.ctz());
}
```

Full docs: [https://docs.rs/eint/latest/eint/](https://docs.rs/eint/latest/eint/)

# Test

```sh
# Test
$ cargo test

# Fuzzing
$ cargo +nightly fuzz run arith_e64
$ cargo +nightly fuzz run arith_e256

# Bench
$ cargo bench
```

# Maintainer

mohanson@outlook.com

# License

MIT.
