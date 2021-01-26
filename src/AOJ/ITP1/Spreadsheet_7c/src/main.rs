fn main() {
	let mut input = String::new();
	std::io::stdin().read_line(&mut input).unwrap();
	let mut input = input.split_whitespace();

	let r: i32 = input.next().unwrap().parse().unwrap();
	let c: i32 = input.next().unwrap().parse().unwrap();

	let mut result_vec: Vec<Vec<i32>> = Vec::new();

	for _i in 0..r {
		let mut input = String::new();
		std::io::stdin().read_line(&mut input).unwrap();
		let input = input.split_whitespace();

		let mut temp_vec: Vec<i32> = Vec::new();
		let mut sum = 0;
		for n in input {
			sum += n.parse::<i32>().unwrap();
			temp_vec.push(n.parse::<i32>().unwrap());
		}
		temp_vec.push(sum);

		result_vec.push(temp_vec)

	}

	result_vec.push(cal_sum(c, &result_vec));

	result_disp(&result_vec);
}

fn cal_sum(c: i32, vec1: &Vec<Vec<i32>>) -> Vec<i32> {
	let mut row_sum_vec: Vec<i32> = Vec::new();
	for i in 0..(c + 1) {
		row_sum_vec.push(0);
	}
	
	for temp_vec in vec1.into_iter() {
		let mut i = 0;
		for value in temp_vec {
			row_sum_vec[i] += value;
			i += 1;
		}
	}

	row_sum_vec
}

fn result_disp(vec1: &Vec<Vec<i32>>) {
	for temp_vec in vec1 {
		let mut flag = true;
		for value in temp_vec{
			if flag {
				print!("{}", value);
				flag = false;
			} else {
				print!(" {}", value);
			}
		}
		println!("");
	}
}
