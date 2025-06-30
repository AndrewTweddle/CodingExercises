use num_bigint::BigUint;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let number = BigUint::new(vec![2_u32]).pow(1000);
        let sum_of_digits: u16 = number
            .to_radix_le(10)
            .iter()
            .map(|digit| *digit as u16)
            .sum();
        if rep == 0 {
            println!("Sum of digits = {}", sum_of_digits);
        }
    }

    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}
