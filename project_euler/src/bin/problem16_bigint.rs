use num_bigint::BigUint;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let number = BigUint::new(vec![2_u32]).pow(1000);
        let number_as_string = number.to_str_radix(10);
        let sum_of_digits = number_as_string
            .chars()
            .map(|ch| ch.to_digit(10).unwrap())
            .sum::<u32>();

        if rep == 0 {
            println!("Sum of digits in 2^1000: {}", sum_of_digits);
        }
    }

    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}
