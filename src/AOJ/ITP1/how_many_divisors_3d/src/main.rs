struct Numbers {
	a: u32,
	b: u32,
	c: u32,
}

impl Numbers {
	fn divisors_count(&self) -> i32 {
		let mut counter = 0;
		for i in self.a..(self.b + 1) {
			if self.c % i == 0{
				counter += 1;
			}
		}
		counter
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	// mutでないとその次の要素の抜き出しができない理由がわからない
	let mut input_iter = input.split_whitespace();

	let num1 = Numbers {
		a: input_iter.next().unwrap().parse().unwrap(),
		b: input_iter.next().unwrap().parse().unwrap(),
		c: input_iter.next().unwrap().parse().unwrap(),
	};

	println!("{}", num1.divisors_count());
}
