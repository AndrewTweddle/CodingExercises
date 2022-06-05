use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let mut line_reader = stdin_lock.lines();
    let _n = line_reader
        .next()
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .expect("Unable to parse n");
    let x_str = line_reader.next().unwrap().unwrap();
    let mut x_iter = x_str
        .split_ascii_whitespace()
        .map(|sub_str| sub_str.parse::<u32>().unwrap());
    let mut prev_x = x_iter
        .next()
        .expect("At least one x value should be provided");
    let mut move_count: u64 = 0;
    for x in x_iter {
        if x < prev_x {
            move_count += (prev_x - x) as u64;
        } else {
            prev_x = x;
        }
    }
    println!("{}", move_count);
}
