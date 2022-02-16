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

    // let c = eint::E64(1);
    // let d = E256::from(c);
    // println!("{:?}", d);
}
