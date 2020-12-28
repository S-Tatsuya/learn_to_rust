// #[derive(Debug)]
struct Person {
	id: i32,
	name: String,
	age: i32,
	addr: String,
}

fn main() {
	// let a = [1, 2, 3, 4, 5];
	// println!("a is {:?}", a);
	// dbg!(a);

	// let p = Person {
	// 	id: 100,
	// 	name: String::from("masuda"),
	// 	age: 50,
	// 	addr: String::from("Tokyo"),
	// };
	// println!("p is {:?}", p);
	// dbg!(p);

	let p = Person {
		id: 100,
		name: String::from("masuda"),
		age: 50,
		addr: String::from("Tokyo"),
	};
	dbg!(p);
}

impl std::fmt::Debug for Person {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{{ {}: `{}` in {}}}",
			self.id, self.name, self.addr)
	}
}
