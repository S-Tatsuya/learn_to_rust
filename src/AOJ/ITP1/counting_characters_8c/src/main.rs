use std::collections::HashMap;

fn main() {
    let mut charactors = HashMap::<char, i32>::new();
    let charactors_list: [char; 26] =
         ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];

    init_hashmap(&mut charactors);

    loop {
        let (string, bytes) = input();

        if bytes == 0 { break; }

        count_chars(&mut charactors, string)
    }
    
    disp_hashmap(&charactors, &charactors_list);
}

fn count_chars(map: &mut HashMap::<char, i32>, string: String) {
    for c in string.trim().chars() {
        if !c.is_alphabetic() { continue; } 
        if c.is_uppercase() { 
            map.insert(c.to_ascii_lowercase(), map.get(&c.to_ascii_lowercase()).unwrap() + 1);
        } else {
            map.insert(c, map.get(&c).unwrap() + 1);
        } 
    }
}

fn input() -> (String, usize) {
    let mut input = String::new();
    let bytes = std::io::stdin().read_line(&mut input).unwrap();
    (input, bytes)
}

fn init_hashmap(map: &mut HashMap::<char, i32>) {
    // &map.entry('A').or_insert(0);
    // &map.entry('B').or_insert(0);
    // &map.entry('C').or_insert(0);
    // &map.entry('D').or_insert(0);
    // &map.entry('E').or_insert(0);
    // &map.entry('F').or_insert(0);
    // &map.entry('G').or_insert(0);
    // &map.entry('H').or_insert(0);
    // &map.entry('I').or_insert(0);
    // &map.entry('J').or_insert(0);
    // &map.entry('K').or_insert(0);
    // &map.entry('L').or_insert(0);
    // &map.entry('M').or_insert(0);
    // &map.entry('N').or_insert(0);
    // &map.entry('O').or_insert(0);
    // &map.entry('P').or_insert(0);
    // &map.entry('Q').or_insert(0);
    // &map.entry('R').or_insert(0);
    // &map.entry('S').or_insert(0);
    // &map.entry('T').or_insert(0);
    // &map.entry('U').or_insert(0);
    // &map.entry('V').or_insert(0);
    // &map.entry('W').or_insert(0);
    // &map.entry('X').or_insert(0);
    // &map.entry('Y').or_insert(0);
    // &map.entry('Z').or_insert(0);
    &map.entry('a').or_insert(0);
    &map.entry('b').or_insert(0);
    &map.entry('c').or_insert(0);
    &map.entry('d').or_insert(0);
    &map.entry('e').or_insert(0);
    &map.entry('f').or_insert(0);
    &map.entry('g').or_insert(0);
    &map.entry('h').or_insert(0);
    &map.entry('i').or_insert(0);
    &map.entry('j').or_insert(0);
    &map.entry('k').or_insert(0);
    &map.entry('l').or_insert(0);
    &map.entry('m').or_insert(0);
    &map.entry('n').or_insert(0);
    &map.entry('o').or_insert(0);
    &map.entry('p').or_insert(0);
    &map.entry('q').or_insert(0);
    &map.entry('r').or_insert(0);
    &map.entry('s').or_insert(0);
    &map.entry('t').or_insert(0);
    &map.entry('u').or_insert(0);
    &map.entry('v').or_insert(0);
    &map.entry('w').or_insert(0);
    &map.entry('x').or_insert(0);
    &map.entry('y').or_insert(0);
    &map.entry('z').or_insert(0);
}

fn disp_hashmap(map: &HashMap<char, i32>, key_list: &[char]) {
    for key in key_list {
        println!("{} : {}", key , map.get(&key).unwrap());
    }

}