fn main() {
    let mut result_vec: Vec<char> = Vec::new();
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let mut input = input.split_whitespace();
        let m: i32 = input.next().unwrap().parse().unwrap();
        let f: i32 = input.next().unwrap().parse().unwrap();
        let r: i32 = input.next().unwrap().parse().unwrap();

        if (m == -1) && (f == -1) && (r == -1) {
            break;
        }

        result_vec.push(get_score(m, f, r));
    }

    for i in result_vec {
        println!("{}", i);
    }
}

fn get_score(m: i32, f: i32, r:i32) -> char {
    let mut result = 'F';
    let total = m + f;
    if m == -1 || f == -1 {
        result = 'F';
    } else if 80 <= total {
        result = 'A';
    } else if 65 <= total && total < 80 {
        result = 'B';
    } else if 50 <= total && total < 65 {
        result = 'C';
    } else if 30 <= total && total < 50 {
        if 50 <= r {
            result = 'C';
        } else {
            result = 'D';
        }
    }
    result
}