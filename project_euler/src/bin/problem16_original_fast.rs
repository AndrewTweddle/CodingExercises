use std::time::Instant;

/* Choose a base B which is the largest power of 10 such that 2(B-1)^2 + (B-1) < 2^128
 * A number in this base will have its digits stored in a Vec<u128>, lowest digit first.
 * The base is chosen to allow the squaring a number to occur safely without overflowing
 * the u128 during the standard operation of multiplying each pair of digits,
 * doubling the result (when digits in different positions are multiplied),
 * adding to an existing digit and carrying the excess (if the value exceeds B-1).
 */
const BASE: u128 = 10_000_000_000_000_000_000;

fn main() {
    let start = Instant::now();

    let num_repetitions = 500;
    for rep in 0..num_repetitions {
        let seed = 1_u128 << 125;
        let representation: Vec<u128> = vec![seed % BASE, seed / BASE];     // 2 ^ 125
        let representation = square(representation);                        // 2 ^ 250
        let representation = square(representation);                        // 2 ^ 500
        let representation = square(representation);                        // 2 ^ 1000

        let mut sum_of_digits: u16 = 0;
        for mut quotient in representation {
            while quotient != 0 {
                sum_of_digits += (quotient % 10) as u16;
                quotient /= 10;
            }
        }
        if rep == 0 {
            println!("Sum of digits = {}", sum_of_digits);
        }
    }

    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / num_repetitions);
}

pub fn square(digits: Vec<u128>) -> Vec<u128> {
    let num_digits = digits.len();
    let min_new_num_digits = (num_digits - 1) * (num_digits - 1) + 1;
    let mut result = vec![0u128; min_new_num_digits];
    for i in 0..num_digits {
        for j in i..num_digits {
            let mut index = i + j;
            let mut adjustment = if i == j {
                digits[i] * digits[i]
            } else {
                2 * digits[i] * digits[j]
            };
            while adjustment > 0 {
                if index == result.len() {
                    result.push(adjustment % BASE);
                } else {
                    adjustment += result[index];
                    result[index] = adjustment % BASE;
                }
                adjustment /= BASE;  // carry
                index += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::BASE;

    #[test]
    fn test_no_overflow_of_twice_square_of_max_digit_plus_max_digit() {
        let max_digit = BASE - 1;
        let _max_value = 2 * max_digit * max_digit + max_digit;
    }

    #[test]
    #[should_panic]
    fn test_overflow_of_twice_square_of_max_digit_in_ten_times_higher_base() {
        let max_digit = 10 * BASE - 1;
        let _max_value = 2 * max_digit * max_digit + max_digit;
    }

    #[test]
    fn test_2_to_the_125_overflows_once_only() {
        let max_digit = BASE - 1;
        assert!(1_u128 << 125 <= max_digit * BASE + max_digit)
    }

    #[test]
    fn test_u16_contains_sum_of_digits() {
        let decimal_digits_in_2_pow_1000 = 1000_u32 * (2.0_f64.log10().ceil() as u32);
        let max_sum_of_digits = decimal_digits_in_2_pow_1000 * 9;
        assert!(max_sum_of_digits < (2 << 16))
    }
}
