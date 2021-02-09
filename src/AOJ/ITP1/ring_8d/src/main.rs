fn main() {
    let s = input();
    let p = input();

    if check_ring(&s, &p) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    
    input.trim().to_string()
}

fn check_ring(s: &String, p: &String) -> bool {
    let len = s.chars().count();
    let mut temp_s = s.clone();
    let mut i = 0;

    while i < len {
        if check_eq_string(&temp_s, &p) { return true }
        temp_s = change_string(&temp_s);
        i += 1;
    }

    false
}

fn change_string(s: &String) -> String {
    let head = &s[0..1];
    let tail = &s[1..];

    [tail, head].join("")
}

fn check_eq_string(s: &String, p: &String) -> bool {
    let len = p.chars().count();
    let mut i = 0;
    for char_s in s.chars() {
        if char_s.to_string() == &p[i..(i+1)] { i += 1; }
        else { i = 0; }

        if len == i { return true }
    }

    false
}