fn main() {
	// func no argument no return
	no_param();

	// func argument no return
	one_param(10);
	two_param(10, 20);

	// func  argument return
	let ret = two_param_and_return(10, 20);
	println!("ret is {}", ret);

	// func argument str type
	str_param("rust");

	// func retrun str type
	let ret = str_param_and_return("rust");
	println!("ret is {}", ret);

	// func argumet vector
	let v = vec![1,2,3,4,5];
	let sum = vec_param(&v);
	println!("sum is {}", sum);

	// func return vector
	let v = vec_return(10);
	for i in v {
		print!("{} ", i);
	}
	println!("");

	// func changed vectro
	let mut v = vec![1,2,3,4,5];
	vec_change(&mut v);
	for i in v {
		print!("{} ", i);
	}
	println!("");
}

fn no_param() {
	println!("called no_param");
}

fn one_param(x: i32) {
	println!("called one_param, x is {}", x);
}

fn two_param(x: i32, y: i32) {
	println!("called two_param, {} and {}", x, y);
}

fn two_param_and_return(x: i32, y: i32) -> i32 {
	println!("called two_param_and_return, {} and {}", x, y);
	x + y
}

fn str_param(s: &str) {
	println!("called str_param, s is {}", s);
}

fn str_param_and_return(s: &str) -> String {
	println!("called str_param_and_return, s is {}", s);
	let ret = format!("hello {} world.", s);
	ret
}

fn vec_param(v: &Vec<i32>) -> i32 {
	println!("called vec_param");
	let mut sum = 0;
	for i in v {
		sum += i;
	}
	sum
}

fn vec_return(max: i32) -> Vec<i32> {
	println!("called vec_return");
	let mut v = Vec::new();
	for i in 0..max {
		v.push(i);
	}
	v
}

fn vec_change(v: &mut Vec<i32>) {
	println!("called vec_change");
	for i in v {
		*i = *i * 10;
	}
}
