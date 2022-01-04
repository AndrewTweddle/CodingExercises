use num_bigint::BigUint;
use std::collections::HashSet;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let mut unique_powers = HashSet::<BigUint>::new();
        for a in 2..=100_u8 {
            let mut pow = BigUint::from(a);
            for _b in 2..=100 {
                pow *= a;
                unique_powers.insert(pow.clone());
            }
        }

        if rep == 0 {
            println!("Count of unique powers: {}", unique_powers.len());
        }
    }

    let duration = start.elapsed() / NUM_REPETITIONS;
    println!("Avg duration of {} runs: {:?}", NUM_REPETITIONS, duration);
}
