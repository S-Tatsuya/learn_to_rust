fn main() {
    loop {
        let card = input();
        if card == "-" { break; }
        let m: i32 = input().parse().unwrap();

        result_disp(&shuffle_run(&card, m));
    }
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn shuffle_run(card: &String, m: i32) -> String {
    let mut i = 0;
    let mut result = card.clone();
    while i < m {
        let h: usize = input().parse().unwrap();

        result = shuffle(&result, h);
        i += 1;
    }

    result
}

fn result_disp(result: &String) {
    println!("{}", result);
}

fn shuffle(card: &String, h: usize) -> String {
    let head = &card[0..h];
    let tail = &card[h..];

    [tail, head].join("")
}