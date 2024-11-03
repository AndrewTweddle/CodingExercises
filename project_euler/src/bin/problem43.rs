use std::time::Instant;

const REPETITIONS: u32 = 10;
const SMALLEST_TEN_DIGIT_PANDIGITAL_GENERATOR: usize = 9 * 8 * 7 * 6 * 5 * 4 * 3 * 2;
const LARGEST_TEN_DIGIT_PANDIGITAL_GENERATOR: usize =
    (((((((9 * 9 + 8) * 8 + 7) * 7 + 6) * 6 + 5) * 5 + 4) * 4 + 3) * 3 + 2) * 2 + 1;
const DIGIT_COUNT: usize = 10;

fn convert_to_ten_digit_pandigital(n: usize) -> Option<usize> {
    let gen_range =
        SMALLEST_TEN_DIGIT_PANDIGITAL_GENERATOR..=LARGEST_TEN_DIGIT_PANDIGITAL_GENERATOR;
    if gen_range.contains(&n) {
        // Store relative indices highest digit first, i.e. in bases 10, 9, 8, ..., 1
        let mut rem_digits_index = vec![0_u8; DIGIT_COUNT];
        let mut quotient = n;
        for base in 1..=DIGIT_COUNT {
            rem_digits_index[DIGIT_COUNT - base] = (quotient % base) as u8;
            quotient /= base;
        }

        let mut digit_used = [false; DIGIT_COUNT];
        let mut result: usize = 0;

        for rem_index in rem_digits_index {
            let digit_index = digit_used
                .iter()
                .enumerate()
                .filter(|(_, &used)| !used)
                .nth(rem_index as usize)
                .unwrap()
                .0;
            digit_used[digit_index] = true;
            result = result * 10 + digit_index;
        }

        Some(result)
    } else {
        None
    }
}

struct TenDigitPandigitalIter {
    n_fwd: usize,
    n_back: usize,
}

impl TenDigitPandigitalIter {
    fn new() -> Self {
        Self {
            n_fwd: SMALLEST_TEN_DIGIT_PANDIGITAL_GENERATOR - 1,
            n_back: LARGEST_TEN_DIGIT_PANDIGITAL_GENERATOR + 1,
        }
    }
}

impl Iterator for TenDigitPandigitalIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.n_fwd + 1 >= self.n_back {
            None
        } else {
            self.n_fwd += 1;
            convert_to_ten_digit_pandigital(self.n_fwd)
        }
    }
}

impl DoubleEndedIterator for TenDigitPandigitalIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.n_back - 1 <= self.n_fwd {
            None
        } else {
            self.n_back -= 1;
            convert_to_ten_digit_pandigital(self.n_back)
        }
    }
}

fn main() {
    let mut start_time = Instant::now();
    for rep in 0..=REPETITIONS {
        let answer: usize = TenDigitPandigitalIter::new()
            .filter(|&n| has_sub_string_divisibility_property(n))
            .sum();

        if rep == 0 {
            println!("Sum of pandigital numbers with property: {answer}");
            println!("Duration (incl I/O): {:?}", start_time.elapsed());

            // Restart timer to exclude I/O
            start_time = Instant::now();
        }
    }
    let duration = start_time.elapsed();
    println!(
        "Average duration over {REPETITIONS} iterations (excl I/O): {:?}",
        duration / REPETITIONS
    );
}

const SEVEN_REVERSE_PRIMES: [usize; 7] = [17, 13, 11, 7, 5, 3, 2];

fn has_sub_string_divisibility_property(mut n: usize) -> bool {
    for prime in SEVEN_REVERSE_PRIMES {
        let sub_number = n % 1000;
        if sub_number % prime != 0 {
            return false;
        }
        n /= 10;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::TenDigitPandigitalIter;
    use crate::has_sub_string_divisibility_property;

    #[test]
    fn test_iter_over_smallest() {
        let expected = [
            1023456789, 1023456798, 1023456879, 1023456897, 1023456978, 1023456987,
        ];
        let actual: Vec<usize> = TenDigitPandigitalIter::new().take(6).collect();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn test_iter_over_largest() {
        let expected = [
            9876543210, 9876543201, 9876543120, 9876543102, 9876543021, 9876543012,
        ];
        let actual: Vec<usize> = TenDigitPandigitalIter::new().rev().take(6).collect();
        assert_eq!(&actual, &expected);
    }

    #[test]
    fn test_has_divisibility_property() {
        let n: usize = 1406357289;
        assert_eq!(has_sub_string_divisibility_property(n), true);
    }

    #[test]
    #[ignore = "Runs too slowly in debug mode (but passes in release mode)"]
    // To test in release mode:
    // $ cargo test --release --bin problem43 tests::test_double_ended_iterator -- --ignored
    fn test_double_ended_iterator() {
        let mut pdi = TenDigitPandigitalIter::new();

        // Iterate backwards
        let last = pdi.next_back();
        let penultimate = pdi.next_back();

        // Iterate forwards
        let first = pdi.next();
        let third_last = (&mut pdi).last(); // using a reference, since last consumes the iterator

        // Check forwards and backwards directions are both exhausted
        let back_after_done = pdi.next_back();
        let fwd_after_done = pdi.next();

        assert_eq!(first, Some(1023456789));
        assert_eq!(last, Some(9876543210));
        assert_eq!(penultimate, Some(9876543201));
        assert_eq!(third_last, Some(9876543120));
        assert_eq!(back_after_done, None);
        assert_eq!(fwd_after_done, None);
    }
}
