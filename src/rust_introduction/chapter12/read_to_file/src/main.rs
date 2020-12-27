use std::fs::File;
// use std::fs::*;
use std::io::Read;
use std::io::{BufRead, BufReader};
// fn main() {
// 	let path = "sample.txt";
// 	println!("read all lines.");
// 
// 	if let Ok(data) = std::fs::read_to_string(path) {
// 		println!("data is {}", data);
// 	}
// 
// 	// error pattern "if let"
// 	let path = "unkonwn.txt";
// 	println!("read all lines.");
// 	
// 	if let Ok(data) = std::fs::read_to_string(path) {
// 		println!("data is {}", data);
// 	} else {
// 		println!("cannot open {}", path);
// 	}
// 
// 	// error pattern "match"
// 	let path = "unkonwn.txt";
// 	println!("read all lines.");
// 	
// 	match std::fs::read_to_string(path) {
// 		Ok(data) => {println!("data is {}", data);},
// 		_ => {println!("cannot open {}", path);}
// 	}
// 
// 	// opne pattern
// 	let path = "sample.txt";
// 	println!("read all lines by buffer.");
// 	let mut file = std::fs::File::open(path)
// 		.expect("file not found.");
// 	let mut data = String::new();
// 	file.read_to_string(&mut data)
// 		.expect("read error");
// 	println!("data is {}", data);
// 
// 	// open pattern if let
// 	let path = "sample.txt";
// 	println!("read all lines by buffer.");
// 	if let Ok(mut file) = std::fs::File::open(path) {
// 		let mut data = String::new();
// 		if let Ok(_) = file.read_to_string(&mut data) {
// 			println!("data is {}", data);
// 		}
// 	}
// 
// 	// byte unit
// 	let path = "sample.txt";
// 	println!("read 16 byte by buffer.");
// 	let mut file = std::fs::File::open(path)
// 		.expect("file not found.");
// 	let mut buf: [u8; 1] = [0; 1];
// 	for i in 0..16 {
// 		file.read(&mut buf);
// 		println!("buf is {}: {}", i, buf[0] as char);
// 	}
// 
// 	// lines unit
// 	println!("read every on line.");
// 	let file = File::open(path)
// 		.expect("file not found.");
// 	for line in BufReader::new(file).lines() {
// 		if let Ok(l) = line {
// 			println!("line is {}", l);
// 		}
// 	}
// }

fn main() -> std::io::Result<()> {
	let path = "sample.txt";
	let file = File::open(path)?;
	for line in BufReader::new(file).lines() {
		println!("line is {}", line?);
	}
	Ok(())
}
