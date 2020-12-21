fn main() {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let n = input.trim().parse().unwrap();

	call(n);
}

fn call(n: i32) {
	let mut i = 1;

	while i <= n {
		if i % 3 == 0 {
			print!(" {}", i);
		} else {
			let mut x = i;
			while x != 0 {
				if x % 10 == 3 {
					print!(" {}", i);
					break;
				} else {
					x /= 10;
				} 
			}
			
		}
		i += 1;
	}
	println!("");
}
