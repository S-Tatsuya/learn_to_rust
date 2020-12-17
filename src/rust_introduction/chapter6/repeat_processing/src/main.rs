fn main() {
	let v = vec![10, 20, 30, 40, 50];
	print!("v is ");
	// for文を参照にする場合としない場合の違いは？
	for i in &v {
		print!("{} ", i);
		let _x: i32 = *i;
	}
	println!("");

	// vector iterator
	let v = vec![10, 20, 30, 40, 50];
	print!("v is ");
	for i in v.iter() {
		print!("{} ", i);
		let _x: i32 = *i;
	}
	println!("");

	// vector iterator enumerate
	let v = vec![10, 20, 30, 40, 50];
	print!("v is ");
	for (i, x) in v.iter().enumerate() {
		print!("{}:{} ", i, x);
	}
	println!("");

	// slice pattern
	print!("For is ");
	for i in 0..10 {
		print!("{} ", i);
	}
	println!("");

	// break
	print!("For is ");
	for i in 0..10 {
		if i == 5 {
			break;
		}
		print!("{} ", i);
	}
	println!("");

	// continue
	print!("For is ");
	for i in 0..10 {
		if i % 2 == 0 {
			continue;
		}
		print!("{} ", i);
	}
	println!("");

	// while
	print!("While is ");
	let mut i = 0;
	while i < 10 {
		print!("{} ", i);
		i += 2;
	}
	println!("");

	// loop
	print!("Loop is ");
	let mut i = 0;
	loop {
		if i >= 10 {
			break;
		}
		print!("{} ", i);
		i += 1;
	}
	println!("");
}
