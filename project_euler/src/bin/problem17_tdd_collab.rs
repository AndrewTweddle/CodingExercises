/*
If the numbers 1 to 5 are written out in words:
one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters
and 115 (one hundred and fifteen) contains 20 letters.

The use of "and" when writing out numbers is in compliance with British usage.
 */
#![allow(clippy::all)]

use std::time::Instant;

fn main() {
    let start = Instant::now();

    let num_repetitions = 1000;
    for _ in 0..num_repetitions {
        let total_letters: usize = (1..=1000)
            .into_iter()
            .map(|i| get_pascal_case_string(i)
                .expect("You overflowed our function.")
                .len())
            .sum();
        println!("Total letters: {}", total_letters);
    }
    let duration = start.elapsed();
    println!("Took {:?}", duration);
    println!("Average duration per repetition: {:?}", duration / num_repetitions);
}

const SMALL_NUMBER_NAMES: [&str; 20] = ["", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
    "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"];

const TENS_NAMES: [&str; 10] = ["", "Ten", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"];

pub fn get_pascal_case_string(n: u16) -> Result<String, &'static str> {
    if n > 1000 {
        return Err("Exceeded maximum of 1000");
    }

    if n == 1000 {
        return Ok(String::from("OneThousand"));
    }

    if n > 99 {
        let hundreds = n / 100;
        let hundreds_str = get_pascal_case_string(hundreds)?;
        let under_hundred = n % 100;

        if under_hundred == 0 {
            Ok(hundreds_str + "Hundred")
        } else {
            let under_hundred_str = get_pascal_case_string(under_hundred);
            Ok(hundreds_str + "HundredAnd" + under_hundred_str?.as_str())
        }
    } else if n > 19 {
        let tens = n / 10;
        let units = n % 10;
        let tens_str = TENS_NAMES[tens as usize];
        let units_str = SMALL_NUMBER_NAMES[units as usize];
        Ok(String::from(tens_str) + units_str)
    } else {
        Ok(String::from(SMALL_NUMBER_NAMES[n as usize]))
    }
}

pub fn get_number_for_pascal_case_string(pc_string: &str) -> Result<u16, &'static str> {
    if pc_string.is_empty() {
        return Err("Unrecognized String");
    }

    if pc_string == "OneThousand" {
        return Ok(1000);
    }

    let hundred_small_pair  = pc_string.split_once("HundredAnd");
    if let Some((hundreds_str, sub_hundred_str)) = hundred_small_pair {
        let hundreds_pair = SMALL_NUMBER_NAMES[0..10]
            .iter()
            .enumerate()
            .skip(1)
            .find(|(_, &small_str)| hundreds_str == small_str);
        let found_hundreds = hundreds_pair.map(|(index, _)| index as u16);
        return match found_hundreds {
            Some(hundreds) => {
                let maybe_sub_hundred = get_number_for_pascal_case_string(sub_hundred_str);
                match maybe_sub_hundred {
                    Ok(sub_hundred) => Ok(100 * hundreds + sub_hundred),
                    Err(_) => Err("Sub-hundreds part of string unrecognized"),
                }
            }
            None => Err("Invalid hundreds part of input string")
        }
    }

    let exact_hundreds_str = pc_string.strip_suffix("Hundred");
    if let Some(found_exact_hundreds) = exact_hundreds_str {
        let hundreds_index = SMALL_NUMBER_NAMES[0..9]
            .iter()
            .position(|&x| x == found_exact_hundreds);
        return match hundreds_index {
            Some(0) => Err("Hundred without a prefix is illegal"),
            Some(exact_hundreds) => Ok(100 * (exact_hundreds) as u16),
            None => Err("Asked for an illegal number of hundreds"),
        };
    }

    let tens_pair = TENS_NAMES
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, &tens_str)| pc_string.starts_with(tens_str));

    let found_small_number = SMALL_NUMBER_NAMES
        .iter()
        .enumerate()
        .skip(1)
        .find(|(_, &small_str)| pc_string == if tens_pair.is_some() {
            tens_pair.unwrap().1.to_string() + small_str
        } else {
            small_str.to_string()
        })
        .map(|(index, _)| index as u16);

    let found_tens = tens_pair.map(|(index, _)| index as u16);

    let small_num = found_small_number.unwrap_or_default();
    let tens_digit = found_tens.unwrap_or_default();

    if tens_digit > 0 && small_num > 9 {
        return Err("The units are more than nine")
    }

    if small_num == 0 && pc_string != TENS_NAMES[tens_digit as usize] {
        return Err("The tens digit is followed by garbage");
    }

    let total = small_num + 10 * tens_digit;
    if total == 0 {
        Err("The string was invalid")
    } else {
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod num_to_string_tests {
        use super::get_pascal_case_string;

        #[test]
        fn test_one() {
            assert_eq!(get_pascal_case_string(1).unwrap(), "One");
        }

        #[test]
        fn test_two() {
            assert_eq!(get_pascal_case_string(2).unwrap(), "Two");
        }

        #[test]
        fn test_nineteen() {
            assert_eq!(get_pascal_case_string(19).unwrap(), "Nineteen");
        }

        #[test]
        fn test_twenty() {
            assert_eq!(get_pascal_case_string(20).unwrap(), "Twenty");
        }

        #[test]
        fn test_thirty() {
            assert_eq!(get_pascal_case_string(30).unwrap(), "Thirty");
        }

        #[test]
        fn test_forty_two() {
            assert_eq!(get_pascal_case_string(42).unwrap(), "FortyTwo");
        }

        #[test]
        fn test_one_hundred() {
            assert_eq!(get_pascal_case_string(100).unwrap(), "OneHundred");
        }

        #[test]
        fn test_one_hundred_and_one() {
            assert_eq!(get_pascal_case_string(101).unwrap(), "OneHundredAndOne");
        }

        #[test]
        fn test_one_thousand() {
            assert_eq!(get_pascal_case_string(1000).unwrap(), "OneThousand");
        }

        #[test]
        #[should_panic]
        fn test_greater_than_a_thousand() {
            let _ = get_pascal_case_string(1001).unwrap();
        }
    }

    mod string_to_num_tests {
        use super::get_number_for_pascal_case_string;

        #[test]
        fn test_1() { assert_eq!(get_number_for_pascal_case_string("One"), Ok(1)); }

        #[test]
        fn test_empty_string() { assert!(get_number_for_pascal_case_string("").is_err())}

        #[test]
        fn test_2() { assert_eq!(get_number_for_pascal_case_string("Two"), Ok(2)); }

        #[test]
        fn test_30() { assert_eq!(get_number_for_pascal_case_string("Thirty"), Ok(30)); }

        #[test]
        fn test_42() { assert_eq!(get_number_for_pascal_case_string("FortyTwo"), Ok(42)); }

        #[test]
        fn test_100() { assert_eq!(get_number_for_pascal_case_string("OneHundred"), Ok(100)); }

        #[test]
        fn test_101() { assert_eq!(get_number_for_pascal_case_string("OneHundredAndOne"), Ok(101)); }

        #[test]
        fn test_just_the_word_and() {
            assert!(get_number_for_pascal_case_string("And").is_err());
        }

        #[test]
        fn test_one_hundred_and_twenty_ten() {
            assert!(get_number_for_pascal_case_string("OneHundredAndTwentyTen").is_err());
        }

        #[test]
        fn test_ten_hundred() {
            assert!(dbg!(get_number_for_pascal_case_string("TenHundred")).is_err());
        }

        #[test]
        fn test_twenty_twenty() {
            assert!(dbg!(get_number_for_pascal_case_string("TwentyTwenty")).is_err());
        }

        #[test]
        fn test_hundred() {
            assert!(dbg!(get_number_for_pascal_case_string("Hundred")).is_err());
        }
    }

    mod property_based_tests {
        use super::*;
        use proptest::prelude::*;

        fn get_alternatives_regex(alts: &[&str]) -> String { format!("({})", alts.join("|")) }

        fn get_zero_small_nums_and_tens_regex() -> String {
            let alternatives: Vec<&str> = vec!["Zero"]
                .iter()
                .chain(SMALL_NUMBER_NAMES[1..].iter())
                .chain(TENS_NAMES[2..].iter())
                .map(|&s| s)
                .collect();
            get_alternatives_regex(alternatives.as_slice())
        }

        proptest! {
            #[test]
            fn test_convert_from_string_then_to_string_is_original_string(
                english_num_str in format!(
                    "(OneThousand|{sub_100_regex})?(Hundred)?(And)?{sub_100_regex}?{sub_100_regex}?)",
                    sub_100_regex = get_zero_small_nums_and_tens_regex())
                )
            {
                let maybe_num = get_number_for_pascal_case_string(dbg!(english_num_str.as_str()));
                if maybe_num.is_ok() {
                    let reconverted_str = get_pascal_case_string(maybe_num.unwrap());
                    assert_eq!(english_num_str, reconverted_str.unwrap());
                };
            }
        }
    }
}