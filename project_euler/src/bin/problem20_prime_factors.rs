use std::time::Instant;

const NUM_REPETITIONS: u32 = 1_000;

/* Choose a base B which is the largest power of 10 such that (B-1)^2 + (B-1) < 2^128
 * A number in this base will have its digits stored in a Vec<u128>, lowest digit first.
 * The base is chosen to allow two numbers to be multiplied safely without overflowing
 * the u128 during the standard operation of multiplying each pair of digits,
 * and adding to any existing digit in that position in the product.
 */
const BASE_EXPONENT: u32 = 19;
const BASE: u128 = 10_u128.pow(BASE_EXPONENT); // 10_000_000_000_000_000_000;

fn main() {
    let start = Instant::now();
    let n: u8 = 100;

    for rep in 0..NUM_REPETITIONS {
        let factorial = get_factorial(n);
        if rep == 0 {
            println!("Duration until factorial calculated: {:?}", start.elapsed());
            print!("100! = {}", factorial.last().unwrap());

            for digit in factorial.iter().rev().skip(1) {
                print!("{:>0width$}", digit, width = BASE_EXPONENT as usize);
            }
            println!();
            println!("Duration until factorial printed: {:?}", start.elapsed());
        }

        let mut sum_of_digits: u16 = 0;
        for digit in &factorial {
            let mut quotient = *digit;
            while quotient != 0_u128 {
                sum_of_digits += (quotient % 10) as u16;
                quotient /= 10;
            }
        }
        if rep == 0 {
            println!(
                "Duration until sum of digits calculated: {:?}",
                start.elapsed()
            );
            println!("Sum of digits = {}", sum_of_digits);
        }
    }

    let duration = start.elapsed();
    println!("Total duration: {:?}", duration);
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn get_factorial(n: u8) -> Vec<u128> {
    let mut factorial: Vec<u128> = vec![1_u128];
    let primes = get_primes_up_to(n);

    for prime in primes {
        let mut count: u8 = 0;
        let mut quotient = n;
        while quotient > 0 {
            quotient /= prime;
            count += quotient;
        }
        let pow_of_prime_u128 = (prime as u128).pow(count as u32);
        let pow_of_prime = vec![pow_of_prime_u128 % BASE, pow_of_prime_u128 / BASE];
        factorial = multiply(&factorial, &pow_of_prime);
    }
    factorial
}

fn get_primes_up_to(n: u8) -> Vec<u8> {
    let mut is_prime = vec![true; (n + 1) as usize];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=(n as usize) {
        if !is_prime[i] {
            continue;
        }
        let mut composite = i * i;
        while composite <= n as usize {
            is_prime[composite] = false;
            composite += i;
        }
    }
    is_prime
        .into_iter()
        .enumerate()
        .filter(|(_, is_prm)| *is_prm)
        .map(|(index, _)| index as u8)
        .collect()
}

fn multiply(a: &Vec<u128>, b: &Vec<u128>) -> Vec<u128> {
    let min_size_of_product = (a.len() - 1) * (b.len() - 1) + 1;
    let mut product = vec![0; min_size_of_product];
    for i in 0..a.len() {
        for j in 0..b.len() {
            let mut adjustment = a[i] * b[j];
            let mut pos = i + j;
            while adjustment > 0 {
                if pos == product.len() {
                    product.push(0);
                }
                let new_digit = product[pos] + adjustment;
                product[pos] = new_digit % BASE;
                adjustment = new_digit / BASE;
                pos += 1;
            }
        }
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_overflow_of_square_of_max_digit_plus_max_digit() {
        let max_digit = BASE - 1;
        let _max_value = max_digit * max_digit + max_digit;
    }

    // Without allowing arithmetic_overflow, the compiler would detect the overflow and complain.
    #[test]
    #[should_panic]
    #[allow(arithmetic_overflow)]
    fn test_overflow_of_square_of_max_digit_in_ten_times_higher_base() {
        let max_digit = 10 * BASE - 1;
        let _max_value = max_digit * max_digit + max_digit;
    }

    #[test]
    fn test_get_primes_up_to_19() {
        assert_eq!(get_primes_up_to(19), vec![2, 3, 5, 7, 11, 13, 17, 19]);
    }
}
