use make_to_package::random::Dice;

fn main() {
	let dice = Dice{max: 6,};
	let x = dice.get();
	println!("x is {}", x);


	let mut data = vec![0, 0, 0];
	dice.fill(&mut data);
	for i in data {
		println!("i is {}", i);
	}
}
