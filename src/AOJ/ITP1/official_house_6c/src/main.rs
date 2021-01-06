fn main() {
	let mut offical_house = [[[0; 10]; 3]; 4];

	let mut input1 = String::new();
	std::io::stdin().read_line(&mut input1).unwrap();
	let input1: i32 = input1.trim().parse().unwrap();

	let mut n = 0;
	while n < input1 {
		let mut input2 = String::new();
		std::io::stdin().read_line(&mut input2).unwrap();
		let mut input2 = input2.split_whitespace();

		let b: usize = input2.next().unwrap().parse().unwrap();
		let f: usize = input2.next().unwrap().parse().unwrap();
		let r: usize = input2.next().unwrap().parse().unwrap();
		let v: i32 = input2.next().unwrap().parse().unwrap();

		offical_house[b - 1][f - 1][r - 1] += v;
		
		n += 1;
	}
	
	let mut count = 0;
	for building in offical_house.iter() {
		for floor in building.iter() {
			for room in floor.iter() {
				print!(" {}", room);
			}
			println!("");
		}
		if count <= 2 {
			println!("####################");
		}
		count += 1;
	}
}
