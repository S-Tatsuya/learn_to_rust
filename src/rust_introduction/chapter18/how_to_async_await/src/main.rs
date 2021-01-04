use std::thread;
use std::time::Duration;

async fn foo(id: i32) {
	for i in 0..10 {
		println!("thread #{} count {}.", id, i);
		thread::sleep(Duration::from_millis(1000));
	}
}

// use tokio
// #[tokio::main] mainの前にasyncをつける
fn main() {
	// not use async
	// println!("program start.");
	// thread::spawn(|| {foo(10);}).join().unwrap();
	// thread::spawn(|| {foo(20);}).join().unwrap();
	// thread::spawn(|| {foo(30);}).join().unwrap();
	// println!("program end.")

	// use async and await, futures
	// let task = async {
	// 	foo(10).await;
	// 	foo(20).await;
	// 	foo(30).await;
	// };

	// println!("program start.");
	// futures::executor::block_on(sub());
	// println!("program end.");

	// use tokio pattern1
	// println!("program start.");
	// foo(10).await;
	// foo(20).await;
	// foo(30).await;
	// println!("program end.");

	// use tokio pattern2
	let mut rt = tokio::runtime::Runtime::new().unwrap();
	println!("prgram start.");
	rt.block_on(async {
		foo(10).await;
		foo(20).await;
		foo(30).await;
	}); 
	println!("program end.");
}

async fn sub() {
	foo(10).await;
	foo(20).await;
	foo(30).await;
}
