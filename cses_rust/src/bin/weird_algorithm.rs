use std::io;
use std::io::{BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();

    let mut line_reader = stdin_lock.lines();
    let mut n = line_reader.next().unwrap().unwrap().parse::<u128>().unwrap();

    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();

    while n != 1 {
        write!(stdout_lock, "{} ", n).unwrap();
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    writeln!(stdout_lock, "{} ", n).unwrap();
}
