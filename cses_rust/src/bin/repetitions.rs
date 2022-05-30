use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let stdin_lock = stdin.lock();
    let mut line_reader = stdin_lock.lines();
    let input_line = line_reader.next().unwrap().unwrap();
    let mut curr_char = ' ';
    let mut curr_len = 0;
    let mut max_len = 0;
    for ch in input_line.chars() {
        if ch == curr_char {
            curr_len += 1;
        } else {
            curr_char = ch;
            curr_len = 1;
        }
        if curr_len > max_len {
            max_len = curr_len;
        }
    }
    println!("{}", max_len);
}
