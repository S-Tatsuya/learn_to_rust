use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
	let path = "sample.txt";
	let mut file = File::create(path)
		.expect("file not found.");
	writeln!(file, "hello rust world.")
		.expect("cannot write.");

	let path = "sample2.txt";
	let mut file = File::create(path)
		.expect("file not found.");
	file.write(b"hello rust world.\n")
		.expect("cannot write.");

	let path = "sample3.txt";
	let mut file = File::create(path)
		.expect("file not found.");
	let s = "hello rsut world.\n";
	file.write(s.as_bytes())
		.expect("cannot write.");

	let path = "sample4.txt";
	let mut file = File::create(path) 
		.expect("file not found.");
	let s = "hello rust world.\n";
	for it in s.as_bytes() {
		// file.write(&[*it])
		// 	.expect("cannot write.");
		let ch = *it;
		let ary = [ch];
		file.write(&ary)
			.expect("cannot write.");
	}

	let path = "sample5.txt";
	let mut file = File::create(path)?;
	file.write(b"hello rust world.\n")?;
	Ok(())
}
