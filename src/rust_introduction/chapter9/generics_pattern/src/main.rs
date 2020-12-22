fn main() {
	// use to generics
	// let mut v = vec![1, 2, 3, 4, 5];
	// let mut v: Vec<i32> = Vec::new();
	// let mut v = Vec::<i32>::new();

	// let mut v: Vec::<&str> = Vec::new();
	// let mut v: Vec::<&String> = Vec::new();
	// let mut v: Vec::<f64> = Vec::new();

	// make to generics function
	let v = [10, 20, 30, 40, 50];
	print(&v);
	let v = ['A', 'B', 'C', 'D', 'E'];
	print(&v);
	let v = ["one", "two", "three", "four", "five"];
	print(&v);
}

// use to generics
fn print<T>(a: &[T]) where T: std::fmt::Debug {
	print!("a is ");
	for i in a {
		print!("{:?} ", i);
	}
	println!("");
}
// not use to generics
// fn print_i32(a: &[i32]) {
// 	print!("a is ");
// 	for i in a {
// 		print!("{} ", i);
// 	}
// 	println!("");
// }
// 
// fn print_char(a: &[char]) {
// 	print!("a is ");
// 	for i in a {
// 		print!("{} ", i);
// 	}
// 	println!("");
// }
// 
// fn print_str(a: &[&str]) {
// 	print!("a is ");
// 	for i in a {
// 		print!("{} ", i);
// 	}
// 	println!("");
// }
