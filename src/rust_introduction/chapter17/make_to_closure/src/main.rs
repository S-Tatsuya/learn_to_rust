fn main() {
	let print_name = |name, age| {
		println!("name: {}, age: {}", name, age);
	};

	println!("call closure");
	print_name("masuda", 50);

	let print_name = |name: &str, age: i32| {
		println!("name: {}, age: {}", name, age);
	};

	println!("call closure2");
	print_name("masuda", 50);

	println!("use return of closure");
	let format_name = |name, age| {
		format!("name: {}, age: {}", name, age)
	};

	let x = format_name("kato", 40);
	println!("x is {}", x);

	println!("use map");
	let a = [("masuda", 50), ("kato", 40), ("yamada", 30)];
	let b = a.iter().map(|(name, age)| {
		format!("name: {}, age: {}", name, age)
	});
	for it in b {
		println!("{}", it);
	}
}
