use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let n = 600_851_475_143_u64;
    let max_prime_factor = get_max_prime_factor_by_dividing_by_odds(n);

    // Previous method is practically instantaneous, even when n is set to u64::MAX.
    // But the following method starts taking seconds for values around 1 million...
    /*
    let n = 1_000_001_u64;
    let max_prime_factor = get_max_prime_factor_using_compact_prime_sieve(n as u64);
    */

    println!("Max prime factor of {} is {}", n, max_prime_factor);
}

fn get_max_prime_factor_by_dividing_by_odds(n: u64) -> u64 {
    let mut quotient = n;
    let mut max_prime_factor = 1;

    if n % 2 == 0 {
        while quotient % 2 == 0 {
            quotient /= 2;
        }
        max_prime_factor = 2
    }

    let mut candidate_prime = 3;

    // if quotient is not prime, then its smallest prime factor is <= its square root
    while candidate_prime * candidate_prime <= quotient {
        if quotient % candidate_prime == 0 {
            max_prime_factor = candidate_prime;
            quotient /= candidate_prime;

            while quotient % max_prime_factor == 0 {
                quotient /= max_prime_factor;
            }
        }
        candidate_prime += 2; // try next odd number
    }

    if quotient != 1 {
        // quotient is prime and hence is the largest prime factor
        quotient
    } else {
        max_prime_factor
    }
}

#[allow(unused)]
fn get_max_prime_factor_using_compact_prime_sieve(n: u64) -> u64 {
    let primes = get_primes_using_compact_prime_sieve(n);
    *primes.iter().rev().find(|prime| n % *prime == 0).unwrap()
}

#[derive(Eq)]
struct PrimeMultiple {
    prime: u64,
    multiple: u64,
}

impl Ord for PrimeMultiple {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.multiple.cmp(&self.multiple) {
            Ordering::Equal => other.prime.cmp(&self.prime),
            ordering => ordering,
        }
    }
}

impl PartialOrd for PrimeMultiple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PrimeMultiple {
    fn eq(&self, other: &Self) -> bool {
        self.multiple == other.multiple
    }
}

#[allow(unused)]
fn get_primes_using_compact_prime_sieve(n: u64) -> Vec<u64> {
    let mut primes = Vec::<u64>::new();
    let mut next_prime_multiples = BinaryHeap::<PrimeMultiple>::new();

    if n < 2 {
        return primes;
    }

    primes.push(2);
    next_prime_multiples.push(PrimeMultiple {
        prime: 2,
        multiple: 4,
    });
    let mut min_prime_multiple = 4;
    let mut candidate_prime = 3;

    while candidate_prime <= n {
        if candidate_prime == min_prime_multiple {
            // advance all multiples of past primes that were reached by the candidate prime
            while candidate_prime == min_prime_multiple {
                {
                    let mut next_mult = next_prime_multiples.peek_mut().unwrap();
                    *next_mult = PrimeMultiple {
                        prime: next_mult.prime,
                        multiple: next_mult.multiple + next_mult.prime,
                    };
                }
                min_prime_multiple = next_prime_multiples.peek().unwrap().multiple;
            }
        } else {
            primes.push(candidate_prime);
            let next_multiple = candidate_prime * 2;
            if next_multiple < min_prime_multiple {
                min_prime_multiple = next_multiple;
            }
            next_prime_multiples.push(PrimeMultiple {
                prime: candidate_prime,
                multiple: next_multiple,
            });
        }
        candidate_prime += 1;
    }

    primes
}

// Following panics with out-of-memory error...
#[allow(unused)]
fn brute_force_sieve(n: u64) -> u64 {
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
        .rev()
        .filter(|i_prime| *i_prime.1 && (n % (i_prime.0 as u64) == 0))
        .map(|i_prime| i_prime.0)
        .next()
        .unwrap() as u64
}

#[test]
fn test_get_primes_using_compact_prime_sieve() {
    let primes = get_primes_using_compact_prime_sieve(25);
    assert_eq!(primes, vec![2, 3, 5, 7, 11, 13, 17, 19, 23]);
}

#[test]
fn test_get_prime_factor_of_a_large_prime() {
    // let n = 6_643_838_879;  // 7 ms
    // let n = 688_846_502_588_399;  // 157 ms
    let n = 2_305_843_009_213_693_951; // 8.519 seconds
    let max_prime_factor = get_max_prime_factor_by_dividing_by_odds(n);
    assert!(n == max_prime_factor);
}
