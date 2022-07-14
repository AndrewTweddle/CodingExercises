use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;

fn main() {
    let start_time = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let mut total: u64 = 0;

        // digit0 is the lowest (and highest) digit in the palindromic representation
        // ignore numbers that end in 0 in decimal or binary...
        for digit0 in [1, 3, 5, 7, 9] {
            add_if_binary_palindrome(digit0, &mut total); // 1 digit palindrome
            add_if_binary_palindrome(11 * digit0, &mut total); // 2 digit palindrome

            for digit1 in 0..10 {
                add_if_binary_palindrome(101 * digit0 + 10 * digit1, &mut total); // 3 digits
                add_if_binary_palindrome(1001 * digit0 + 110 * digit1, &mut total); // 4 digits

                for digit2 in 0..10 {
                    let five_digit_palindrome = 10001 * digit0 + 1010 * digit1 + 100 * digit2;
                    add_if_binary_palindrome(five_digit_palindrome, &mut total);

                    let six_digit_palindrome = 100001 * digit0 + 10010 * digit1 + 1100 * digit2;
                    add_if_binary_palindrome(six_digit_palindrome, &mut total);
                }
            }
        }

        if rep == 0 {
            println!("Sum of double-base palindromes under 1 million = {}", total);
        }
    }

    let duration = start_time.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn add_if_binary_palindrome(n: u32, total: &mut u64) {
    // Check whether the binary representation is palindromic
    let mut rem = n;
    let mut bin_rev: u32 = 0;
    while rem != 0 {
        bin_rev <<= 1;
        if rem % 2 != 0 {
            bin_rev |= 1;
        }
        rem /= 2;
    }
    if bin_rev == n {
        *total += n as u64;
    }
}
