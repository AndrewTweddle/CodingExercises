use std::collections::HashSet;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

// Represent a^b in a base such that each digit satisfies: BASE * 100 + CARRY < 2 ^ 128
// where CARRY is the digit carried over from the previous position.
// Choose BASE = 2^120, since 100 < 2^7.
const BASE: u128 = 2 << 120;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let mut unique_powers = HashSet::<Vec<u128>>::new();
        for a in 2..=100 {
            let mut pow = vec![a as u128];
            for _b in 2..=100 {
                // Multiply pow by a
                let mut carry: u128 = 0;
                for i in 0.. {
                    if i == pow.len() {
                        if carry != 0 {
                            pow.push(carry);
                        }
                        break;
                    }
                    let digit = pow[i] * a as u128 + carry;
                    pow[i] = digit % BASE;
                    carry = digit / BASE;
                }
                unique_powers.insert(pow.clone());
            }
        }

        if rep == 0 {
            println!("Count of unique powers: {}", unique_powers.len());
        }
    }

    let duration = start.elapsed() / NUM_REPETITIONS;
    println!("Avg duration of {} runs: {:?}", NUM_REPETITIONS, duration);
}

#[cfg(test)]
mod tests {
    use super::BASE;

    #[test]
    fn test_no_overflow_of_100_times_max_digit_plus_99() {
        let max_digit = BASE - 1;
        let _max_value = 100 * max_digit + 99;
    }

    #[test]
    #[allow(arithmetic_overflow)]
    #[should_panic]
    fn test_overflow_of_100_times_max_digit_plus_99_in_twice_the_base() {
        let max_digit = 2 * BASE - 1;
        let _max_value = 100 * max_digit + 99;
    }
}
