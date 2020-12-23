use linked_hash_map::LinkedHashMap;

fn main() {
	let mut trump_map = LinkedHashMap::new();

	let v = ["S 1", "S 2", "S 3", "S 4", "S 5", "S 6", "S 7", "S 8", "S 9", "S 10", "S 11", "S 12", "S 13",
			 "H 1", "H 2", "H 3", "H 4", "H 5", "H 6", "H 7", "H 8", "H 9", "H 10", "H 11", "H 12", "H 13",
			 "C 1", "C 2", "C 3", "C 4", "C 5", "C 6", "C 7", "C 8", "C 9", "C 10", "C 11", "C 12", "C 13",
			 "D 1", "D 2", "D 3", "D 4", "D 5", "D 6", "D 7", "D 8", "D 9", "D 10", "D 11", "D 12", "D 13",];
	
	for i in v.iter() {
		trump_map.entry(i.to_string()).or_insert(0);
	}

	let mut input1 = String::new();
	std::io::stdin().read_line(&mut input1).unwrap();
	let input1 = input1.trim().parse().unwrap();

	let mut input_vector: Vec<String> = Vec::new();

	for _i in 0..input1 {
		let mut input2 = String::new();
		std::io::stdin().read_line(&mut input2).unwrap();
		input_vector.push(input2.trim().to_string());

	}

	for i in input_vector.iter() {
		trump_map.insert(i.to_string(), 1);
	}

	let mut result_vector: Vec<String> = Vec::new();
	for (key, value) in trump_map {
		if value == 0 {
			result_vector.push(key);
		}	
	}

	for i in result_vector.iter() {
		println!("{}", i);
	}

}

