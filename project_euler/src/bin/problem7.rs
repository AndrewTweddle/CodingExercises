use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn main() {
    let n = 10_001;
    let nth_prime = get_nth_prime(n).unwrap();
    println!("The {}-th prime is {}", n, nth_prime);
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
            ordering @ _ => ordering,
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

fn get_primes_up_to_nth_prime(n: u64) -> Vec<u64> {
    let mut primes = Vec::<u64>::new();
    if n == 0 {
        return primes;
    }

    // Seed data structures with the first prime...
    primes.push(2);
    let mut prime_count = 1;

    let mut next_prime_multiples = BinaryHeap::<PrimeMultiple>::new();
    next_prime_multiples.push(PrimeMultiple {
        prime: 2,
        multiple: 4,
    });

    let mut min_prime_multiple = 4;
    let mut candidate_prime = 3;

    while prime_count < n {
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
            prime_count += 1;
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

fn get_nth_prime(n: u64) -> Option<u64> {
    let primes = get_primes_up_to_nth_prime(n);
    primes.iter().rev().next().cloned()
}

#[test]
fn test_get_primes() {
    // Looked up on https://en.wikipedia.org/wiki/List_of_prime_numbers
    let expected_primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];
    let n = expected_primes.len() as u64;
    assert_eq![get_primes_up_to_nth_prime(n), expected_primes];
}

#[test]
fn test_nth_prime() {
    // Looked up on https://en.wikipedia.org/wiki/List_of_prime_numbers
    let n = 41;
    assert_eq!(get_nth_prime(n), Some(179));
}

#[test]
fn test_zeroth_prime() {
    let n = 0;
    assert_eq!(get_nth_prime(n), None);
}

#[test]
fn test_first_prime() {
    let n = 1;
    assert_eq!(get_nth_prime(n), Some(2));
}
