use std::io;
use std::io::BufRead;

fn main() {
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    let mut line_iter = stdin_lock.lines();
    let n = line_iter
        .next()
        .unwrap()
        .unwrap()
        .parse::<u32>()
        .expect("Unable to read n");
    if n != 1 && n <= 3 {
        println!("NO SOLUTION");
    } else if n == 4 {
        println!("2 4 1 3");
    } else {
        // Do all odds (in sequence) then all evens if n is even. Else evens followed by odds.
        let starts = if n % 2 == 0 { [1, 2] } else { [2, 1] };
        for start in starts {
            for i in (start..n).step_by(2) {
                print!("{} ", i);
            }
        }
        println!("{}", n);
    }
}
