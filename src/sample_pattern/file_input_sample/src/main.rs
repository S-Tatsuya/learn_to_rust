
// ファイルのそのまま読み込む
/*
use std::fs;

fn main() -> Result<(), Box<std::error::Error>> {
    let content = fs::read_to_string("in.txt")?;
    println!("{}", content);
    Ok(())
}
*/

// 一行ずつ読み込む
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<std::error::Error>> {
    for result in BufReader::new(File::open("in.txt")?).lines() {
        let l = result?;
        println!("{}", l)
    }
    Ok(())
}