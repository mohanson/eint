use eint::{Eint, E256};

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
