use std::time::Instant;

// See https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple.
// Pythagorean triples (a, b, c) with integer a, b and c have the form:
//  (k(m^2-n^2), 2kmn, k(m^2+n^2))
// where k,m and n are positive integers with m > n, gcd(m, n) == 1, and m and n not both odd.
// and k = gcd(a, b, c).
// The perimeter is then p = a + b + c = 2km^2 + 2kmn = 2km(m+n)

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let mut start_time = Instant::now();

    for rep in 0..=NUM_REPETITIONS {
        let mut best_p = 0;
        let mut best_soln_count = 0;

        for p in (2..1000).step_by(2) {
            let mut soln_count = 0;
            for m in 2..(p / 2) {
                let mut dividend = p / 2;
                if dividend % m == 0 {
                    dividend /= m;
                    // n < m, so m + n < 2m
                    for m_plus_n in (m + 1)..(2 * m) {
                        if m % 2 != 0 && m_plus_n % 2 == 0 {
                            // m and n can't both be odd
                            continue;
                        }
                        if dividend % m_plus_n == 0 {
                            if gcd(m_plus_n, m) == 1 {
                                // m and n must be relatively prime
                                soln_count += 1;
                            }
                        }
                    }
                }
            }

            if soln_count > best_soln_count {
                best_soln_count = soln_count;
                best_p = p;

                if rep == 0 {
                    println!("p = {} has {} solutions", p, soln_count);
                }
            }
        }

        if rep == 0 {
            println!("Best p: {}", best_p);
            println!("Solutions: {}", best_soln_count);
            println!("Duration: {:?}", start_time.elapsed());

            // Restart the timer, so that further repetitions don't count the print statements
            start_time = Instant::now();
        }
    }

    let duration = start_time.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn gcd(larger: u32, smaller: u32) -> u32 {
    if smaller == 0 {
        larger
    } else {
        let remainder = larger % smaller;
        gcd(smaller, remainder)
    }
}

/*
 * Initially I tried building the prime factorization of all even numbers up to 1000.
 * However, this took 12µs, which is slower than the second approach to solving this problem.
 * So I didn't continue with looking for a way of counting suitable factorizations of
 * possible perimeters into 2km(m+n), as the code would have become quite tricky.
 *
use std::fmt::{Debug, Formatter};

#[derive(Copy, Clone, Debug)]
struct PrimeFactor {
    prime: u16,
    exponent: u8,
}

#[derive(Copy, Clone)]
struct PrimeFactorization {
    prime_count: usize,
    factors: [PrimeFactor; 4],  // 2 * 3 * 5 * 7 * 11 > 1000 => at most 4 unique prime factors
}

impl Debug for PrimeFactorization {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimeFactorization")
            .field("factors", &&self.factors[0..self.prime_count])
            .finish()
    }
}

impl PrimeFactorization {
    fn new() -> Self {
        PrimeFactorization {
            prime_count: 0,
            factors: [PrimeFactor { prime: 0, exponent: 1}; 4],
        }
    }

    fn add_factor(&mut self, prime: u16) {
        self.factors[self.prime_count].prime = prime;
        self.prime_count += 1;
    }

    fn update_exponent(&mut self, exponent: u8) {
        if self.prime_count == 0 {
            panic!("There is no prime factor to update the exponent of");
        }
        self.factors[self.prime_count - 1].exponent = exponent;
    }
}

fn main() {
    let mut start_time = Instant::now();

    for rep in 0..=NUM_REPETITIONS {
        // We only need to store factorizations for even numbers, since perimeters must be even.
        // So store the factorization for n in slot n/2...
        let mut factorizations = [PrimeFactorization::new(); 501];

        // For all odd primes, we don't need to factorize the prime itself,
        // since we only care about factorizations of even numbers.
        // But 2 is an even prime, so add the following special case logic for 2...
        factorizations[1].add_factor(2);

        for prime in 2_u16..=1000 {
            if is_composite[prime as usize] {
                continue;
            }
            for i in ((2*prime)..=1000).step_by(prime as usize) {
                // Sieve of Eratosthenes logic (to help identify further primes)...
                is_composite[i as usize] = true;

                // Add a prime factor, but only for even numbers, as perimeters must be even...
                if i % 2 == 0 {
                    factorizations[(i / 2) as usize].add_factor(prime);
                }
            }
            let mut prime_power = prime as u32;
            for exponent in 2.. {
                prime_power *= prime as u32;
                if prime_power > 1000 {
                    break;
                }
                for j in (prime_power..=1000).step_by(prime_power as usize) {
                    if j % 2 == 0 {
                        factorizations[(j / 2) as usize].update_exponent(exponent);
                    }
                }
            }
        }
...
        if rep == 0 {
            println!("{:#?}", factorizations);
...
 */
