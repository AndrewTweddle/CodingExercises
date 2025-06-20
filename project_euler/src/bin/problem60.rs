use num::integer::Roots;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 10)
}

struct PrimeSet<const N: usize> {
    sets: Vec<[u64; N]>,
}

impl<const N: usize> PrimeSet<N> {
    fn new() -> Self {
        Self { sets: Vec::new() }
    }

    fn add_set(&mut self, set: &[u64], prime: u64) {
        let mut new_set: [u64; N] = [0; N];
        new_set[0..(N - 1)].copy_from_slice(set);
        new_set[N - 1] = prime;
        self.sets.push(new_set);
    }
}

struct PrimeSets {
    primes: Vec<u64>,
    primes_x2: PrimeSet<2>,
    primes_x3: PrimeSet<3>,
    primes_x4: PrimeSet<4>,
    exhausted: bool,
}

impl PrimeSets {
    fn new() -> Self {
        let mut prime_sets = Self {
            primes: Vec::with_capacity(10_000),
            primes_x2: PrimeSet::new(),
            primes_x3: PrimeSet::new(),
            primes_x4: PrimeSet::new(),
            exhausted: false,
        };
        prime_sets.primes.push(3);
        prime_sets
    }

    fn check_prime(&mut self, prime: u64, pow_of_10: u64, lowest_sum: &mut Option<u64>) {
        // See if there is any set of 4 that can be completed
        if let Some(min_sum) = lowest_sum {
            self.primes_x4
                .sets
                .retain(|set| set.iter().sum::<u64>() + prime < *min_sum);
        }

        for set in self.primes_x4.sets.iter() {
            if prime_expands_set(set, prime, pow_of_10) {
                *lowest_sum = Some(set.iter().sum::<u64>() + prime);
            }
        }

        // Eliminate any set of 3 that would be too large of completed with this prime and prime + 6
        if let Some(min_sum) = lowest_sum {
            self.primes_x3
                .sets
                .retain(|set| set.iter().sum::<u64>() + 2 * prime + 6 < *min_sum);
        }

        for set in self.primes_x3.sets.iter() {
            if prime_expands_set(set, prime, pow_of_10) {
                self.primes_x4.add_set(set, prime);
            }
        }

        // Eliminate sets of 2 that would be too large if extended by prime, prime + 6 and prime + 12
        if let Some(min_sum) = lowest_sum {
            self.primes_x2
                .sets
                .retain(|set| set.iter().sum::<u64>() + 3 * prime + 18 < *min_sum);
        }

        for set in self.primes_x2.sets.iter() {
            if prime_expands_set(set, prime, pow_of_10) {
                self.primes_x3.add_set(set, prime);
            }
        }

        // Exclude primes that would be too large if extended with prime and the next 3 candidates
        if let Some(min_sum) = lowest_sum {
            self.primes.retain(|p| p + 4 * prime + 36 < *min_sum);
        }

        for &p in self.primes.iter() {
            if is_prime(pow_of_10 * p + prime) && is_prime(calc_factor(p) * prime + p) {
                self.primes_x2.add_set(&[p], prime);
            }
        }

        // Add this prime to the primes, but only if it and the following 4 candidates
        // wouldn't exceed the lowest sum found this far
        match lowest_sum {
            Some(min_sum) => {
                if 5 * prime + 60 < *min_sum {
                    self.primes.push(prime);
                }
            }
            None => {
                self.primes.push(prime);
            }
        }

        if self.primes.is_empty()
            && self.primes_x2.sets.is_empty()
            && self.primes_x3.sets.is_empty()
            && self.primes_x4.sets.is_empty()
        {
            self.exhausted = true;
        }
    }
}

fn prime_expands_set(primes: &[u64], prime: u64, pow_of_10: u64) -> bool {
    let mut all_pairs = true;
    for &p in primes {
        if !is_prime(pow_of_10 * p + prime) {
            all_pairs = false;
            continue;
        }
        if !is_prime(calc_factor(p) * prime + p) {
            all_pairs = false;
            continue;
        }
    }
    all_pairs
}

fn calc_factor(mut n: u64) -> u64 {
    let mut factor = 1;
    while n > 0 {
        n /= 10;
        factor *= 10;
    }
    factor
}

fn solve() -> u64 {
    // We consider primes of the form 6n - 1 and 6n + 1 separately,
    // since they can never be paired together, otherwise the result will be divisible by 3.
    // This is because shifting one of the primes left does not change the sum of its digits,
    // and the (sum of the digits) % 3 == (6n +/- 1) % 3.
    let mut prime_sets_less_1 = PrimeSets::new();
    let mut prime_sets_plus_1 = PrimeSets::new();

    let mut lowest_sum: Option<u64> = None;
    let mut pow_of_10 = 10;

    // c is a candidate prime
    let mut c = 5;

    while !prime_sets_less_1.exhausted && !prime_sets_plus_1.exhausted {
        // Consider candidates that are 1 less than a multiple of 6
        if c >= pow_of_10 {
            pow_of_10 *= 10;
        }

        if is_prime(c) {
            prime_sets_less_1.check_prime(c, pow_of_10, &mut lowest_sum);
        }

        // Consider candidates that are 1 more than a multiple of 6
        c += 2;
        if c >= pow_of_10 {
            pow_of_10 *= 10;
        }

        if is_prime(c) {
            prime_sets_plus_1.check_prime(c, pow_of_10, &mut lowest_sum);
        }

        // Move on to the next multiple of 6, less 1
        c += 4;
    }
    lowest_sum.expect("There should be a valid set of 5 primes")
}

// n is an odd number greater than 7. Test whether it is prime.
fn is_prime(n: u64) -> bool {
    if n % 3 == 0 {
        return false;
    }

    let int_sqrt = n.sqrt();

    // We already know n is not divisible by 2 or 3.
    // If it is composite, then its prime factors must have the form 6i +/- 1.
    // So only check divisors (possibly composite) of that form.
    // Also, consider the smaller of a pair of divisors (possibly equal) that multiply to n.
    // It must be <= sqrt(n), so stop checking divisors above this number (except maybe 1)
    let max_multiple_of_6 = (int_sqrt + 1) / 6;

    for multiple_of_6 in 1..=max_multiple_of_6 {
        let mut factor = 6 * multiple_of_6 - 1;
        if n % factor == 0 {
            return false;
        }

        factor += 2;
        if n % factor == 0 {
            return false;
        }
    }
    true
}

fn solve_and_print_solution_and_time_more_runs_without_printing<S, T>(solve: S, repetitions: u32)
where
    S: Fn() -> T,
    T: std::fmt::Debug,
{
    use std::time::Instant;

    let mut start_time = Instant::now();
    for i in 0..=repetitions {
        let solution = solve();
        if i == 0 {
            println!("Solution: {solution:?}");
            println!(
                "Solved (including writing to terminal) in {:?}",
                start_time.elapsed()
            );

            // Now restart the timer, so that the timings don't include I/O...
            start_time = Instant::now();
        }
    }

    if repetitions > 0 {
        let avg_duration = start_time.elapsed() / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
    }
}
