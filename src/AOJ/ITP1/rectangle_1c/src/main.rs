use std::io;

struct Rectangle {
	x: u32,
	y: u32,
}

impl Rectangle {
	fn area(&self) -> u32 {
		self.x * self.y	
	}

	fn circumference(&self) -> u32 {
		(self.x * 2) + (self.y * 2)
	}
}

fn main() {
	let mut rect1 = Rectangle {
		x: 0,
		y: 0,
	};

	let mut temp = String::new();

	io::stdin().read_line(&mut temp)
		.expect("aaa");

	let mut ws = temp.split_whitespace();

	rect1.x = ws.next().unwrap().parse().unwrap();
	rect1.y = ws.next().unwrap().parse().unwrap();

	println!("{} {}", rect1.area(), rect1.circumference());
}
