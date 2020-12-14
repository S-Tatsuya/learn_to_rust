fn main() {
    // statement ;
    let a = 10;
    let b = 20;
    println!("a is {}, b is {}", a, b);

    // statement {;} <- error {} <- expression not error
    let a = { 10 + 20 };
    println!("a is {}", a);

    // func return valeu is expression
    let a = add(20, 20);
    println!("a is {}", a);

    let a = 10;
    if a > 0 {
        println!("a is {}", a);
    }
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}