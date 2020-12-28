fn main() {
    println!("hello rust world.");
	println!("hello {} world.", "rust");

	println!("number is {}", 10);
	println!("float is {}", 10.234);

	println!("tuple is {:?}.", ("masuda", 50));

	let n = Option::<i32>::Some(10);
	println!("option is {:?}.", n);
	let n = Option::<i32>::None;
	println!("option is {:?}.", n);
	let n = Option::<i32>::Some(11);
	println!("option is {}.", n.unwrap());

	println!("a is {}, b is {}.", 100, "test");
	println!("a is {a}, b is {b}.", b = "test", a = 100);

	let n = 10;
	println!("Decimal number {}.", n);
	println!("Hexadecimal nubmer {:x}.", n);
	println!("Hexadecimal nubmer {:X}.", n);
	println!("Octadecimal nubmer {:o}.", n);
	println!("Binary nubmer {:b}.", n);

	let n = 10;
	println!("nomal          {}", n);
	println!("Aignment       {:4}", n);
	println!("Aignment(zero) {:04}", n);
	println!("Hexadecimal    {:02x}", n);
	println!("binary         {:08b}", n);

	let f = 10.234;
	println!("{}", f);
	println!("{:e}", f);
	println!("{:.2}", f);
	let f = 0.0234;
	println!("{} ", f);
	println!("{:e}", f);
	println!("{:E}", f);
	println!("{:.2}", f);

	println!("hello, `{:8}` world.", "rust");
	println!("hello, `{:<8}` world.", "rust");
	println!("hello, `{:>8}` world.", "rust");
	println!("hello, `{:^8}` world.", "rust");
	println!("hello, `{:8}` world.", 123);
	println!("hello, `{:<8}` world.", 123);
	println!("hello, `{:>8}` world.", 123);
	println!("hello, `{:^8}` world.", 123);
}
