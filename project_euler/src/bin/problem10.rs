use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::Instant;

const BUFFER_SIZE: usize = 1024 * 1024;

fn main() {
    let n = 2_000_000 - 1;

    // Get answer using sieve of Eratosthenes
    let start = Instant::now();
    let primes = get_primes_up_to_by_eratosthenes(n);
    let sum: u64 = primes.iter().sum();
    println!("Sum of primes up to {} is {}", n, sum);
    let duration = start.elapsed();
    println!("Time elapsed (sieve of Eratosthenes): {:?}", duration);

    // In release mode, this takes 7.120721ms, compared to:
    // 1. around 15.723373ms seconds using the segmented sieve method
    //    with a sieve "buffer" of 1024 * 1024,
    //    or 511.948721ms with a buffer of 1024.
    // 2. around 187.886413ms seconds using the incremental sieve method.
    //
    // So, the segmented sieve performs similarly when given a much bigger buffer.

    // -------------------------------------------------------------------------
    // Get answer using segmented sieve...
    // See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve
    println!();
    let start = Instant::now();
    let primes = get_primes_up_to_using_segmented_sieve(n);
    let sum: u64 = primes.iter().sum();
    println!("Sum of primes up to {} is {}", n, sum);
    let duration = start.elapsed();
    println!("Time elapsed (segmented sieve): {:?}", duration);

    // -------------------------------------------------------------------------
    // Get answer using incremental sieve method..
    // See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Incremental_sieve
    println!();
    let start = Instant::now();
    let primes = get_primes_up_to_using_incremental_sieve(n);
    let sum: u64 = primes.iter().sum();
    println!("Sum of primes up to {} is {}", n, sum);
    let duration = start.elapsed();
    println!("Time elapsed (incremental sieve): {:?}", duration);
}

// -----------------------------------------------------
// Sieve of Eratosthenes.
// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes

fn get_primes_up_to_by_eratosthenes(n: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (n + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for candidate_prime in 2..=n {
        if !is_prime[candidate_prime as usize] {
            continue;
        }
        let mut curr_multiple = candidate_prime * candidate_prime;
        if curr_multiple > n {
            break;
        }
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

// -----------------------------------------------------
// Incremental sieve of Eratosthenes (using Priority Queue)
// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Incremental_sieve

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

fn get_primes_up_to_using_incremental_sieve(n: u64) -> Vec<u64> {
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
            let next_multiple = candidate_prime * candidate_prime;
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

// -----------------------------------------------------
// Segmented sieve of Eratosthenes
// See https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Segmented_sieve

pub struct PrimeIter {
    range_start: u64,
    range_size: usize,
    next_offset: usize,
    primes: Vec<u64>,
    is_prime: Vec<bool>,
}

impl PrimeIter {
    pub fn new(range_size: usize) -> Self {
        PrimeIter {
            range_start: 2,
            range_size,
            next_offset: 0,
            primes: Vec::new(),
            is_prime: vec![true; range_size],
        }
    }

    fn advance_next_offset(&mut self) {
        self.next_offset += 1;
        if self.next_offset == self.range_size {
            self.next_offset = 0;
            self.range_start += self.range_size as u64;
            self.is_prime.fill(true);
            for prime in &self.primes {
                let next_multiple = ((self.range_start - 1) / prime + 1) * prime;

                // Calling self.mark_prime_multiples(...) directly doesn't work here, since self
                // must be mutable, but it has been borrowed immutably through self.primes...
                mark_prime_multiples(
                    &mut self.is_prime,
                    self.range_start,
                    self.range_size,
                    *prime,
                    next_multiple,
                );
            }
        }
    }

    fn mark_prime_multiples(&mut self, prime: u64, starting_multiple: u64) {
        mark_prime_multiples(
            &mut self.is_prime,
            self.range_start,
            self.range_size,
            prime,
            starting_multiple,
        );
    }
}

fn mark_prime_multiples(
    is_prime: &mut [bool],
    range_start: u64,
    range_size: usize,
    prime: u64,
    starting_multiple: u64,
) {
    let mut mul_offset = (starting_multiple - range_start) as usize;
    while mul_offset < range_size {
        is_prime[mul_offset] = false;
        mul_offset += prime as usize;
    }
}

impl Iterator for PrimeIter {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.is_prime[self.next_offset] {
            self.advance_next_offset();
        }
        let prime = self.range_start + self.next_offset as u64;
        self.primes.push(prime);
        self.mark_prime_multiples(prime, prime * prime);
        self.advance_next_offset();
        Some(prime)
    }
}

fn get_primes_up_to_using_segmented_sieve(n: u64) -> Vec<u64> {
    let prime_iter = PrimeIter::new(BUFFER_SIZE);
    prime_iter.take_while(|&prime| prime <= n).collect()
}

#[test]
fn test_prime_iter() {
    let prime_iter = PrimeIter::new(4);
    let mut primes = prime_iter.take(22);

    // let expected_primes: [u64; 4] = [2, 3, 5, 7];
    let expected_primes: [u64; 22] = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79,
    ];

    for prime in &expected_primes {
        let next_prime = primes.next();
        assert_eq!(next_prime, Some(*prime));
    }
    assert_eq!(primes.next(), None);
}
