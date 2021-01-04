use std::thread;
use std::time::Duration;
use std::io::Read;

fn main() {
	// input end timing
	// let handle = thread::spawn(|| {
	// 	for i in 0..10 {
	// 		println!("thread #1 count {}.", i);
	// 		thread::sleep(Duration::from_millis(1000));
	// 	}
	// });
	// println!("press ENTER KEY");
	// std::io::stdin().read(&mut [0]);
	// println!("program end.");

	// thread endo timing
	// let handle = thread::spawn(|| {
	// 	for i in 0..10 {
	// 		println!("thread #1 count {}", i);
	// 		thread::sleep(Duration::from_millis(1000));
	// 	}
	// });
	// println!("please wait.");
	// handle.join().unwrap();
	// println!("program end.");
	
	// three thread pattan
	// let handle = thread::spawn(|| {
	// 	for i in 0..10 {
	// 		println!("thread #1 coutn {}", i);
	// 		thread::sleep(Duration::from_millis(1000));
	// 	} 
	// });
	// let handle2 = thread::spawn(|| {
	// 	for i in 0..10 {
	// 		println!("thread #2 coutn {}", i);
	// 		thread::sleep(Duration::from_millis(2000));
	// 	} 
	// });
	// let handle3 = thread::spawn(|| {
	// 	for i in 0..10 {
	// 		println!("thread #3 coutn {}", i);
	// 		thread::sleep(Duration::from_millis(1500));
	// 	} 
	// });
	// println!("please wait.");
	// handle.join().unwrap();
	// handle2.join().unwrap();
	// handle3.join().unwrap();
	// println!("program end.");

	// other closure
	// let task = || {
	// 	for i in 0..10 {
	// 		println!("thread #1 count {}.", i);
	// 		thread::sleep(Duration::from_millis(1000));
	// 	}
	// };

	// let handle = thread::spawn(task);
	// println!("please wait.");
	// handle.join().unwrap(); 
	// println!("program end.");

	// function thread
	let handle = thread::spawn(task);
	println!("please wait.");
	handle.join().unwrap();
	println!("program end.");
}

fn task() {
	for i in 0..10 {
		println!("thread #1 count {}.", i);
		thread::sleep(Duration::from_millis(1000));
	}
}
