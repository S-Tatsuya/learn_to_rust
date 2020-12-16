struct Calculator {
	a: i32,
	b: i32,
	op: String,
}

impl Calculator {
	fn calc_op(&self) -> i32 {
		if self.op == "+" {
			self.a + self.b
		} else if self.op == "-" {
			self.a - self.b
		} else if self.op == "*" {
			self.a * self.b
		} else if self.op == "/" {
			self.a / self.b
		} else {
			0
		}
	}
}

fn main() {
	let mut calc_vec: Vec<Calculator> = Vec::new();
	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		let mut input = input.split_whitespace();

		let calc_temp = Calculator {
			a: input.next().unwrap().parse().unwrap(),
			op: input.next().unwrap().to_string(),
			b: input.next().unwrap().parse().unwrap(),
		};

		if calc_temp.op == "?" {
			break;
		}

		calc_vec.push(calc_temp);
	}

	for i in calc_vec {
		println!("{}", i.calc_op());
	}

}
