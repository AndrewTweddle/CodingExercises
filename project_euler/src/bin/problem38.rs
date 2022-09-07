use std::time::Instant;

// Create an iterator over the digits of a number (from smallest to largest)
struct DigitIter(u32);

impl Iterator for DigitIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            None
        } else {
            let digit = (self.0 % 10) as u8;
            self.0 /= 10;
            Some(digit)
        }
    }
}

const NUM_REPETITIONS: u32 = 1000;

fn main() {
    let mut start_time = Instant::now();
    for rep in 0..=NUM_REPETITIONS {
        let mut pandigital_count: usize = 0;
        let mut max_pandigital: u32 = 0;

        // Pandigitals have exactly 9 unique, non-zero digits.
        // There must be at least 2 integers in the consecutive sequence.
        // So the number can't have 5 or more digits.
        for num in 1..=9999 {
            let mut pandigital: u32 = 0;

            // Check that only unique digits are seen
            let mut seen_digit = [false; 10];

            // 0 is not allowed in a pandigital, so mark it as already seen
            seen_digit[0] = true;

            // Track how many digits have been seen, so we can stop when we've seen them all
            let mut combined_digit_count: u32 = 0;

            'seq: for n in 1..10 {
                let i = n * num;
                let mut digit_count: u32 = 0;
                for digit in DigitIter(i) {
                    if seen_digit[digit as usize] {
                        // duplicate digit seen, so not a pandigital
                        break 'seq;
                    }
                    seen_digit[digit as usize] = true;
                    digit_count += 1;
                }

                pandigital *= 10_u32.pow(digit_count);
                pandigital += i;

                combined_digit_count += digit_count;
                if combined_digit_count == 9 {
                    // All digits have been found, so it's a valid pandigital number...
                    pandigital_count += 1;
                    if pandigital > max_pandigital {
                        max_pandigital = pandigital;
                    }
                    if rep == 0 {
                        println!(
                            "{}: {} * (1,..,{}) = {}",
                            pandigital_count, num, n, pandigital
                        );
                    }
                    break;
                }
            }
        }

        if rep == 0 {
            println!("Max pandigital: {}", max_pandigital);
            println!(
                "Duration (including println statements): {:?}",
                start_time.elapsed()
            );

            // Restart the timer, so that the average duration excludes the println calls...
            start_time = Instant::now();
        }
    }

    if NUM_REPETITIONS > 0 {
        let duration = start_time.elapsed();
        println!(
            "Avg duration of {} further repetitions: {:?}",
            NUM_REPETITIONS,
            duration / NUM_REPETITIONS
        );
    }
}
