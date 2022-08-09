use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;

fn main() {
    let start_time = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let mut total: u64 = 0;

        // a is the lowest (and highest) digit in the decimal palindromic representation.
        // Ignore numbers that end in 0 in binary (i.e. even numbers),
        // since the binary palindrome must not start with zero...
        for a in [1, 3, 5, 7, 9] {
            add_if_binary_palindrome(a, &mut total); // 1 digit palindrome - "a"
            add_if_binary_palindrome(11 * a, &mut total); // 2 digit palindrome - "aa"

            for b in 0..10 {
                add_if_binary_palindrome(101 * a + 10 * b, &mut total); // 3 digits - "aba"
                add_if_binary_palindrome(1001 * a + 110 * b, &mut total); // 4 digits - "abba"

                for c in 0..10 {
                    let abcba = 10001 * a + 1010 * b + 100 * c; // 5 digits
                    add_if_binary_palindrome(abcba, &mut total);

                    let abccba = 100001 * a + 10010 * b + 1100 * c; // 6 digits
                    add_if_binary_palindrome(abccba, &mut total);
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
        rem >>= 1;
    }
    if bin_rev == n {
        *total += n as u64;
    }
}
