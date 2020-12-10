fn main() {
	// 型の宣言
	println!("{}", add(10 , 20));

	// 浮動小数点
	let x = 100.234;
	println!("x is {}", x);
	let x : f64 = 900.876;
	println!("x is {}", x);

	// 2進数
	let b = 0b1010;
	println!("b is {}", b);

	// 論理値
	let f = true;
	println!("f is {}", f);

	// &str Type, String Type
	let dog = "Dog";
	let cat = "CAT";
	println!("{} and {}", dog, cat);
	let s = String::from("Hello Rust world.");
	println!("{}", s);
}

fn add( x: i32, y: i32 ) -> i64 {
	println!("call add");
	// into()メソッドを使わないとi32とi64で型が違うため、
	// コンパルエラーになってしまう。
	(x + y).into()
}
