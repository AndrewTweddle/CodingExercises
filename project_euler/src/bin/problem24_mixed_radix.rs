use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        // Convert the number to a mixed radix representation, or more correctly,
        // the factorial number system - see https://en.wikipedia.org/wiki/Factorial_number_system.
        // This has radix 10 in the most significant position, then 9, 8, etc
        // down to radix 1 in the least significant position
        let mut digits: [usize; 10] = [0_usize; 10];
        let mut n = 1_000_000_usize - 1;

        // The final digit is always zero, so start with the 9th digit and work backwards
        for digits_in_pos in 2..=10 {
            let pos = 10 - digits_in_pos;
            digits[pos] = n % digits_in_pos;
            n /= digits_in_pos;
        }

        // Convert this mixed radix representation to a permutation of the 10 decimal digits...
        let mut rem_digits = String::from("0123456789");
        let mut lex_digits = String::from("");
        for digit_index in digits {
            let next_digit = rem_digits.remove(digit_index);
            lex_digits.push(next_digit);
        }
        if rep == 0 {
            println!("{}", lex_digits);
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS)
}
