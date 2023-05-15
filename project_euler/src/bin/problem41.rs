use std::time::Instant;

const REPETITIONS: u32 = 100_000;

struct DescendingPandigitalIter {
    num_digits: usize,
    n: usize,
}

impl DescendingPandigitalIter {
    fn new(num_digits: usize) -> Self {
        DescendingPandigitalIter {
            num_digits,
            n: (2..=num_digits).product(),
        }
    }
}

impl Iterator for DescendingPandigitalIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n == 0 {
            None
        } else {
            self.n -= 1;
            let mut digit_used = vec![false; self.num_digits];
            let mut rem_digits_index = vec![0_u8; self.num_digits];
            let mut quotient = self.n;
            for i in 0..self.num_digits {
                rem_digits_index[self.num_digits - i - 1] = (quotient % (i + 1)) as u8;
                quotient /= i + 1;
            }
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
                result = result * 10 + digit_index + 1;
            }
            Some(result)
        }
    }
}

fn get_largest_pandigital_prime() -> usize {
    // The problem statement says that 2143 is a pandigital prime, so the answer has >= 4 digits
    for max_digit in (4..=9).rev() {
        let sum_of_digits: usize = (1..=max_digit).sum();
        if sum_of_digits % 3 == 0 {
            continue;
        }
        for next_perm in DescendingPandigitalIter::new(max_digit) {
            if is_multidigit_prime(next_perm) {
                return next_perm;
            }
        }
    }
    panic!("Unexpected error: no pandigital prime found, yet we know 2143 is a pandigital prime");
}

fn is_multidigit_prime(n: usize) -> bool {
    // if n is prime, it must be of the form 6k+1 or 6k+5...
    let mod_6 = n % 6;
    if mod_6 != 1 && mod_6 != 5 {
        return false;
    }

    // And its prime factors must also be of this form
    for multiple_of_6 in 1.. {
        let factor = 6 * multiple_of_6 - 1;
        if n % factor == 0 {
            return false;
        }

        let factor = 6 * multiple_of_6 + 1;
        if n % factor == 0 {
            return false;
        }

        if factor * factor >= n {
            return true;
        }
    }
    true
}

fn main() {
    let mut start_time = Instant::now();
    for rep in 0..=REPETITIONS {
        let largest_pandigital_prime = get_largest_pandigital_prime();
        if rep == 0 {
            println!("Largest pandigital prime: {largest_pandigital_prime}");
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

#[cfg(test)]
mod tests {
    use crate::DescendingPandigitalIter;

    #[test]
    fn test_descending_pandigital_iter() {
        let expected = [
            4321, 4312, 4231, 4213, 4132, 4123, 3421, 3412, 3241, 3214, 3142, 3124, 2431, 2413,
            2341, 2314, 2143, 2134, 1432, 1423, 1342, 1324, 1243, 1234,
        ];
        let actual: Vec<usize> = DescendingPandigitalIter::new(4).collect();
        assert_eq!(&actual, &expected);
    }
}
