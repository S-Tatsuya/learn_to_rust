use std::io;

fn main() {
	let mut number = String::new();

    io::stdin().read_line(&mut number)
		.expect("Failed to read line");

	let number: u32 = number.trim().parse()
		.expect("Please type a number!");

	println!("{}", number.pow(3));
}
