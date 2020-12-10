//fn main() {
	// tuple type
	let t = ("masuda", 30);
	println!("name is {} age {}", t.0, t.1);

	let name = "masuda";
	let age = 30;
	let t = (name, age);
	println!("name is {} age {}", t.0, t.1);
	// array type
	let a = ["spring", "summer", "autumn", "winter"];
	println!("first season {}", a[0]);
	println!("final season {}", a[3]);
}
