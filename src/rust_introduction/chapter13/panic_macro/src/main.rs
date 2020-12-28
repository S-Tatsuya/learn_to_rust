fn main() {
	// panic!("This progrma don't move.");

	// let args = std::env::args().collect::<Vec<String>>();
	// if args.len() <= 1 {
	// 	panic!("need to parameter!!");
	// } else {
	// 	for (i, s) in args.iter().enumerate() {
	// 		println!("{}: {}", i, s);
	// 	}
	// }

	// let path = "unknown.txt";
	// if std::path::Path::new(path).exists() {
	// 	println!("exists");
	// } else {
	// 	panic!("not exists");
	// }

	sub();
}

fn sub() {
	subsub();
}

fn subsub() {
	panic!("This program don't move.");
}
