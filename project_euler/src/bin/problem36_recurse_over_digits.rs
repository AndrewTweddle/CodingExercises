use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;
const MAX_PALINDROME: u64 = 999_999;

fn main() {
    let mut start_time: Instant = Instant::now();

    for rep in 0..=NUM_REPETITIONS {
        let total = get_sum_of_double_palindromes_up_to(MAX_PALINDROME);
        if rep == 0 {
            println!(
                "Sum of double-base palindromes under {} = {}",
                MAX_PALINDROME + 1,
                total
            );
            println!(
                "Duration of initial repetition (incl println): {:?}",
                start_time.elapsed()
            );

            // Re-start timer after repetition zero, to exclude the println statement from timings
            start_time = Instant::now();
        }
    }

    let duration = start_time.elapsed();
    println!(
        "Avg duration of {} further repetitions: {:?}",
        NUM_REPETITIONS,
        duration / NUM_REPETITIONS
    );
}

fn get_sum_of_double_palindromes_up_to(n: u64) -> u64 {
    let mut total: u64 = 0;
    generate_first_digit_and_add_palindromes(n, &mut total);
    total
}

fn generate_first_digit_and_add_palindromes(n: u64, total: &mut u64) {
    // Binary palindromes can't end in zero, since the first bit must be 1.
    // So the decimal palindromes must also be an odd number.
    for digit in [1, 3, 5, 7, 9] {
        if digit <= n {
            // All single digit numbers are decimal palindromes
            add_if_binary_palindrome(digit, total);
        }
        let palindrome = 11 * digit;
        if palindrome <= n {
            add_if_binary_palindrome(palindrome, total);
            generate_next_digit_and_add_palindromes(10, 10 * digit, digit, n, total);
        }
    }
}

fn generate_next_digit_and_add_palindromes(
    pow_of_10: u64, // The power of 10 to multiply the next middle digit by
    hi_val: u64,    // The value of the high digits at the previous level (first half of the digits)
    lo_val: u64,    // The value of the low digits at the previous level (last half of the digits)
    n: u64,
    total: &mut u64,
) {
    // Generate palindrome with odd number of digits
    for digit in 0..10 {
        let mid_val = digit * pow_of_10;
        let odd_palindrome = 10 * hi_val + mid_val + lo_val;
        if odd_palindrome > n {
            return;
        }
        add_if_binary_palindrome(odd_palindrome, total);

        let new_hi_val = 100 * hi_val + 10 * mid_val;
        let new_lo_val = mid_val + lo_val;
        let even_palindrome = new_hi_val + new_lo_val;
        if even_palindrome <= n {
            add_if_binary_palindrome(even_palindrome, total);

            // Recursively generate palindromes with more digits
            generate_next_digit_and_add_palindromes(
                10 * pow_of_10,
                new_hi_val,
                new_lo_val,
                n,
                total,
            );
        }
    }
}

fn add_if_binary_palindrome(n: u64, total: &mut u64) {
    // Check whether the binary representation is palindromic
    let mut rem = n;
    let mut bin_rev: u64 = 0;
    while rem != 0 {
        bin_rev <<= 1;
        if rem % 2 != 0 {
            bin_rev |= 1;
        }
        rem >>= 1;
    }
    if bin_rev == n {
        *total += n as u64;
    }
}
