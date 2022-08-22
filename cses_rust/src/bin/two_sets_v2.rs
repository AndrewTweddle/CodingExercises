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
        let target = sum_of_ints / 2;
        for i in 1..=n {
            let sum_of_higher_values = (n - i) * (n + i + 1) / 2;
            let difference = target - sum_of_higher_values;
            if difference >= 0 {
                println!("YES");

                // set 1
                let set1_count = i - if difference == 0 { 0 } else { 1 };
                println!("{}", set1_count);

                for j in 1..i {
                    if j != difference {
                        print!("{} ", j);
                    }
                }
                println!("{}", i);

                // set 2
                let set2_count = n - i + if difference == 0 { 0 } else { 1 };
                println!("{}", set2_count);

                if difference != 0 {
                    print!("{} ", difference)
                }
                for j in (i + 1)..n {
                    print!("{} ", j);
                }
                println!("{}", n);
                break;
            }
        }
    } else {
        // An odd sum cannot be split into two equal parts
        println!("NO")
    }
}
