struct Circle {
	x: i32,
	y: i32,
	r: i32,
	top: i32,
	right: i32,
	left: i32,
	bottom: i32,
}

impl Circle {
	fn calc_point(&mut self) {
		self.top = self.y + self.r;
		self.right = self.x + self.r;
		self.left = self.x - self.r;
		self.bottom = self.y - self.r;
	}
}

struct Rectangle {
	w: i32,
	h: i32,
}

impl Rectangle {
	fn can_hold_circle(&self, _circle : &Circle) -> bool {
		if self.h < _circle.top {
			return false
		}

		if self.w < _circle.right {
			return false
		}

		if 0 > _circle.left {
			return false
		}

		if 0 > _circle.bottom {
			return false
		}

		true
	}
}

fn main() {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let mut input = input.split_whitespace();

	let mut circle1 = Circle {
		x: 0,
		y: 0,
		r: 0,
		top: 0,
		right: 0,
		left: 0,
		bottom: 0,
	};

	let mut rect1 = Rectangle {
		w: 0,
		h: 0,
	};

	rect1.w = input.next().unwrap().parse().unwrap();
	rect1.h = input.next().unwrap().parse().unwrap();
	circle1.x = input.next().unwrap().parse().unwrap();
	circle1.y = input.next().unwrap().parse().unwrap();
	circle1.r = input.next().unwrap().parse().unwrap();

	circle1.calc_point();
	
	if rect1.can_hold_circle(&circle1) {
		println!("Yes");
	} else {
		println!("No");
	}
}
