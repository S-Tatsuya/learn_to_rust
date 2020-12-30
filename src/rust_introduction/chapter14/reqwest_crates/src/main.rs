use reqwest::StatusCode;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// nomal patten
	// let url = "http://openccpm.com/blog/";
	// println!("call {}", url);
	// let res = reqwest::get(url).await?;
	// let body = res.text().await?;
	// println!("response is \n{}", body);
	// Ok(())

	// query pattern
	// let n = 7;
	// let url = format!("http://openccpm.com/blog/?p={}", n);
	// println!("call {}", url);
	// let res = reqwest::get(&url).await?;
	// let body = res.text().await?;
	// println!("response is \n{}", body);
	// Ok(())

	// Error404 pattern
	// let url = "http://openccpm.com/unkown.txt";
	// println!("call {}", url);
	// let res = reqwest::get(url).await?;
	// match res.status() {
	// 	StatusCode::OK => {
	// 		let body = res.text().await?;
	// 		println!("response is \n{}", body);
	// 	}, 
	// 	StatusCode::NOT_FOUND => {
	// 		println!("error: target page not found.");
	// 	},
	// 	_ => {
	// 		println!("error: other error!!");
	// 	},
	// }
	// Ok(())

	// Error404 pattern2
	let url = "http://unknown.openccpm.com/blog";
	println!("call {}", url);
	if let Ok(res) = reqwest::get(url).await{
		let body = res.text().await?;
		println!("response is \n{}", body);
	} else {
		println!("error: not found!!");
	}
	Ok(())
}
