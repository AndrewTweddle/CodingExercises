use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start = Instant::now();

    let mut max_primes: usize = 0;
    let mut best_a: i32 = 0;
    let mut best_b: i32 = 0;
    for rep in 0..NUM_REPETITIONS {
        for b in 2..=1000_i32 {
            if !is_prime(b) {
                continue;
            }
            for a in -1000..=1000_i32 {
                let num_primes = (0..).take_while(|n| is_prime(n * n + a * n + b)).count();
                if num_primes > max_primes {
                    max_primes = num_primes;
                    best_a = a;
                    best_b = b;
                }
            }
        }

        if rep == 0 {
            println!("Product of coefficients: {}", best_a * best_b);
            println!("a = {}", best_a);
            println!("b = {}", best_b);
            println!("max primes in a row: {}", max_primes);
            let sign_before_a = if best_a < 0 { "" } else { "+ " };
            println!("equation: n^2 {}{} n + {}", sign_before_a, best_a, best_b);
        }
    }

    let avg_duration = start.elapsed() / NUM_REPETITIONS;
    println!("Average duration: {:?}", avg_duration);
}

fn is_prime(n: i32) -> bool {
    if n <= 1 {
        return false;
    }
    let sqrt_n: i32 = integer_sqrt(n as u32) as i32;
    (2..=sqrt_n).all(|q| n % q != 0)
}

// From https://en.wikipedia.org/wiki/Integer_square_root#Using_bitwise_operations
fn integer_sqrt(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    //  Recursive call:
    let small_cand = integer_sqrt(n >> 2) << 1;
    let large_cand = small_cand + 1;
    if large_cand * large_cand > n {
        small_cand
    } else {
        large_cand
    }
}
