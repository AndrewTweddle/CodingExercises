use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;
const NUM_DIVISORS: u64 = 500;
const PRIME_CUTOFF: u64 = 30;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let primes = get_primes_up_to(PRIME_CUTOFF);
        if rep == 0 {
            println!("# primes up to {} = {}", PRIME_CUTOFF, primes.len());
        }

        let mut triangular_num: u64 = 0;
        let mut divs_i = 1;

        for i in 1_u64.. {
            triangular_num += i;
            let divs_i_plus_1 = if i % 2 == 0 {
                count_divisors_using_prime_decomposition(i + 1, &primes)
            } else {
                // if i+1 is even, count divisors of (i+1)/2
                count_divisors_using_prime_decomposition(i.div_ceil(2), &primes)
            };
            // i and i+1 have no common factors except 1
            // (since common factors divide their difference, and the difference is 1).
            // So after first halving whichever of i and i + 1 is even, we can multiply
            // the number of divisors of each to get the number of divisors of their product
            if divs_i * divs_i_plus_1 > NUM_DIVISORS {
                if rep == 0 {
                    println!("T({}) = {}", i, triangular_num);
                    println!(
                        "Divisor counts: {} * {} = {}",
                        divs_i,
                        divs_i_plus_1,
                        divs_i * divs_i_plus_1
                    );
                }
                break;
            }
            divs_i = divs_i_plus_1;
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn get_primes_up_to(n: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (n + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for candidate_prime in 2..=n {
        if !is_prime[candidate_prime as usize] {
            continue;
        }
        let mut curr_multiple = candidate_prime * 2;
        while curr_multiple <= n {
            is_prime[curr_multiple as usize] = false;
            curr_multiple += candidate_prime;
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|i_prime| *i_prime.1)
        .map(|i_prime| i_prime.0 as u64)
        .collect()
}

fn count_divisors_using_prime_decomposition(mut n: u64, primes: &Vec<u64>) -> u64 {
    let mut div_count = 1;
    for &prime in primes {
        let mut prime_exponent = 0;
        while n % prime == 0 {
            n /= prime;
            prime_exponent += 1;
        }
        // prime^prime_count has prime_count+1 divisors (1, prime, prime^2, ..., prime^prime_count)
        let prime_power_divs = prime_exponent + 1;
        div_count *= prime_power_divs
    }
    if n != 1 {
        let rem_divs = count_divisors_without_primes(n);
        div_count *= rem_divs;
    }
    div_count
}

fn count_divisors_without_primes(n: u64) -> u64 {
    let floor_sqrt_n = int_sqrt(n);

    // All numbers up to PRIME_CUTOFF are decomposed into powers of the selected primes,
    // so none of them will be divisors of n, since n contains none of these primes as factors
    // EXCEPT for 1.
    let div_pair_count = ((PRIME_CUTOFF + 1)..=floor_sqrt_n)
        .filter(|i| i % 2 != 0 && n % i == 0)
        .count() as u64
        + 1;
    if floor_sqrt_n * floor_sqrt_n == n {
        // Don't double count the int_sqrt divisor...
        div_pair_count * 2 - 1
    } else {
        div_pair_count * 2
    }
}

// From https://en.wikipedia.org/wiki/Integer_square_root#Using_bitwise_operations
fn int_sqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let small_cand = int_sqrt(n >> 2) << 1;
    let large_cand = small_cand + 1;
    if large_cand * large_cand > n {
        small_cand
    } else {
        large_cand
    }
}
