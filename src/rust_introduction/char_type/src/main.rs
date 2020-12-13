fn main() {
	// char_type Char型は4バイトである。
	let ch = 'A';
	println!("ch is {}", ch);
	let ch = 'あ';
	println!("ch is {}", ch);
	let ch = '😺';
	println!("ch is {}", ch);
	let ch: char = '🐶';
	println!("ch is {}", ch);

	// cast char <-> u8 Unicode(32bit) -> ASCII(8bit)となるので、英数記号のみ変換可能。日本語は無理
	let ch = 'A';
	println!("ch is {}", ch);
	let u = ch as u8;
	println!("u is {}", u);
	let ch = u as char;
	println!("ch is {}", ch);
}
