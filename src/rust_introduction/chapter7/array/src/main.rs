fn main() {
	let ary = [1,2,3,4,5];
	println!("ary[0] is {}", ary[0]);
	println!("ary[4] is {}", ary[4]);
	println!("ary.len is {}", ary.len());

	// range out
	// let ary = [1,2,3,4,5];
	// println!("ary[0] is {}", ary[10]);
	// let n = 10
	// println!("ary[0] is {}", ary[n]);
	// for n in 0..10 {
	// 	println!("ary[0] is {}", ary[n]);
	// }

	// array str
	let ary = ["one", "two", "three", "four", "five"];
	println!("ary[0] is {}", ary[0]);
	println!("ary[4] is {}", ary[4]);
	println!("ary.len is {}", ary.len());

	// array type designation
	let ary: [u8; 5] = [0x10, 0x20, 0x30, 0x40, 0x50];
	println!("ary[0] is {}", ary[0]);
	println!("ary[4] is {}", ary[4]);
	println!("ary.len is {}", ary.len());

	// array no element
	let ary: [u8; 0] = [];
	println!("arr.len is {}", ary.len());

	// array initialize
	let mut ary: [u8; 16] = [0; 16];
	println!("ary[0] is {}", ary[0]);
	println!("ary[15] is {}", ary[15]);
	ary[0] = 10;
	println!("ary[0] is {}", ary[0]);
	println!("ary[15] is {}", ary[15]);

	// array forced cast
	let a = [1u8, 2u8, 3u8, 4u8];
	// let b = a as u32
	unsafe {
		let b = std::mem::transmute::<[u8; 4], u32>(a);
		println!("b is {:X}", b);
	}

	let a: u32 = 0x11223344;
	unsafe {
		let b = std::mem::transmute::<u32, [u8; 4]>(a);
		println!("b[] is {:X} {:X} {:X} {:X}", 
				b[0], b[1], b[2], b[3]);
	}
}
