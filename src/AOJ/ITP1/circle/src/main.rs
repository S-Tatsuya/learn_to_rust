struct Circles {
	r: f64,
}

impl Circles {
	fn area(&self) -> f64 {
		// 小数点以下の出力を揃えるためf64で計算して、小数点以下の値を揃える
		(self.r * self.r * std::f64::consts::PI * 1000000.0).round() / 1000000.0
	}

	fn circumference(&self) -> f64 {
		(self.r * 2.0 * std::f64::consts::PI * 1000000.0).round() / 1000000.0
	}
}
fn main() {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let circle = Circles {
		r: input.trim().parse().unwrap(),
	};
	
	println!("{} {}", circle.area(), circle.circumference());
}
