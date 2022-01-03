use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        // Choose a base which is the largest power of 10 that fits in 64 bits
        let decimal_digits_in_base = ((1u128 << 64) as f64).log10().floor() as u32;
        let base = 10u128.pow(decimal_digits_in_base);
        if rep == 0 {
            println!("Base: {}", base);
            println!("Decimal digits per base digit: {}", decimal_digits_in_base);
        }

        // Store the "digits" (in this base) in a vector with the lowest digit first
        let mut representation: Vec<u128> = Vec::new();
        representation.push(1_u128 << 125); // 2 ^ 125
        carry_digit(&mut representation, 0, base);

        let representation = square(representation, base); // 2 ^ 250
        let representation = square(representation, base); // 2 ^ 500
        let representation = square(representation, base); // 2 ^ 1000

        let sum_of_digits = representation
            .iter()
            .map(|digit| {
                (*digit)
                    .to_string()
                    .chars()
                    .map(|ch| ch.to_digit(10).unwrap())
                    .sum::<u32>()
            })
            .sum::<u32>();

        if rep == 0 {
            println!("Sum of digits = {}", sum_of_digits);
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration / NUM_REPETITIONS);
}

pub fn carry_digit(digits: &mut Vec<u128>, index: usize, base: u128) {
    if index >= digits.len() {
        return;
    }
    let digit = digits[index];
    if digit >= base {
        digits[index] = digit % base;
        let carry = digit / base;
        if index + 1 == digits.len() {
            digits.push(carry);
        } else {
            digits[index + 1] += carry;
        }
        // recursively apply carries
        carry_digit(digits, index + 1, base);
    }
}

pub fn square(digits: Vec<u128>, base: u128) -> Vec<u128> {
    let num_digits = digits.len();
    let min_new_num_digits = (num_digits - 1) * (num_digits - 1);
    let mut result = vec![0u128; min_new_num_digits];
    for i in 0..num_digits {
        for j in i..num_digits {
            let prod = digits[i] * digits[j] * if i == j { 1 } else { 2 };
            let lower = prod % base;
            let upper = prod / base;
            result[i + j] += lower;
            carry_digit(&mut result, i + j, base);
            if upper > 0 {
                if i + j + 1 == result.len() {
                    result.push(upper)
                } else {
                    result[i + j + 1] += upper;
                    carry_digit(&mut result, i + j + 1, base);
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_no_overflow_of_twice_square_of_max_digit() {
        let decimal_digits_in_base = ((1u128 << 64) as f64).log10().floor() as u32;
        let base = 10u128.pow(decimal_digits_in_base);
        let _max_value = 2 * (base - 1) * (base - 1);
    }
}

/*
-----------------------------------------------------------------------------
Notes: code to calculate digits directly, is roughly twice as fast as this...

let dec_rep: String = representation
    .iter()
    .map(|digit| format!("{:0<1$}", *digit, decimal_digits_in_base as usize))
    .collect::<Vec<String>>()
    .concat();
println!("Decimal digits of 2^1000 = {}", dec_rep);

let sum_of_digits: u32 = dec_rep.chars().map(|ch| ch.to_digit(10).unwrap()).sum();
println!("Sum of digits = {}", sum_of_digits);
 */
