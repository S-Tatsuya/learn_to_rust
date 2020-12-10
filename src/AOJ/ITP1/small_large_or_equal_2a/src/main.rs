use std::cmp::Ordering;

fn main() {
	let mut number = String::new();
	std::io::stdin().read_line(&mut number).unwrap();

	let mut ws = number.split_whitespace();

	let a: i32 = ws.next().unwrap().parse().unwrap();
	let b: i32 = ws.next().unwrap().parse().unwrap();

	match a.cmp(&b) {
		Ordering::Less => println!("a < b"),
		Ordering::Greater => println!("a > b"),
		Ordering::Equal => println!("a == b"),
	}


}
