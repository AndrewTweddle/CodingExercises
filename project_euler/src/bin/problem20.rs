use std::time::Instant;

const BASE_EXPONENT: u32 = 36;
const BASE: u128 = 10_u128.pow(BASE_EXPONENT);
    // 1_000_000_000_000_000_000_000_000_000_000_000_000_u128;

fn main() {
    // 2^7 > 100, which is the largest factor we will multiply by
    let decimal_digits_in_base = ((1u128 << 120) as f64).log10().floor() as u32;
    println!("Estimated decimal digits needed in base: {}", decimal_digits_in_base);

    let estimated_digits_in_answer: usize = (2..=100)
        .into_iter()
        .map(|factor| (factor as f64).log10().ceil() / (decimal_digits_in_base as f64))
        .sum::<f64>()
        .ceil() as usize;
    println!("Estimated digits in answer: {}", estimated_digits_in_answer);

    let start = Instant::now();
    let num_repetitions = 1000;
    for rep in 0..num_repetitions {
        let mut factorial = Vec::with_capacity(estimated_digits_in_answer);
        factorial.push(1);
        for factor in 2..=100 {
            // We could reduce multiples of 10 to their tens digit only,
            // because the extra zero doesn't change the sum of digits.
            // In practice this made the code slower...
            // if factor % 10 == 0 { factor /= 10; }
            let mut carry = 0_u128;
            for pos in 0..factorial.len() {
                let new_digit = factorial[pos] * factor + carry;
                carry = new_digit / BASE;
                factorial[pos] = new_digit % BASE;
            }
            if carry > 0 {
                factorial.push(carry);
            }
        }

        if rep == 0 {
            // Print out the number...
            print!("100! =  {}", factorial.last().unwrap());
            factorial
                .iter()
                .rev()
                .skip(1)
                .for_each(|digit| print!("{:0width$}", *digit, width = BASE_EXPONENT as usize));
            println!("");
        }

        let mut sum_of_digits: u16 = 0;
        for mut quotient in factorial {
            while quotient != 0 {
                sum_of_digits += (quotient % 10) as u16;
                quotient /= 10;
            }
        }

        if rep == 0 {
            println!("Sum of digits of 100! = {}", sum_of_digits);
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
    println!("Avg Duration: {:?}", duration / num_repetitions);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base_x_100_plus_carry_will_not_overflow() {
        let max_digit = BASE - 1;
        let max_digit_x_100 = 100 * max_digit;
        let carry = max_digit_x_100 / BASE;
        let _max_value_in_u128 = max_digit_x_100 + carry;
    }

    #[test]
    #[should_panic]
    #[allow(arithmetic_overflow)]
    fn test_ten_times_higher_base_x_100_plus_carry_will_overflow() {
        let bigger_base = 10 * BASE;
        let max_digit = bigger_base - 1;
        let max_digit_x_100 = 100 * max_digit;
        let carry = max_digit_x_100 / bigger_base;
        let _max_value_in_128 = dbg!(max_digit_x_100 + carry);
    }
}