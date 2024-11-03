use std::ops::Index;
use std::time::Instant;

const DIGITS: [u8; 4] = [1, 3, 7, 9];

fn main() {
    let start_time = Instant::now();

    // Get lookup of primes
    let primes = get_primes_below(1_000_000);

    // The single digit primes are automatically circular
    let mut circular_primes: Vec<usize> = vec![2, 3, 5, 7];

    // All other circular primes must only have digits from {1, 3, 7, 9}
    // - otherwise the number is even or divisible by 5, and hence not prime,
    //  when it has been cycled to have a digit not from that set as its unit digit.

    for digit_count in 2..=6 {
        // There are 4 digits available for each of the positions
        let candidate_count = 4_usize.pow(digit_count as u32);
        let mut candidate_excluded = BitArray::with_capacity(candidate_count, false);
        let mut permutations: Vec<usize> = Vec::with_capacity(digit_count);
        for encoded_candidate in 0..candidate_count {
            // encoded_candidate is a base 4 number with the 4 DIGITS
            if candidate_excluded[encoded_candidate] {
                continue;
            }
            candidate_excluded.set(encoded_candidate, true);

            let digits = get_digits(encoded_candidate, digit_count);
            let candidate = digits_to_number(&digits);
            permutations.clear();
            permutations.push(candidate);
            for rotation in 1..digit_count {
                let permutation = digits_to_permuted_number(&digits, rotation);
                if !permutations.contains(&permutation) {
                    permutations.push(permutation);
                    let encoded_permutation =
                        permute_encoded_number(encoded_candidate, rotation, digit_count);
                    candidate_excluded.set(encoded_permutation, true);
                }
            }
            if permutations
                .iter()
                .all(|perm| primes.binary_search(perm).is_ok())
            {
                circular_primes.extend(permutations.iter());
            }
        }
    }

    let end_time = start_time.elapsed();
    println!("Duration: {:?}", end_time);

    println!("# of primes below 1 million = {}", primes.len());
    println!("# of circular primes below 1 million: {}", circular_primes.len());
    for p in circular_primes {
        println!("    {}", p);
    }
}

fn digits_to_number(digits: &[u8]) -> usize {
    digits_to_permuted_number(digits, 0)
}

fn digits_to_permuted_number(digits: &[u8], rotation: usize) -> usize {
    digits_to_permuted_number_in_base(digits, rotation, 10)
}

fn get_digits(mut candidate: usize, digit_count: usize) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::with_capacity(digit_count);
    for _ in 0..digit_count {
        let digit_index = candidate % 4;
        digits.push(DIGITS[digit_index]);
        candidate /= 4;
    }
    digits
}

fn permute_encoded_number(mut encoded_number: usize, rotation: usize, digit_count: usize) -> usize {
    let mut digits: Vec<u8> = Vec::with_capacity(digit_count);
    for _ in 0..digit_count {
        let digit_index = encoded_number % 4;
        digits.push(digit_index as u8);
        encoded_number /= 4;
    }
    digits_to_permuted_number_in_base(&digits, rotation, 4)
}

fn digits_to_permuted_number_in_base(digits: &[u8], rotation: usize, base: usize) -> usize {
    let digit_count = digits.len();
    let mut result: usize = 0;
    let mut power: usize = 1;
    for i in 0..digit_count {
        let digit = digits[(i + rotation) % digit_count];
        result += power * (digit as usize);
        power *= base;
    }
    result
}

fn get_primes_below(n: usize) -> Vec<usize> {
    let mut primes = Vec::<usize>::with_capacity(n);
    let mut is_prime = BitArray::with_capacity(n, true);
    is_prime.set(0, false);
    is_prime.set(1, false);
    for i in 2..n {
        if is_prime[i] {
            primes.push(i);
            for j in ((i * i)..n).step_by(i) {
                is_prime.set(j, false);
            }
        }
    }
    primes
}

struct BitArray {
    bytes: Vec<u64>,
    default_value: bool,
}

impl BitArray {
    fn with_capacity(n: usize, default_value: bool) -> Self {
        let byte_count = (n + 63) / 64;
        BitArray {
            bytes: Vec::<u64>::with_capacity(byte_count),
            default_value,
        }
    }

    fn set(&mut self, index: usize, value: bool) {
        let byte_index = index / 64;
        if byte_index >= self.bytes.len() {
            if value == self.default_value {
                return;
            }
            self.bytes.resize(byte_index + 1, 0);
        }
        let byte = &mut self.bytes[byte_index];
        let bit_index = index % 64;
        let bit_mask = 1 << bit_index;
        if value == self.default_value {
            *byte &= u64::MAX ^ bit_mask;
        } else {
            *byte |= bit_mask;
        }
    }
}

impl Index<usize> for BitArray {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        let byte_index = index / 64;
        let is_bit_set = if byte_index >= self.bytes.len() {
            false
        } else {
            let bit_index = index % 64;
            let bit_map = 1 << bit_index;
            self.bytes[byte_index] & bit_map != 0
        };

        // A bit being set means return the opposite of the default value...
        if is_bit_set == self.default_value {
            &false
        } else {
            &true
        }
    }
}
