use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let stdin_lock = stdin.lock();
    let mut lines = stdin_lock.lines();
    let n = lines
        .next()
        .unwrap()
        .unwrap()
        .parse::<i64>()
        .expect("A positive integer is expected on the first line");
    let sum_of_ints = n * (n + 1) / 2;
    if sum_of_ints % 2 == 0 {
        let mut target = sum_of_ints / 2;
        let sets: (Vec<_>, Vec<_>) = (1..=n).rev().partition(|&i| {
            if i <= target {
                target -= i;
                true
            } else {
                false
            }
        });
        assert!(target == 0);
        println!("YES");
        print_set(&sets.0);
        print_set(&sets.1);
    } else {
        // An odd sum cannot be split into two equal parts
        println!("NO")
    }
}

fn print_set(set: &Vec<i64>) {
    println!("{}", set.len());
    print!("{}", set.last().unwrap());
    for i in set.iter().rev().skip(1) {
        print!(" {}", i);
    }
    println!();
}
