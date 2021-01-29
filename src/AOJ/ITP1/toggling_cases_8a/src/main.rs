fn main() {
	let data = input1();

	let change_data = change_upper_lower(&data);
	
	print!("{}", change_data);
}

fn input1() -> String {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();

	input
}

fn change_upper_lower(data: &str) -> String {
	data.chars().map(char_change).collect()
}

fn char_change(c: char) -> char {
	if c.is_uppercase() {
		c.to_ascii_lowercase()
	} else {
		c.to_ascii_uppercase()
	}
}
