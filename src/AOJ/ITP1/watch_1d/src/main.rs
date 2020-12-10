struct Watch {
	h: u32,
	m: u32,
	s: u32,
}

impl Watch {
	fn time_calc(&mut self, num: u32) {
		let temp = self.second(num);
		let temp = self.minutes(temp);
		self.hour(temp);
	}
	
	fn second(&mut self, num: u32) -> u32 {
		self.s = num % 60;
		num / 60
	}

	fn minutes(&mut self, num: u32) -> u32 {
		self.m = num % 60;
		num / 60
	}

	fn hour(&mut self, num: u32) {
		self.h = num
	}

	fn disp(&self) {
		println!("{}:{}:{}", &self.h, &self.m, &self.s);
	}
}
use std::io;

fn main() {
	let mut s = String::new();

	io::stdin().read_line(&mut s).unwrap();
	let s = s.trim().parse().unwrap();

	let mut times = Watch {
		h: 0,
		m: 0,
		s: 0,
	};

	times.time_calc(s);
	times.disp();
}
