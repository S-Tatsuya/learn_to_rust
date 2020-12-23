#[derive(Debug)]
struct Person {
	name: &'static str,
	age: i32,
}

fn print_a(a: &Person) {
	println!("print_a: a is {:?}", a);
}

fn move_a(a: Person) {
	println!("move_a: a is {:?}", a);
}

fn add_age(a: &mut Person) {
	a.age += 1;
}

fn main() {
	let a = Person {name: "masuda", age: 50};
	print_a(&a);
	println!("main: a is {:?}", a);

	move_a(a);
	// println!("main: a is {:?}", a);

	let a = Person {name: "masuda", age: 50};
	let x = &a;
	println!("変数a の場合: {:?}", a);
	println!("変数x の場合: {:?}", x);

	let x = a;
	// println!("変数a の場合: {:?}", a);
	println!("変数x の場合: {:?}", x);

	let a = Person {name: "masuda", age: 50};
	let x = a;
	// let y = a;
	// println!("a is {:?}", a);
	println!("x is {:?}", x);
	// println!("y is {:?}", y);

	println!("mutable variable");
	let mut a = Person {name: "masuda", age: 50};
	println!("a is {:?}", a);
	add_age(&mut a);
	println!("a is {:?}", a);

	let a = Person {name: "masuda", age: 50};
	let mut x = a;
	println!("x is {:?}", x);
	add_age(&mut x);
	println!("x is {:?}", x);
	// add_age(&mut a);
	// println!("a is {:?}", x);
	
	let a = Person {name: "masuda", age: 50};
	let mut x = &a;
	println!("x is {:?}", x);
	// x.age += 1;
	// println!("x is {:?}", x);

	let mut a = Person {name: "masuda", age: 50};
	let mut x = &mut a;
	println!("x is {:?}", x);
	x.age += 1;
	println!("x is {:?}", x);

	let a = Person {name: "masuda", age: 50};
	let mut x = a;
	println!("x is {:?}", x);
	x.age += 1;
	println!("x is {:?}", x);
	add_age(&mut x);
	println!("x is {:?}", x);
	// a.age += 1;
	// println!("a is {:?}", a);

	println!("number and tuple copy");
	let a = 100;
	println!("a is {}", a);
	let x = a;
	println!("x is {}", x);
	let y = a;
	println!("y is {}", y);

	let a = (100, "masuda");
	println!("a is {:?}", a);
	let x = a;
	println!("x is {:?}", x);
	let y = a;
	println!("y is {:?}", y);

	println!("vector and string type");
	let a = vec!["one", "two", "three"];
	println!("a[] is {:?}", a);
	let x = &a;
	println!("x[] is {:?}", x);
	let y = a;
	println!("y[] is {:?}", y);

	let s = String::from("masuda");
	println!("a is {}", s);
	let x = &s;
	println!("x is {}", x);
	let y = s;
	println!("y is {}", y);

	println!("ownership mix");
	let a = Person {name: "masuda", age: 50};
	println!("a is {:?}", a);
	let x = &a;
	println!("variable borrowed");
	println!("x is {:?}", x);
	println!("a is {:?}", a);
	let y = a;
	println!("variable move");
	println!("y is {:?}", y);
	// println!("a is {:?}", a);
	// println!("x is {:?}", x);
}
