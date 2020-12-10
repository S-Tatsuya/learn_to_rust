fn main() {
	// number
	let x = 100;
	let y = x;
	println!("x is {}", x);
	println!("y is {}", y);

	// String
	let x = String::from("Hello");
	let y = x;
	// Error borrow of moved value: x
	// println!("x is {}", x);
	println!("y is {}", y);

	// borrow
	let x = String::from("Hello");
	let len = string_length(&x);
	println!("len is {}", len);
	println!("x is {}", x);
}

fn string_length(s: &String) -> usize {
	let length = s.len();
	length
}
