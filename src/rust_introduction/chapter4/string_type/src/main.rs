fn main() {
	// String basic
	let s = "hello rust world.";
	println!("s is {}", s);
	let hello = &s[0..5];
	let world = &s[11..];
	println!("hello is {}", hello);
	println!("world is {}", world);
	println!("s is {}", s);
	let len = s.len();
	println!("s.len is {}", len);

	// String add
	let mut s = String::new();
	s.push_str("hello ");
	s.push_str("rust ");
	s.push_str("world!");
	println!("s is {}", s);

	let hello = "HELLO";
	let rust = "RUST";
	let world = "WORLD.";
	let s = format!("{} {} {}", hello, rust, world);
	println!("s is {}", s);

	// String japanese
	let s = "こんにちは rust コードの世界";
	println!("s is {}", s);

	let hello = &s[0..15];
	let world = &s[21..];
	println!("こんにちは is {}", hello);
	println!("コードの世界 is {}", world);

	let len = s.len();
	println!("len is {}", len);

	let mut s = String::new();
	s.push_str("こんにちは ");
	s.push_str("rust ");
	s.push_str("コードの世界");
	println!("s is {}", s);

	let hello = "こんにちは";
	let rust = "rust";
	let world = "コードの世界.";
	let s = format!("{} {} {}", hello, rust, world);
	println!("s is {}", s);

	// Japanese String to char Vectro
	let s = "This is ねこ😺neko 文字列";
	let mut v: Vec<char> = Vec::new();
	for c in s.chars() {
		v.push(c);
	}

	let v = &v[8..15];
	let mut s = String::new();
	for c in v {
		s.push(*c)
	}
	println!("s is {}", s);
	
}
