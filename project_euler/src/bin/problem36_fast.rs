use std::time::Instant;

const NUM_REPETITIONS: u32 = 10;

fn main() {
    let start_time = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let mut total: u64 = 0;
        let mut digits: Vec<u8> = Vec::with_capacity(6);
        let mut bin_rev: usize;
        for n in 1..1_000_000_usize {
            // Skip numbers ending in zero, as the reverse can't start with zero
            if n % 2 == 0 || n % 10 == 0 {
                continue;
            }

            // Check that the binary representations are palindromic
            let mut rem = n;
            bin_rev = 0;
            while rem != 0 {
                bin_rev <<= 1;
                if rem % 2 != 0 {
                    bin_rev |= 1;
                }
                rem /= 2;
            }
            if bin_rev != n {
                continue;
            }

            // Check that the decimal representations are palindromic
            digits.clear();
            rem = n;
            while rem != 0 {
                let digit = (rem % 10) as u8;
                digits.push(digit);
                rem /= 10;
            }
            let num_digits = digits.len();
            let mut is_palindrome = true;
            for i in 0..(num_digits / 2) {
                if digits[i] != digits[num_digits - i - 1] {
                    is_palindrome = false;
                    break;
                }
            }

            if is_palindrome {
                total += n as u64;
            }
        }
        if rep == 0 {
            println!("Sum of double-base palindromes under 1 million = {}", total);
        }
    }

    let duration = start_time.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}
