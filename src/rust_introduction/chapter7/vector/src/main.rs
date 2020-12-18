fn main() {
	let v = vec![1,2,3,4,5];
	println!("v[0] is {}", v[0]);
	println!("v[4] is {}", v[4]);
	println!("v.len is {}", v.len());

	// vector get() method
	let v = vec![1, 2, 3, 4, 5];
	println!("v.get(0) is {:?}", v.get(0));
	println!("v.get(4) is {:?}", v.get(4));
	println!("v.get(0) is {}", v.get(0).unwrap());
	println!("v.get(4) is {}", v.get(4).unwrap());

	// vector string
	let v = vec!["one", "two", "three", "four", "five"];
	println!("v[0] is {}", v[0]);
	println!("v[4] is {}", v[4]);
	println!("v.len is {}", v.len());

	// vector increase decrease
	let mut v: Vec<i32> = Vec::new();
	println!("v.len is {}", v.len());
	v.push(10);
	v.push(20);
	v.push(30);
	println!("pop is {:?}", v.pop());
	println!("pop is {:?}", v.pop());
	println!("pop is {:?}", v.pop());
	println!("v.len is {}", v.len());

	// vector reference
	let v = vec![1, 2, 3, 4, 5];
	println!("v.first is {:?}", v.first());
	println!("v.las is {:?}", v.last());
	println!("v.get(1) is {:?}", v.get(1));
	println!("v.get(10) is {:?}", v.get(10));
	println!("v.first is {}", v.first().unwrap());
	println!("v.last is {}", v.last().unwrap());

	// vectro delete
	let mut v = vec![1, 2, 3, 4, 5];
	println!("v.first is {:?}", v.first());
	println!("v.remove(0) is {:?}", v.remove(0));
	println!("v.first is {:?}", v.first());

	// vector insert()
	println!("v.first is {:?}", v.first());
	v.insert(0 , 10);
	println!("v.first is {:?}", v.first());
	v.insert(v.len(), 99);
	println!("v.last is {:?}", v.last());

	// vector concat()
	let a = vec![10, 20, 30];
	let b = vec![40, 50];
	let v = [a, b].concat();
	println!("v.len is {}", v.len());
	for i in v {
		print!("{} ", i);
	}
	println!("");

	// vector join()
	let v = vec!["one", "two", "three", "four", "five"];
	let x = v.join("-");
	println!("x is {}", x);

	// vector split()
	let s = "one,two,three,four,five";
	let v = s.split(',');
	for x in v {
		print!("{} ", x);
	}
	println!("");

	// vector sort()
	let mut v = vec!["one", "two", "three", "four", "five"];
	v.sort();
	let x = v.join(" ");
	println!("x is {}", x);
	v.reverse();
	let x = v.join(" ");
	println!("x is {}", x);
}
