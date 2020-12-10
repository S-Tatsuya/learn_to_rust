fn main() {
	let mut number = String::new();
	std::io::stdin().read_line(&mut number).unwrap();
	let mut s_number = number.split_whitespace();

	let a: u32 = s_number.next().unwrap().parse().unwrap();
	let b: u32 = s_number.next().unwrap().parse().unwrap();
	let c: u32 = s_number.next().unwrap().parse().unwrap();

	disp_abc_result(a, b, c);
}

fn disp_abc_result(a: u32, b: u32, c:u32) {
	if a < b {
		if b < c {
			println!("Yes");
			return
		}
	}
	println!("No");
}
