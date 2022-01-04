use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;
const ZERO_BASED_INDEX: usize = 1_000_000_usize - 1;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let permutation = get_lex_digits_at_target(ZERO_BASED_INDEX, String::from("0123456789"));
        if rep == 0 {
            println!("{}", permutation);
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS)
}

fn get_lex_digits_at_target(mut target_index: usize, mut rem_digits: String) -> String {
    let mut lex_digits: String = String::from("");
    while rem_digits.len() > 0 {
        let (digit_index, next_target_index) =
            get_next_digit_index_and_remainder(rem_digits.len(), target_index);
        target_index = next_target_index;
        let next_digit = rem_digits.remove(digit_index);
        lex_digits.push(next_digit);
    }
    lex_digits
}

fn get_next_digit_index_and_remainder(digits_left: usize, target_index: usize) -> (usize, usize) {
    let perms_per_digit = count_perms(digits_left - 1);
    let next_digit_index = target_index / perms_per_digit;
    let remainder = target_index % perms_per_digit;
    (next_digit_index, remainder)
}

fn count_perms(digits_left: usize) -> usize {
    // Just a factorial
    if digits_left <= 1 {
        1
    } else {
        (2..=digits_left).into_iter().product()
    }
}

#[cfg(test)]
mod tests {
    use super::{count_perms, get_lex_digits_at_target, get_next_digit_index_and_remainder};

    #[test]
    fn test_count_perms_9_factorial() {
        assert_eq!(count_perms(9), 362880);
    }

    #[test]
    fn test_next_index_and_remainder_3digits_index0() {
        assert_eq!(get_next_digit_index_and_remainder(3, 0), (0, 0));
    }

    #[test]
    fn test_next_index_and_remainder_3digits_index5() {
        assert_eq!(get_next_digit_index_and_remainder(3, 5), (2, 1));
    }

    #[test]
    fn test_get_lex_digits_at_target_abc() {
        assert_eq!(get_lex_digits_at_target(0, String::from("abc")), "abc");
        assert_eq!(get_lex_digits_at_target(1, String::from("abc")), "acb");
        assert_eq!(get_lex_digits_at_target(2, String::from("abc")), "bac");
        assert_eq!(get_lex_digits_at_target(3, String::from("abc")), "bca");
        assert_eq!(get_lex_digits_at_target(4, String::from("abc")), "cab");
        assert_eq!(get_lex_digits_at_target(5, String::from("abc")), "cba");
    }
}
