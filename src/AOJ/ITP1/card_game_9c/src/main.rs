enum Result {
    Taro,
    Hanako,
    Draw,
}

fn main() {
    let n = input();
    let result = games(n.parse().unwrap());
    result_disp(result);
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}

fn games(n: u64) -> (u64, u64) {
    let mut i = 0;
    let mut taro_point: u64 = 0;
    let mut hanako_point: u64 = 0;
    while i < n {
        let cards = input();
        let mut cards = cards.split_whitespace();

        let taro = cards.next().unwrap().to_string();
        let hanako = cards.next().unwrap().to_string();

        let result = get_point(&taro, &hanako);

        taro_point += result.0;
        hanako_point += result.1;

        i += 1;
    }

    (taro_point, hanako_point)
}

fn get_point(taro: &String, hanako: &String) -> (u64, u64) {
    match is_taro_win(taro, hanako) {
        Result::Taro   => return (3, 0),
        Result::Hanako => return (0, 3),
        Result::Draw   => return (1, 1),
    }
}

fn is_taro_win(taro: &String, hanako: &String) -> Result {
    if taro == hanako { return Result::Draw }

    let mut i: usize = 0;
    let taro_len = taro.chars().count();

    while i < taro_len {
        if &taro[i..(i+1)] > &hanako[i..(i+1)] {
            return Result::Taro 
        } else if &taro[i..(i+1)] < &hanako[i..(i+1)] {
            return Result::Hanako
        } else {
            // equle
        }
        i += 1;
    }
    // 文字数がTaroの方が短い場合はTaroの勝ち
    Result::Hanako
}

fn result_disp(result: (u64, u64)) {
    println!("{} {}", result.0, result.1);
}