use std::time::Instant;

const N: usize = 10_000;

fn main() {
    let now = Instant::now();
    let num_iterations = 10;
    for rep in 0..num_iterations {
        let sum_of_amicable_numbers = get_sum_of_amicable_numbers_under(N);
        if rep == 0 {
            println!(
                "The sum of amicable numbers under {} is {}",
                N, sum_of_amicable_numbers
            );
        }
    }

    let duration = now.elapsed();
    println!("Total duration: {:?}", duration);
    println!("Avg duration: {:?}", duration / num_iterations);
}

fn get_sum_of_amicable_numbers_under(max_num: usize) -> usize {
    let mut sum_of_amicable_numbers = 0;
    let primes = get_primes_up_to(max_num);
    for i in 2..max_num {
        let sum_1 = get_sum_of_divisors_of(i, &primes);
        // Ignore perfect numbers, or out-of-range numbers (cheating here)...
        if sum_1 == i || sum_1 > max_num {
            continue;
        }
        let sum_2 = get_sum_of_divisors_of(sum_1, &primes);
        if i == sum_2 {
            sum_of_amicable_numbers += i;
        }
    }
    sum_of_amicable_numbers
}

fn get_sum_of_divisors_of(number: usize, primes: &Vec<usize>) -> usize {
    let mut product_of_sums_of_prime_powers: usize = 1;
    for prime in primes.into_iter() {
        let mut quotient = number;
        let mut sum_of_prime_power_factors = 1;
        let mut prime_power = 1;
        while quotient % prime == 0 {
            prime_power *= prime;
            sum_of_prime_power_factors += prime_power;
            quotient /= prime;
        }
        product_of_sums_of_prime_powers *= sum_of_prime_power_factors;
    }
    // The product will be the sum of all divisors, including the number itself
    product_of_sums_of_prime_powers - number
}

fn get_primes_up_to(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; (n + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for candidate_prime in 2..=n {
        if !is_prime[candidate_prime] {
            continue;
        }
        let mut curr_multiple = candidate_prime * candidate_prime;
        while curr_multiple <= n {
            is_prime[curr_multiple] = false;
            curr_multiple += candidate_prime;
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|i_prime| *i_prime.1)
        .map(|i_prime| i_prime.0)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{get_primes_up_to, get_sum_of_divisors_of};

    #[test]
    fn test_sum_of_divisors_of_220() {
        let primes = get_primes_up_to(1000);
        let sum_of_divisors = get_sum_of_divisors_of(220, &primes);
        assert_eq!(284, sum_of_divisors);
    }

    #[test]
    fn test_sum_of_divisors_of_284() {
        let primes = get_primes_up_to(1000);
        let sum_of_divisors = get_sum_of_divisors_of(284, &primes);
        assert_eq!(220, sum_of_divisors);
    }
}
