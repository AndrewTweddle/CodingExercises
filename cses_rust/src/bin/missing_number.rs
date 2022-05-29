use std::io;
use std::io::{BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();

    let mut line_reader = stdin_lock.lines();
    let n = line_reader.next().unwrap().unwrap().parse::<u64>().unwrap();
    let nums_str = line_reader.next().unwrap().unwrap();
    let num_sum: u64 = nums_str
        .split_whitespace()
        .map(|num_str| num_str.parse::<u64>().unwrap())
        .sum();

    let missing_num = n * (n + 1) / 2 - num_sum;

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    writeln!(stdout_lock, "{} ", missing_num).unwrap();
}
