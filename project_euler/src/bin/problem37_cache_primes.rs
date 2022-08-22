// Note: This solution pre-calculates a cache of primes up to some max number.
//       This is used for testing primality.
//
//       The primes are calculated using a modified prime sieve, but using bits not bools.
//       Furthermore, each pair of bits covers the next pair of numbers of the form 6k-1 and 6k + 1.
//       If this doesn't go high enough then the cache is recalculated with a max prime 10x higher.
//       This is probably the fastest prime sieve I've written yet (see problem10 for others).
//       It looks like it takes about 1/3 to 40% of the time of the next fastest.
//
//       HOWEVER, for problem 37, this consumes massive amounts of memory and time.
//       This is because the max prime needs to go up to 100's of billions.
//
//       You have been warned! THIS CODE SHOULD NOT ACTUALLY BE RUN!
//
//       This code is retained for instructive purposes, and because it contains
//       a nice variation on Eratosthenes' prime sieve.

// Observations:
// 1. The first and last digit must be prime, so they must come from {2, 3, 5, 7}
// 2. All digits but the first must be valid final digits of a multi-digit prime number.
//    So they must come from the set {1, 3, 7, 9}, otherwise they are divisible by 2 or by 5.
// 3. Combining 1 and 2, the final digit can only be 3 or 7.
// 4. The digits 1 and 7 are of the form 3k+1 (they are from the same class modulo 3).
//    The digits 3 and 9 are of the form 3k.
//    So, since the sum of the decimal digits of a number is divisible by 3 iff the number is also,
//    then at most 2 of the digits of a truncatable prime can be from the set {1, 7},
//    otherwise a truncated partial number on the right, involving 3 of those digits,
//    will be divisible by 3 (excluding the very first digit which could be 2 or 5).
// 5. A truncatable prime starting with 2 or 5 can only have 3 or 9 for all remaining digits
//    (and must consequently have 3 for the final digit, since 9 is not prime),
//    otherwise a truncated prime on the left involving 2 or 5, up to and including the first digit
//    that is not a multiple of 3, will have digits adding to a multiple of 3 and will not be prime.
//    But if all digits except the first are multiples of 3, then the truncated primes on the right
//    (that don't include the first digit) will be divisible by 3. So they will not be prime,
//    unless there is exactly 1 such digit and it is 3.
//    i.e. 23 and 53 are the only truncatable primes that start with a 2 or a 5.
// 6. All truncatable primes other than 23 and 53 must start with a 3 or a 7.
//
// In summary:
//    All truncatable primes other than 23 and 53 must start with a 3 or a 7, end in a 3 or a 7,
//    and the remaining digits must come from the set {1, 3, 7, 9}.

use std::collections::vec_deque::VecDeque;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 0;

fn main() {
    let mut start_time = Instant::now();
    for rep in 0..=NUM_REPETITIONS {
        let truncatable_primes = calculate_truncatable_primes();
        let sum_of_primes = truncatable_primes.iter().sum::<u64>();

        if rep == 0 {
            println!("Truncatable primes: {:?}", truncatable_primes);
            println!();
            println!("The sum of the truncatable primes is {}", sum_of_primes);
            println!("Duration (including printing): {:?}", start_time.elapsed());

            // Now perform further repetitions to calculate the avg duration,
            // excluding the time spent printing the solution (i.e. restart the clock after rep 0)
            start_time = Instant::now();
        }
    }

    if NUM_REPETITIONS > 0 {
        let avg_duration = start_time.elapsed() / NUM_REPETITIONS;
        println!(
            "Average duration over {} iterations: {:?}",
            NUM_REPETITIONS, avg_duration
        );
    }
}

struct Node {
    n: u64,
    sum_of_digits: u16,
    highest_digit_pow_10: u64,
    left_truncations: Vec<u64>,
}

impl Node {
    fn new(initial_digit: u8) -> Self {
        Node {
            n: initial_digit as u64,
            sum_of_digits: initial_digit as u16,
            highest_digit_pow_10: 1,
            left_truncations: vec![initial_digit as u64],
        }
    }
}

fn calculate_truncatable_primes() -> Vec<u64> {
    let mut truncatable_primes = Vec::<u64>::with_capacity(11);

    let mut prime_search = PrimeSearch::new(1000);

    // Add all truncatable primes beginning with 2 or 5 (see observation 5 above)
    truncatable_primes.push(23);
    truncatable_primes.push(53);

    // Do a breadth first search of possible candidates, from smallest to largest
    let mut bfs_queue = VecDeque::with_capacity(100);

    // The rightmost (units) digit can only be 3 or 7 - see observation 3 above
    bfs_queue.push_back(Node::new(3));
    bfs_queue.push_back(Node::new(7));

    while let Some(curr) = bfs_queue.pop_front() {
        let new_num_digits = curr.left_truncations.len() + 1;
        let new_digit_pow_10 = 10 * curr.highest_digit_pow_10;

        // prepend possible digits to the current candidate node
        for new_digit in [1, 3, 7, 9] {
            let new_sum_of_digits = curr.sum_of_digits + new_digit;
            if new_sum_of_digits % 3 == 0 {
                continue;
            }
            let new_n = curr.n + new_digit_pow_10 * (new_digit as u64);

            // The new candidate must itself be prime, since it will be
            // a partial number on the right in further attempts
            if !prime_search.is_prime(new_n) {
                continue;
            }

            // Form truncated primes on the left
            let mut new_left_truncs = Vec::<u64>::with_capacity(new_num_digits);
            new_left_truncs.push(new_digit as u64);
            let mut pow_10: u64 = 1;
            for i in 1..new_num_digits {
                pow_10 *= 10;
                let left_trunc = pow_10 * (new_digit as u64) + curr.left_truncations[i - 1];
                new_left_truncs.push(left_trunc);
            }

            // The leading digit of a truncatable prime must be 3 or 7 - see observation 6 above
            if new_digit == 3 || new_digit == 7 {
                // if all partial numbers on the left are prime, then this is a truncatable prime
                if new_left_truncs
                    .iter()
                    .skip(1)
                    .all(|&left_partial| prime_search.is_prime(left_partial))
                {
                    truncatable_primes.push(new_n);
                }
            }

            let new_node = Node {
                n: new_n,
                sum_of_digits: new_sum_of_digits,
                highest_digit_pow_10: new_digit_pow_10,
                left_truncations: new_left_truncs,
            };
            bfs_queue.push_back(new_node);
        }
    }

    truncatable_primes
}

struct PrimeSearch {
    max_prime: u64,
    primes: Vec<u64>,
}

impl PrimeSearch {
    fn new(max_prime: u64) -> Self {
        let primes = get_primes_with_truncated_prime_pattern_up_to(max_prime);
        Self { max_prime, primes }
    }

    fn is_prime(&mut self, n: u64) -> bool {
        if n > self.max_prime {
            #[cfg(debug_assertions)]
            println!("is_prime({})", n);

            // Calculate more primes if n is outside the range of primes last calculated
            if n > 10 * self.max_prime {
                self.max_prime = n;
            } else {
                self.max_prime = 10 * self.max_prime;
            }
            let primes = get_primes_with_truncated_prime_pattern_up_to(self.max_prime);
            self.primes = primes;
        }
        self.primes.binary_search(&n).is_ok()
    }
}

fn get_primes_with_truncated_prime_pattern_up_to(max_prime: u64) -> Vec<u64> {
    #[cfg(debug_assertions)]
    println!("get_primes_with_truncated_prime_pattern_up_to({})", max_prime);

    #[cfg(debug_assertions)]
    let start_time = Instant::now();

    // Use a bit vector to track which numbers of the form 6k-1 (odd bits) or 6k+1 (even bits)
    // are composite.
    let sieve_count = (((max_prime + 5) / 3 + 7) / 8) as usize;
    let mut is_composite: Vec<u8> = vec![0; sieve_count];
    let mut is_trunc_prime: Vec<u8> = vec![0; sieve_count];

    for i in 0.. {
        // Calculate the potential prime number corresponding to index i
        let prime: u64 = if i % 2 == 0 {
            3 * (i as u64) + 5
        } else {
            3 * (i as u64) + 4
        };

        if prime > max_prime {
            break;
        }

        // Skip if the number is marked as a composite number
        let byte_index = i / 8;
        let bit_index = i % 8;
        let bit_mask = 1 << bit_index;
        if is_composite[byte_index] & bit_mask != 0 {
            continue;
        }

        // mark all higher multiples as composite, starting with prime^2
        let mut composite = prime * prime;
        while composite <= max_prime {
            let div6 = composite / 6;
            let rem6 = composite % 6;
            composite += prime;
            let composite_index = match rem6 {
                1 => 2 * div6 - 1,
                5 => 2 * div6,
                _ => continue,
            } as usize;

            let byte_index = composite_index / 8;
            let bit_index = composite_index % 8;
            let bit_mask = 2 << bit_index;
            is_composite[byte_index] |= bit_mask;
        }

        if has_truncated_prime_pattern(prime) {
            is_trunc_prime[byte_index] |= bit_mask;
        }
    }

    let prime_count = is_trunc_prime.iter().map(|byte| byte.count_ones() as usize).sum::<usize>();
    let mut primes = Vec::with_capacity(prime_count + 1);

    // although 2 is prime, it is not in the pattern required of truncated primes, so exclude it
    primes.push(3);

    let mut prime: u64 = 5;
    for byte_index in 0..sieve_count {
        let byte = is_trunc_prime[byte_index];
        if byte == 0 {
            prime += 24;
            continue;
        }
        let mut bit_mask: u8 = 1;
        for _ in 0..4 {
            if byte & bit_mask != 0 {
                primes.push(prime);
            }
            prime += 2;
            bit_mask <<= 1;
            if byte & bit_mask != 0 {
                primes.push(prime);
            }
            prime += 4;
            bit_mask <<= 1;
        }
    }

    #[cfg(debug_assertions)]
    println!("  took {:?}", start_time.elapsed());

    primes
}

fn has_truncated_prime_pattern(mut prime: u64) -> bool {
    while prime > 0 {
        let next_digit = prime % 10;
        match next_digit {
            1 | 3 | 7 | 9 => prime /= 10,
            _ => return false,
        }
    }
    true
}
