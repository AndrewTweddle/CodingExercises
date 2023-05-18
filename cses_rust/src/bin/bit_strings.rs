use std::io::{stdin, BufRead};

const M: u32 = 10_u32.pow(9) + 7;

fn main() {
    // Note: 10^9 = (10^3)^3 < (2^10)^3 = 2^30, so 32 bits is enough
    let std_in = stdin();
    let stdin_lock = std_in.lock();
    let mut line_reader = stdin_lock.lines();
    let n = line_reader.next().unwrap().unwrap().parse::<u32>().unwrap();

    let mut r = 1;
    for _ in 0..n {
        r *= 2;
        if r > M {
            r -= M;
        }
    }
    println!("{r}");
}
