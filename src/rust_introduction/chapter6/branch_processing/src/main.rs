fn main() {
	// if sentence
	let a = 40;
	let b = 30;
	if a == b {
		println!("a == b is ok.");
	} else if a < b {
		println!("a < b is ok.");
	} else {
		println!("a > b is ok.");
	}

	let a = 10;
	let b = 20;

	if a == 10 && b == 20 {
		println!("AND is ok.");
	}

	if a == 0 || b == 20 {
		println!("OR is ok.");
	}

	if test(a, b) {
		println!("test is ok.");
	}

	let a = 10;
	let b = 20;
	let x = if a < b {1} else {0};
	println!("x is {}", x);
}

fn test(x: i32, y: i32) -> bool {
	x < y
}
