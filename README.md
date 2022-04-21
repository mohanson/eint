# Eint

Extended precision integer Rust library. Provides signed/unsigned integer 256 to 2048.

```text
[dependencies]
eint = "0.1"
```

# Usage

```rs
use eint::E256;

fn main() {
    let a = E256::from(u128::MAX);
    let b = E256::from(u128::MAX);
    println!("    a = {:?}", a);
    println!("    b = {:?}", a);
    println!("a + b = {:?}", a + b);
    println!("a - b = {:?}", a - b);
    println!("a * b = {:?}", a * b);
    println!("a / b = {:?}", a / b);
}
```

# Test

```sh
$ cargo test
```

# Maintainer

mohanson@outlook.com

# License

MIT.
