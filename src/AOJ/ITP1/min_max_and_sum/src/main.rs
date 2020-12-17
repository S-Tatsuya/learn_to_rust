fn main() {
	let mut input1 = String::new();
	std::io::stdin().read_line(&mut input1).unwrap();
	let mut input2 = String::new();
	std::io::stdin().read_line(&mut input2).unwrap();
	
	let input2 = input2.split_whitespace();
	
	let mut num_vec: Vec<i64> = Vec::new();
	for i in input2 {
		num_vec.push(i.parse().unwrap());
	}
	
	let mut min = num_vec[0];
	let mut max = num_vec[0];
	let mut sum = 0;
	for i in &num_vec {
		min = if min > *i { *i } else { min };
		max = if max < *i { *i } else { max };
		sum += i;
	}
	println!("{} {} {}", min, max, sum);
}
