struct Chessbard {
	h: u32, 
	w: u32,
}

impl Chessbard {
	fn disp(&self) {
		for i in 0..self.h {
			for m in 0..self.w {
				if (i + m) % 2 == 0 {
					print!("#");
				} else {
					print!(".");
				}
			}	
			println!("");
		}
		println!("");
	}
}

fn main() {
	let mut chess: Vec<Chessbard> = Vec::new();

	loop {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		let mut input = input.split_whitespace();

		let temp_chess = Chessbard {
			h: input.next().unwrap().parse().unwrap(),
			w: input.next().unwrap().parse().unwrap(),
		};

		if temp_chess.h == 0 && temp_chess.w == 0 {
			break;
		}

		chess.push(temp_chess);
	}

	for i in chess {
		i.disp();
	}
}
