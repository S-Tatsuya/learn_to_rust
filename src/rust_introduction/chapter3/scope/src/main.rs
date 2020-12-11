fn main() {
    // scope block
    let x = 100;
    println!("x is {}", x);
    {
        let x = 200;
        println!("x is {}", x);
    }

    println!("x is {}", x);
    
    // scope block test
    println!("{}", test(-1));
    println!("{}", test(50));
    println!("{}", test(101));
}

fn test(x: i32) -> i32 {
    // scope block
    let mut ans = x;
    if x < 0 {
        ans = 0;
    }
    if x > 100 {
        ans = 100;
    }
    ans
}