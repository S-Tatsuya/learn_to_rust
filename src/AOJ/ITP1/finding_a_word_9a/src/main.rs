fn main() {
    let w = input();

    println!("{}", count_eq_word(&w));
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn count_eq_word(w: &String) -> i32 {
    let end_word = String::from("END_OF_TEXT");
    let mut count = 0;
    loop {
        let t = input();
        if check_string_eq(&t, &end_word) { break; } 
        let t = t.chars().map(|x| x.to_ascii_lowercase()).collect::<String>();

        let t_iter = t.split_whitespace();

        for word in t_iter {
            if check_string_eq(w, &word.to_string()) {
                count += 1;
            }  
        }
    }

    count
}

fn check_string_eq(w: &String, t: &String) -> bool {
    w == t
}