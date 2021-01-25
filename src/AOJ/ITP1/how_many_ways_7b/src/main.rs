fn main() {
	let mut count = 0;
	let mut result: Vec<i32> = Vec::new();
	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		let mut input = input.split_whitespace();

		let n: i32 = input.next().unwrap().parse().unwrap();
		let x: i32 = input.next().unwrap().parse().unwrap();

		if n == 0 && x == 0{
			break;
		}

		for i in 1..(n + 1){
			for j in (i + 1)..(n + 1){
				for k in (j + 1)..(n + 1){
					if (i + j + k) == x{
						count += 1;	
					}
				}

			}
		}
		result.push(count);
		count = 0;
	}

	for i in result {
		println!("{}", i);
	}
}
