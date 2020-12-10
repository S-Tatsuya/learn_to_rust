fn main() {
	let mut input_number = String::new();
	std::io::stdin().read_line(&mut input_number).unwrap();
	let mut number = input_number.split_whitespace();

	let a = number.next().unwrap().parse().unwrap();
	let b = number.next().unwrap().parse().unwrap();
	let c = number.next().unwrap().parse().unwrap();

	let numbers = sorting_three_numbers(a, b, c);
	println!("{} {} {}", numbers.0, numbers.1, numbers.2);
}

fn sorting_three_numbers(a: u32, b: u32, c: u32) -> (u32, u32, u32){
	let mut _numbers = [a, b, c];
	let mut min = _numbers[0];
	let mut mid = _numbers[0];
	let mut max = _numbers[0];

	for i in _numbers.iter() {
		if min > *i {
			mid = min;
			min = *i;
		} else if mid > *i {
			max = mid;
			mid = *i;
		} else if max > *i {
			mid = *i;
		} else { 
			mid = max;
			max = *i;
		} 
	}
	(min, mid, max)
}
