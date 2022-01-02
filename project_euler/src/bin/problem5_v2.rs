fn main() {
    let numbers: Vec<u64> = (2..=20).collect();
    let smallest_multiple = lcm_of_many(numbers).unwrap();
    println!("Smallest multiple of 2 to 20 is {}", smallest_multiple)
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

fn get_prime_exponents(n: u64, primes: &Vec<u64>) -> Vec<u32> {
    let num_primes = primes.len();
    let mut exponents = vec![0_u32; num_primes];
    if n <= 1 {
        return exponents;
    }
    let mut quotient = n;
    for i in 0..num_primes {
        let prime = primes[i];
        let mut power = 0;
        while quotient % prime == 0 {
            quotient /= prime;
            power += 1;
        }
        exponents[i] = power;
    }
    exponents
}

fn prime_exponents_to_value(primes: &Vec<u64>, exponents: &Vec<u32>) -> u64 {
    primes
        .iter()
        .zip(exponents)
        .map(|(prime, exponent)| {
            if *exponent == 0 {
                1_u64
            } else {
                prime.pow(*exponent)
            }
        })
        .product()
}

fn lcm_of_many(numbers: Vec<u64>) -> Option<u64> {
    numbers.iter().max().map(|max_number| {
        let primes = get_primes_up_to(*max_number);
        let one = get_prime_exponents(1, &primes);
        let lcm_exponents = numbers
            .iter()
            .map(|i| get_prime_exponents(*i, &primes))
            .fold(one, |accum, exponents| {
                accum
                    .iter()
                    .zip(exponents)
                    .map(|(prime, exponent)| *prime.max(&exponent))
                    .collect()
            });
        prime_exponents_to_value(&primes, &lcm_exponents)
    })
}
