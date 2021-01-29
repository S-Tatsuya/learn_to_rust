fn main() {
	loop {
		let data = input1();

		if data.trim()	== "0" {
			break;
		}
		
		let mut sum = 0;
		for x in data.trim().chars() {
			sum += x.to_string().parse::<u64>().unwrap();
			// println!("{}, {}", x, sum);
		}
			
		println!("{}", sum);
	}
}

fn input1() -> String{
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	
	input
}
