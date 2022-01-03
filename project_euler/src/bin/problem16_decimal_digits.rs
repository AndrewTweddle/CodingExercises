use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let num_digits = (1000_f32 * 2_f32.log10()).ceil() as usize;
        let mut digits = vec![0_u8; num_digits];
        digits[0] = 1;
        for _ in 0..1000 {
            let mut carry = 0;
            for i in 0..num_digits {
                let new_digit = 2 * digits[i] + carry;
                if new_digit >= 10 {
                    digits[i] = new_digit % 10;
                    carry = new_digit / 10;
                } else {
                    digits[i] = new_digit;
                    carry = 0;
                }
            }
        }
        let sum_of_digits: u32 = digits.iter().map(|digit| *digit as u32).sum();
        if rep == 0 {
            println!("Sum of digits of 2^1000 = {}", sum_of_digits);
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration / NUM_REPETITIONS);
}
