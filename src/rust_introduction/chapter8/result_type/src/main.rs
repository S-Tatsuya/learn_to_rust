fn main() {
    let r = "100".parse::<i32>();
    match r {
        Ok(n) => println!("n is {}", n),
        Err(e) => println!("error: {:?}", e),
    }

    let r = "xxx".parse::<i32>();
    match r {
        Ok(b) => println!("b is {}", b),
        Err(m) => println!("error: {:?}", m),
    }
    
    let n = "100".parse::<i32>().unwrap();
    println!("n is {}", n);

    // panic pattern
    // let n = "xxx".parse::<i32>().unwrap();
    // println!("n is {}", n);
    
    // return result type
    println!("---return result type---");
    // let n: i32 = half_number("100");
    // println!("n is {}", n);
    // let n: i32 = half_number("xxx");
    // println!("n is {}", n);

    // match half_number("100") {
    //     Ok(n) => println!("Ok: {}", n),
    //     Err(err) => println!("Error: {:?}", err),
    // }

    // match half_number("xxx") {
    //     Ok(n) => println!("Ok: {}", n),
    //     Err(err) => println!("Error: {:?}", err),
    // }

    // Result type alias
    // match half_number2("100") {
    //     Ok(n) => println!("Ok: {}", n),
    //     Err(err) => println!("Error: {:?}", err),
    // }

    // match half_number3("100") {
    //     Ok(n) => println!("Ok: {}", n),
    //     Err(err) => println!("Error: {:?}", err),
    // }

    // match half_number3("xxx") {
    //     Ok(n) => println!("Ok: {}", n),
    //     Err(err) => println!("Error: {:?}", err),
    // }
    match half_number("100") {
        Ok(n) => println!("Ok:: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }

    match half_number("xxx") {
        Ok(n) => println!("Ok: {}", n),
        Err(err) => println!("Error: {:?}", err),
    }
}

use std::num::ParseIntError;
// fn half_number(s: &str) -> Result<i32, ParseIntError> {
//     match s.parse::<i32>() {
//         Ok(n) => Ok(n / 2),
//         Err(err) => Err(err),
//     }
// }

// type Result<T> = std::result::Result<T, ParseIntError>;
// fn half_number2(s: &str) -> Result<i32> {
//     match s.parse::<i32>() {
//         Ok(n) => Ok(n / 2),
//         Err(err) => Err(err),
//     }
// }

// fn half_number3(s: &str) -> Result<i32, ParseIntError> {
//     s.parse::<i32>().map(|n| n / 2)
// }

// fn half_number(s: &str) -> Result<i32, ParseIntError> {
//     let n = s.parse::<i32>()?;
//     Ok(n / 2)
// }
fn half_number(s: &str) -> Result<i32, &str> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n / 2),
        Err(err) => Err("run error: not number"),
    }
}