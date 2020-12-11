fn main() {
    // mutable
    let mut x = 100;
    println!("x is {}", x);
    x = 200;
    println!("x is {}", x);

    // shadowing
    let x = 500;
    println!("x is {}", x);
    let x = 300;
    println!("x is {}", x);
    let x = 400;
    println!("x is {}", x);
}
