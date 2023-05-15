use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let (denominator, (max_cycle_len, decimals)) = (2..1000)
            .map(|divisor| (divisor, get_recurring_cycle_len(divisor).unwrap()))
            .max_by_key(|&(_, (decimals, _))| decimals)
            .unwrap();
        if rep == 0 {
            println!("Denominator: {}", denominator);
            println!("Maximum cycle length: {}", max_cycle_len);
            println!("Decimals up to next cycle: {}", decimals);
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn get_recurring_cycle_len(denominator: usize) -> Result<(usize, String), String> {
    if denominator == 0 {
        return Err("Denominator cannot be zero".to_string());
    }
    let mut last_index_of_remainder = vec![0_usize; denominator];
    let mut remainder: usize = 1;
    let mut index: usize = 0;
    let mut representation: String = "0.".to_string();

    loop {
        index += 1;
        remainder *= 10;
        let quotient = remainder / denominator;
        representation.push((b'0' + quotient as u8) as char);
        remainder %= denominator;
        if remainder == 0 {
            return Ok((1, representation));
        }
        if quotient == 0 {
            continue;
        }
        if remainder == 1 {
            // This was the first remainder, so if it repeats, just return the index
            return Ok((index, representation));
        }
        let last_index = last_index_of_remainder[remainder];
        if last_index != 0 {
            return Ok((index - last_index, representation));
        }
        last_index_of_remainder[remainder] = index;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_len_of_1_over_zero() {
        assert!(get_recurring_cycle_len(0).is_err());
    }

    #[test]
    fn test_cyle_len_of_zero() {
        assert_eq!(get_recurring_cycle_len(2), Ok((1, "0.5".to_string())));
    }

    #[test]
    fn test_recurring_cycle_len_with_remainder_1() {
        assert_eq!(get_recurring_cycle_len(7), Ok((6, "0.142857".to_string())));
    }

    #[test]
    fn test_recurring_cycle_len_with_remainder_not_1() {
        assert_eq!(get_recurring_cycle_len(6), Ok((1, "0.16".to_string())));
    }
}
