fn main() {
	// vector
	// let mut v1: Vec<i32> = Vec::new();
	let mut array = [0; 10000];
	// Panicked
	// let mut input = String::new();

	let mut counter = 0;
	loop {
		// NoPanicked
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		// Debug
		// println!("{}", input);
		let num = input.trim().parse().unwrap();
		if num == 0 {
			break;
		} else {
			// vector
			// v1.push(num);
			array[counter] = num;
			counter += 1;
		}
	}

	let mut i = 0;
	while counter > i {
		println!("{}", array[i]);
		i += 1;
	}
	// vector
	// for i in v1 {
	// 	println!("{}", i);
	// }
}

// fn main() {
// 	let mut input = String::new();
// 	std::io::stdin().read_line(&mut input).unwrap();
// 	let input = input.split_whitespace();
// 
// 	let mut counter = 1;
// 	for i in input {
// 		// Error この[i]は何者？strではない？
// 		// if i.parse().unwrap() == 0 {
// 		// 	break;
// 		// }
// 
// 		// NoError
// 		if i.parse::<i32>().unwrap() == 0 {
// 			break;
// 		}
// 		println!("case {} : {}",counter ,i);
// 		counter += 1;
// 	}
// }
