#[derive(Debug)]
enum LANG {
	JAPANESE = 81,
	ENGLISH = 44,
	CHINESE = 86,
	FRANCH = 33,
}

fn main() {
	// enum type
	let lang = LANG::JAPANESE;
	println!("lang is {}", lang as i32);

	// match
	let lang = LANG::ENGLISH;
	let m = match lang {
		LANG::JAPANESE => "日本語",
		LANG::ENGLISH => "英語",
		LANG::CHINESE => "中国語",
		LANG::FRANCH => "フランス語",
	};
	println!("lang is {}", m);

	// mathc _
	let lang = LANG::CHINESE;
	let m = match lang {
		LANG::JAPANESE => "日本語",
		_ => "日本語以外",
	};
	println!("lang is {}", m);

	// Option<T>
	let x = Some(10);
	let v = match x {
		Some(i) => i,
		None => -1,
	};
	println!("v is {}", v);

	let x = None;
	let v = match x {
		Some(i) => i,
		None => -1,
	};
	println!("v is {}", v);

	// if let
	let x = Some(10);
	match x {
		Some(i) => println!("i is {}", i),
		_ => println!("i is None"),
	};
	if let Some(i) = x {
		println!("i is {}", i);
	}
	
	// match do't use enum
	let x = 3;
	let m = match x {
		1 => "one",
		2 => "two",
		3 => "three",
		_ => "other",
	};
	println!("m is {}", m);

	let x = 7;
	let m = match x {
		0..=5 => "5以下",
		6..=10 => "6以上10以下",
		_ => "10より大きい",
	};
	println!("m is {}", m);

	let x = 'C';
	let m = match x {
		'A' => 1,
		'B' => 2,
		'C' => 3,
		_ => -1,
	};
	println!("m is {}", m);

	let x = "three";
	let m = match x {
		"one" => 1,
		"two" => 2,
		"three" => 3,
		_ => -1,
	};
	println!("m is {}", m);
}
