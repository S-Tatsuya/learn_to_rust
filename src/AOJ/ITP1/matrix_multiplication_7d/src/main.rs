fn main() {
	let (n, m, l) = input1();

	let first_vec = input2(n);
	let second_vec = input2(m);

	let result_vec = cal(&first_vec, &second_vec, l);

	disp_result(&result_vec);
}

fn input1() -> (u64, u64, u64) {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let mut input = input.split_whitespace();

	let data1 = input.next().unwrap().parse().unwrap();
	let data2 = input.next().unwrap().parse().unwrap();
	let data3 = input.next().unwrap().parse().unwrap();

	(data1, data2, data3)
}

fn input2(n: u64) -> Vec<Vec<u64>> {
	let mut return_vec: Vec<Vec<u64>> = Vec::new();
	for _i in 0..n {
		let mut temp_vec: Vec<u64> = Vec::new();
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		let input = input.split_whitespace();

		for value in input {
			temp_vec.push(value.parse::<u64>().unwrap());
		}

		return_vec.push(temp_vec);
	}

	return_vec
}

fn cal(vec1: &Vec<Vec<u64>>, vec2: &Vec<Vec<u64>>, l: u64) -> Vec<Vec<u64>> {
	let mut return_vec: Vec<Vec<u64>> = Vec::new();
	for sub_vec1 in vec1 {
		let mut y: usize = 0;
		let mut temp_vec: Vec<u64> = Vec::new();
		for _i in 0..l {
			let mut x: usize = 0;
			let mut result = 0;
			for sub_vec2 in vec2 {
				result += sub_vec1[x] * sub_vec2[y];
				x += 1;
			}
			temp_vec.push(result);
			y += 1;
		}
		
		return_vec.push(temp_vec);
	}

	return_vec
}

fn disp_result(vec1: &Vec<Vec<u64>>) {
	for sub_vec in vec1 {
		let mut is_first = true;
		for value in sub_vec {
			if is_first {
				print!("{}",value);
				is_first = false;
			} else {
				print!(" {}", value);
			}
		}
		println!("");
	}
}
