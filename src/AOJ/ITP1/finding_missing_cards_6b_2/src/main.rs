fn main() {
	let mut cards: [[bool; 13]; 4] = [[false; 13]; 4];
	let mut input1 = String::new();

	std::io::stdin().read_line(&mut input1).unwrap();
	let input1 = input1.trim().parse().unwrap();

	let mut count = 0;
	while count < input1 {
		count += 1;
		let mut input2 = String::new();
		std::io::stdin().read_line(&mut input2).unwrap();
		let mut input2 = input2.split_whitespace();

		let temp_1: char = input2.next().unwrap().chars().nth(0).unwrap();
		let temp_2: usize = input2.next().unwrap().parse().unwrap();

		if temp_1 == 'S' {
			cards[0][temp_2 - 1] = true;
		} else if temp_1 == 'H' {
			cards[1][temp_2 - 1] = true;
		} else if temp_1 == 'C' {
			cards[2][temp_2 - 1] = true;
		} else if temp_1 == 'D' {
			cards[3][temp_2 - 1] = true;
		}
	}

	let mut type_no = 0;
	let mut number = 0;
	for i in cards.iter() {
		for n in i.iter() {
			if *n == false {
				print_no(type_no, number);
			}
			number += 1;
		}
		type_no += 1;
		number = 0;
	}

}

fn print_no(type_no: i32, number: i32) {
	if type_no == 0 {
		println!("S {}", number + 1);
	} else if type_no == 1 {
		println!("H {}", number + 1);
	} else if type_no == 2 {
		println!("C {}", number + 1);
	} else if type_no == 3 {
		println!("D {}", number + 1);
	}
}
