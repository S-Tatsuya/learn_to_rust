struct Number {
	x: u32,
	y: u32,
}

impl Number {
	fn compare(&self) {
		if self.x < self.y {
			println!("{} {}", self.x, self.y);
		} else {
			println!("{} {}", self.y, self.x)
		}
	}

	fn disp(&self) {
		println!("{} {}", self.x, self.y);
	}
}

fn main() {
	let mut num_vector: Vec<Number> = Vec::new();
	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();

		let mut input_iter = input.split_whitespace();

		let temp_num = Number{
			x: input_iter.next().unwrap().parse().unwrap(),
			y: input_iter.next().unwrap().parse().unwrap(),
		};

		if temp_num.x == 0 && temp_num.y == 0 {
			break;
		}

		num_vector.push(temp_num);
	}

	for i in num_vector {
		i.compare();
	}
}
