fn main() {
	let mut input1 = String::new();
	std::io::stdin().read_line(&mut input1).unwrap();
	let mut input1 = input1.split_whitespace();
	let n: usize = input1.next().unwrap().parse().unwrap();
	let m: usize = input1.next().unwrap().parse().unwrap();

	let mut n_m_array = [[0; 100]; 100];
	let mut m_1_array = [0; 100];

	let mut x: usize = 0;
	while x < n {
		let mut input2 = String::new();
		std::io::stdin().read_line(&mut input2).unwrap();
		let input2 = input2.split_whitespace();

		let mut y: usize = 0;
		for i in input2 {
			n_m_array[x][y] = i.parse().unwrap();
			y += 1;
		}
		x += 1;
	}

	let mut x: usize = 0;
	while x < m {
		let mut input2 = String::new();
		std::io::stdin().read_line(&mut input2).unwrap();
		let input2: i32 = input2.trim().parse().unwrap();

		m_1_array[x] = input2;
		x += 1;
	}

	let mut result = [0; 100];
	let mut x: usize = 0;
	while x < n {
		let mut y: usize = 0;
		while y < m {
			result[x] += n_m_array[x][y] * m_1_array[y];
			y += 1;
		} 
		println!("{}", result[x]);
		x += 1;
	}
}
