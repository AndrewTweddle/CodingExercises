use std::time::Instant;

const NUM_REPETITIONS: u32 = 10;

fn main() {
    let start_time = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let mut total: u64 = 0;
        for n in 1..1_000_000 {
            // Skip numbers ending in zero, as the reverse can't start with zero
            if n % 2 == 0 || n % 10 == 0 {
                continue;
            }
            let n_decimal = n.to_string();
            let n_dec_rev = n_decimal.as_str().chars().rev().collect::<String>();
            if n_decimal != n_dec_rev {
                continue;
            }

            let n_binary = format!("{:b}", n);
            let n_binary_rev = n_binary.as_str().chars().rev().collect::<String>();
            if n_binary != n_binary_rev {
                continue;
            }

            total += n as u64;
        }
        if rep == 0 {
            println!("Sum of double-base palindromes under 1 million = {}", total);
        }
    }

    let duration = start_time.elapsed() ;
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}
