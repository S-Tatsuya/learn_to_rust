fn main() {
	let mut input1 = String::new();
	std::io::stdin().read_line(&mut input1).unwrap();
	let input1: i32 = input1.trim().parse().unwrap();

	let mut input2 = String::new();
	std::io::stdin().read_line(&mut input2).unwrap();
	let input2 = input2.split_whitespace();

	let mut v = Vec::new();
	for i in input2 {
		v.push(i)
	}

	let mut count = 1;
	for i in v.iter().rev() {
		print!("{}", i);
		count += 1;
		if count <= input1 {
			print!(" ");
		}
	}
	
	println!("");
}
