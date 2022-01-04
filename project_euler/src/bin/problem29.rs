use std::collections::HashSet;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let mut unique_powers = HashSet::<PrimeRepr>::new();
        for a in 2..=100 {
            let base_exps = get_prime_exponents(a);
            for b in 2..=100 {
                let mut prime_exps = new_zero_prime_repr();
                for i in 0..prime_exps.len() {
                    prime_exps[i] = base_exps[i] * b;
                }
                unique_powers.insert(prime_exps);
            }
        }

        if rep == 0 {
            println!("Count of unique powers: {}", unique_powers.len());
        }
    }

    let duration = start.elapsed() / NUM_REPETITIONS;
    println!("Avg duration of {} runs: {:?}", NUM_REPETITIONS, duration);
}

const PRIMES: [u32; 25] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

type PrimeRepr = [u32; 25];

fn new_zero_prime_repr() -> PrimeRepr {
    [0_u32; 25]
}

fn get_prime_exponents(mut a: u32) -> PrimeRepr {
    let mut exponents = new_zero_prime_repr();
    for i in 0..PRIMES.len() {
        let prime = PRIMES[i];
        let mut exponent = 0;
        while a % prime == 0 {
            a /= prime;
            exponent += 1;
        }
        exponents[i] = exponent;
    }
    exponents
}
