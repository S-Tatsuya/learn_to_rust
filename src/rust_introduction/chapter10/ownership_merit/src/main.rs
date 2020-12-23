#[derive(Debug)]
struct Person {
	name: String,
	age: i32,
}

fn main() {
	// let x: &Person;
	let x: Person;
	{
		let a = Person {
			name: String::from("masuda"),
			age: 50,
		};

		// x = &a;
		x = a;
	}
	println!("x is {:?}", x);

	let a = new_person("masuda", 50);
	println!("a is {:?}", a);


	let mut a = Person {
		name: String::from("masuda"),
		age: 50,
	};
	println!("a is {:?}", a);
	let mut x = &mut a;
	x.age = 0;
	println!("x is {:?}", x);
	let mut y = &mut a;
	y.name = String::from("kato");
	println!("a is {:?}", y);
	println!("a is {:?}", a);


	let mut a = Person {
		name: String::from("masuda"),
		age: 50,
	};
	println!("a is {:?}", a);
	let mut x = &mut a;
	println!("x is {:?}", x);
	x.name = String::from("kato");
	x.age = 0;
	println!("x is {:?}", x);
	println!("a is {:?}", a);
}

fn new_person(_name: &str, _age: i32) -> Person {
	let p = Person {
		name: _name.to_string(),
		age: _age,
	};
	p
}
