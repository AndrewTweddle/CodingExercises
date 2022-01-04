use std::time::Instant;
use num_bigint::BigUint;

const NUM_REPETITIONS: u32 = 1000;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let mut factorial = BigUint::from(1_u32);
        for factor in 2..=100_u32 {
            factorial *= factor;
        }

        let sum_of_digits: u64 = factorial.to_radix_be(10).iter().map(|&digit| digit as u64).sum();

        if rep == 0 {
            println!("Factorial = {}", factorial);
            println!("Sum of digits of 100! = {}", sum_of_digits);
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
    println!("Avg Duration: {:?}", duration / NUM_REPETITIONS);
}
