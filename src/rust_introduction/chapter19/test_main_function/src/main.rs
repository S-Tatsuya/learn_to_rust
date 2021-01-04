fn main() {
	println!("Hello, Rust world!");
	let mut a = Person {id: 100, name: "masuda".to_string(), age: 50};
	let mut b = Person {id: 100, name: "masuda".to_string(), age: 50};
	let mut c = Person {id: 100, name: "kato".to_string(), age: 40};

	assert_eq!("masuda", a.name);
	assert_eq!("kato", c.name);
	assert_ne!("unknown", c.name);
	println!("a is {:?}", a);
	println!("c is {:?}", c);
	assert_eq!(a, b);
	a.age += 1;
	println!("a is {:?}", a);
	assert_ne!(a, b);
}

#[derive(Debug, PartialEq)]
struct Person {
	id: i32,
	name: String,
	age: i32,
}

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn test_add() {
		assert_eq!(3, add(1, 2));
	}

	#[test]
	fn test_add_zero() {
		assert_eq!(0, add(0, 0));
	}

	#[test]
	fn test_add_under_zero() {
		assert_eq!(0, add(-1, -1));
	}
}

fn add(x: i32, y: i32) -> i32 {
	let ans = x + y;
	if ans < 0 {
		0
	} else {
		ans
	}
}
