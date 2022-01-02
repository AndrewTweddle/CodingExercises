use std::time::Instant;

fn main() {
    let start = Instant::now();

    // There are 40 choose 20: 40 total moves (right or down), any 20 of them can be rightward
    let num_paths = choose(40, 20);
    println!("Number of paths: {}", num_paths);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

pub fn choose(n: u32, c: u32) -> u128 {
    if c > n {
        return 0;
    }

    let primes = get_primes_up_to(n);

    primes.iter().fold(1_u128, |combinatorial, prime| {
        let num_exp = count_prime_factors_of_factorial(n, *prime);
        let denom1_exp = count_prime_factors_of_factorial(c, *prime);
        let denom2_exp = count_prime_factors_of_factorial(n - c, *prime);
        let exponent = num_exp - denom1_exp - denom2_exp;
        let prime_power = (*prime as u128).pow(exponent);
        combinatorial
            .checked_mul(prime_power)
            .expect("Overflow while calculating choose")
    })
}

fn count_prime_factors_of_factorial(n: u32, prime: u32) -> u32 {
    let mut count = 0;
    let mut candidate_factors = n;
    while candidate_factors > 1 {
        // The # of the remaining integers in n! which have this prime as a prime factor
        // at least k times, where k corresponds (theoretically) to the loop index, starting from 1
        candidate_factors /= prime;
        count += candidate_factors;
    }
    count
}

fn get_primes_up_to(n: u32) -> Vec<u32> {
    let mut is_prime = vec![true; (n + 1) as usize];
    is_prime[0] = false;
    if n > 0 {
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
    }
    is_prime
        .iter()
        .enumerate()
        .filter(|i_prime| *i_prime.1)
        .map(|i_prime| i_prime.0 as u32)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod prime_utils {
        use super::get_primes_up_to;

        #[test]
        fn test_get_primes_up_to_0() {
            let primes = get_primes_up_to(1);
            let expected_primes = vec![];
            assert_eq!(primes, expected_primes);
        }

        #[test]
        fn test_get_primes_up_to_1() {
            let primes = get_primes_up_to(1);
            let expected_primes = vec![];
            assert_eq!(primes, expected_primes);
        }

        #[test]
        fn test_get_primes_up_to_10() {
            let primes = get_primes_up_to(10);
            let expected_primes = vec![2, 3, 5, 7];
            assert_eq!(primes, expected_primes);
        }
    }

    mod choose_tests {
        use super::choose;

        #[test]
        fn test_choose() {
            assert_eq!(choose(4, 0), 1);
            assert_eq!(choose(4, 1), 4);
            assert_eq!(choose(4, 2), 6);
            assert_eq!(choose(4, 3), 4);
            assert_eq!(choose(4, 4), 1);
        }

        #[test]
        fn test_choose_zero_from_zero() {
            assert_eq!(choose(0, 0), 1);
        }

        #[test]
        fn test_choose_large() {
            assert_eq!(choose(40, 20), 137846528820_u128);
        }

        #[test]
        #[should_panic]
        fn test_choose_very_large() {
            println!("250 choose 125 = {}", choose(250, 125));
        }
    }
}
