struct Rectange {
	width: f32,
	height: f32,
}

struct Triangle {
	base: f32,
	height: f32,
}

struct Circle {
	radius: f32,
}

trait CalcArea {
	fn calc_area(&self) -> f32;
}

trait ExprString {
	fn expr_str(&self) -> String {
		"width * height = ".to_string()
	}
}

impl CalcArea for Rectange {
	fn calc_area(&self) -> f32 {
		self.width * self.height
	}
}

impl ExprString for Rectange {}

impl CalcArea for Triangle {
	fn calc_area(&self) -> f32 {
		self.base * self.height * 0.5
	}
}

impl ExprString for Triangle {
	fn expr_str(&self) -> String {
		"base * height = ".to_string()
	}
}

impl CalcArea for Circle {
	fn calc_area(&self) -> f32 {
		self.radius * self.radius * 3.14
	}
}

impl ExprString for Circle {
	fn expr_str(&self) -> String {
		"pi * redius * redius = ".to_string()
	}
}

trait ToNumber {
	fn to_i(&self) -> i32;
}

impl ToNumber for String {
	fn to_i(&self) -> i32 {
		match self.parse::<i32>() {
			Ok(n) => n,
			Err(_) => 0,
		}
	}
}

fn main() {
	let rect = Rectange {
		width: 10.0,
		height: 20.0,
	};

	let tri = Triangle {
		base: 10.0,
		height: 20.0,
	};

	let cir = Circle {
		radius: 10.0,
	};
	println!("rect {} {}", rect.expr_str(), rect.calc_area());
	println!("tri {} {}", tri.expr_str(), tri.calc_area());
	println!("cir {} {}", cir.expr_str(), cir.calc_area());

	let s = String::from("100");
	let n = s.to_i();
	println!("n is {}", n);
}
