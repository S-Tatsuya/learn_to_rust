struct Sample {
	x: i32,
}
impl Sample {
	fn new(x: i32) -> Sample {
		Sample {x: x}
	}
	fn inc(&self) -> i32 {
		self.x + 1
	}
	fn add(&self, x: i32) -> i32 {
		self.x + x
	}
}

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

	// strucp scope
	let a = Sample::new(10);
	let ans = a.inc();
	println!("ans is {}", ans);
	let ans = a.add(20);
	println!("ans is {}", ans);

	// closure scope
	let num = 10;
	let add_one = |x| {num + x};
	let add_two = |x, y| {x + y};

	let ans = add_one(1);
	println!("ans is {}", ans);
	let ans = add_two(10, 20);
	println!("ans is {}", ans);
	
	// test
	test2();
}

fn test(x: i32) -> i32 {
    // scope block
    let ans = if x < 0 {
        0
    } else if x > 100 {
        100
    } else {
		x
	};
    ans
}

fn test2() {
	let num = 10;
	fn add_one(x: i32, num: i32) -> i32 {
		num + x
	}
	let ans = add_one(1, num);
	println!("ans is {}", ans);
}
