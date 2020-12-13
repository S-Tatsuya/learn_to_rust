struct Numbers {
	a: u64,
	b: u64,
}

impl Numbers {
	fn add(&self) -> u64 {
		self.a / self.b
	}

	fn remainder(&self) -> u64 {
		self.a % self.b
	} 

	fn division(&self) -> f64 {
		self.a as f64 / self.b as f64
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let mut input = input.split_whitespace();

	let num = Numbers {
		a : input.next().unwrap().parse().unwrap(),
		b : input.next().unwrap().parse().unwrap(),
	};
	
	println!("{} {} {}", num.add(), num.remainder(), num.division());
}
