fn main() {
    let n: usize = 5;
    let w: usize = 10;
    let a: [usize; 5] = [1, 2, 4, 5, 11];

    let mut exist = false;
    let mut bit = 0;
    while bit < n {
        let mut sum: usize = 0;
        let mut i: usize = 0;
        while i < n {
            if 0 != (bit & (1 << i)) {
                println!("match: {}", i);
                sum += a[i];
            }
            i += 1;
        }

        println!("{}, {}", bit , sum);
        if sum == w {exist = true;}
        bit += 1;
    }
    println!("{}", exist);
}