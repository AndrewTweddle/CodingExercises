use std::time::Instant;
use regex::Regex;

fn main() {
    let start = Instant::now();

    let mut max_len = 0;
    let mut longest_number = 0;
    let mut longest_roman = "".to_string();
    for i in 1..=3000 {
        let roman = convert_to_roman(i).unwrap();
        if roman.len() > max_len {
            max_len = roman.len();
            longest_roman = roman.clone();
            longest_number = i;
        }
        println!("{} = {}", i, roman);
    }
    println!("longest number: {} = {} ({} numerals)", longest_number, longest_roman, max_len);

    let duration = start.elapsed();
    println!("Duration to print all Roman numbers from 1 to 3000: {:?}", duration);
    print!("Average duration per number: {:?}", duration / 3000);
}

pub fn convert_to_roman(mut num: u16) -> Result<String, &'static str> {
    if num == 0 {
        return Err("Zero cannot be converted to a Roman numeral");
    }
    if num > 3000 {
        return Err("Roman numerals above 3000 are not supported");
    }

    let thousands_digit = num / 1000;
    num %= 1000;
    let hundreds_digit = num / 100;
    num %= 100;
    let tens_digit = num / 10;
    let units_digit = num % 10;

    let mut roman = String::from("");
    append_repeating_numerals(&mut roman, 'M', thousands_digit as usize);
    append_decimal_digit_to_roman_numeral_representation(&mut roman, hundreds_digit, 'C', 'D', 'M');
    append_decimal_digit_to_roman_numeral_representation(&mut roman, tens_digit, 'X', 'L', 'C');
    append_decimal_digit_to_roman_numeral_representation(&mut roman, units_digit, 'I', 'V', 'X');

    Ok(roman)
}

fn append_decimal_digit_to_roman_numeral_representation(roman: &mut String,
    decimal_digit: u16, one_numeral: char, five_numeral: char, ten_numeral: char) {

    let mod_5 = decimal_digit % 5;
    if mod_5 == 4 {
        roman.push(one_numeral);
        roman.push(if decimal_digit < 5 { five_numeral } else { ten_numeral });
    } else {
        if decimal_digit >= 5 { roman.push(five_numeral); }
        append_repeating_numerals(roman, one_numeral, mod_5 as usize);
    }
}

fn append_repeating_numerals(prefix: &mut String, repeating_numeral: char, repeat_count: usize) {
    prefix.push_str(repeating_numeral.to_string().repeat(repeat_count).as_str());
}

pub fn convert_from_roman(roman: &str) -> Result<u16, &'static str> {
    if roman.is_empty() {
        return Err("An empty string is not a Roman numeral")
    }

    let re = get_roman_numeral_regex();

    let caps = match re.captures(roman) {
        Some(valid_caps) => valid_caps,
        None => {
            return Err("Invalid Roman numeral format");
        }
    };

    let thousands_capture = caps.name("thousands").expect("Thousands capture group not found");
    let hundreds_capture = caps.name("hundreds").expect("Hundreds capture group not found");
    let tens_capture = caps.name("tens").expect("Tens capture group not found");
    let units_capture = caps.name("units").expect("Units capture group not found");

    let thousands = thousands_capture.as_str().len() as u16;
    let hundreds = convert_from_roman_digit(hundreds_capture.as_str(), "CD", 'D', "CM");
    let tens = convert_from_roman_digit(tens_capture.as_str(), "XL", 'L', "XC");
    let units = convert_from_roman_digit(units_capture.as_str(), "IV", 'V', "IX");

    let number = 1000 * thousands + 100 * hundreds + 10 * tens + units;
    if number > 3000 {
        // Some valid patterns are out of range
        Err("Roman numerals above 3000 are not supported")
    } else {
        Ok(number)
    }
}

fn convert_from_roman_digit(roman_digit: &str,
                            four_numeral: &str, five_numeral: char, nine_numeral: &str) -> u16
{
    if roman_digit == four_numeral {
        4
    } else if roman_digit == nine_numeral {
        9
    } else if roman_digit.starts_with(five_numeral) {
        4 + roman_digit.len() as u16
    } else {
        // e.g. I, II, II, X, XX, XXX, ...
        roman_digit.len() as u16
    }
}

pub fn is_roman_numeral(roman: &str) -> bool {
    if roman == "VV" {
        true
    }
    else if roman.is_empty() {
        false
    } else if roman.starts_with("MMM") && roman.len() > 3 {
        false
    } else {
        let re = get_roman_numeral_regex();
        re.is_match(roman)
    }
}

fn get_roman_numeral_regex() -> Regex {
    let thousands_re = "(?P<thousands>M{0,3})";
    let hundreds_re = "(?P<hundreds>(CD|D?C{0,3}|CM)?)";
    let tens_re = "(?P<tens>(XL|L?X{0,3}|XC)?)";
    let units_re = "(?P<units>(IV|V?I{0,3}|IX)?)";
    let re_pattern = format!("^{}{}{}{}$", thousands_re, hundreds_re, tens_re, units_re);
    Regex::new(re_pattern.as_str()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{convert_to_roman, convert_from_roman, is_roman_numeral};
    use quickcheck::{TestResult, Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    /// Check that the function that converts from a Roman numeral
    /// is the left inverse of the function that converts to a Roman numeral
    #[quickcheck]
    fn check_convert_from_roman_is_left_inverse_of_convert_to_roman(num: u16) -> TestResult {
        if num == 0 || num > 3000 {
            return TestResult::discard();
        }
        TestResult::from_bool(
            num == convert_from_roman(
                convert_to_roman(num)
                    .unwrap()
                    .as_str()
            ).unwrap()
        )
    }

    #[derive(Clone, Debug)]
    struct RomanString(String);

    #[derive(Clone)]
    struct RomanChar(&'static char);

    // Select from the usual characters in a Roman number, plus one extra impostor
    const ROMAN_CHARS: [char; 8] = ['I', 'V', 'X', 'L', 'C', 'D', 'M', 'Z'];

    impl Arbitrary for RomanChar {
        fn arbitrary(g: &mut Gen) -> Self {
            RomanChar(g
                .choose::<char>(&ROMAN_CHARS)
                .unwrap()
            )
        }
    }

    impl Arbitrary for RomanString {
        fn arbitrary(g: &mut Gen) -> Self {
            let vec_roman_chars: Vec<RomanChar> = Vec::<RomanChar>::arbitrary(g);
            let roman_char_string: String = vec_roman_chars
                .iter()
                .fold(String::new(), |mut roman_str, roman_char| {
                    roman_str.push(*roman_char.0);
                    roman_str
                });
            RomanString(roman_char_string)
        }
    }

    /// Check that the function that converts to a Roman numeral
    /// is the inverse of the function that converts from a Roman numeral
    #[quickcheck]
    fn check_convert_to_roman_is_left_inverse_of_convert_from_roman(
        roman_numeral: RomanString) -> TestResult
    {
        if !is_roman_numeral(roman_numeral.0.as_str()) {
            return TestResult::discard();
        }
        TestResult::from_bool(
            roman_numeral.0 == convert_to_roman(
                convert_from_roman(roman_numeral
                    .0
                    .as_str())
                    .unwrap()
            ).unwrap()
        )
    }

    mod to_roman {
        use super::convert_to_roman;

        #[test]
        fn test_below_four() {
            assert_eq!(convert_to_roman(1).unwrap(), "I".to_string());
            assert_eq!(convert_to_roman(2).unwrap(), "II".to_string());
            assert_eq!(convert_to_roman(3).unwrap(), "III".to_string());
        }

        #[test]
        fn test_four() {
            assert_eq!(convert_to_roman(4).unwrap(), "IV".to_string());
        }

        #[test]
        fn test_five_to_eight() {
            assert_eq!(convert_to_roman(5).unwrap(), "V".to_string());
            assert_eq!(convert_to_roman(6).unwrap(), "VI".to_string());
            assert_eq!(convert_to_roman(7).unwrap(), "VII".to_string());
            assert_eq!(convert_to_roman(8).unwrap(), "VIII".to_string());
        }

        #[test]
        fn test_nine() {
            assert_eq!(convert_to_roman(9).unwrap(), "IX".to_string());
        }

        #[test]
        fn test_multiples_of_tens() {
            assert_eq!(convert_to_roman(10).unwrap(), "X".to_string());
            assert_eq!(convert_to_roman(20).unwrap(), "XX".to_string());
            assert_eq!(convert_to_roman(30).unwrap(), "XXX".to_string());
            assert_eq!(convert_to_roman(40).unwrap(), "XL".to_string());
            assert_eq!(convert_to_roman(50).unwrap(), "L".to_string());
            assert_eq!(convert_to_roman(60).unwrap(), "LX".to_string());
            assert_eq!(convert_to_roman(70).unwrap(), "LXX".to_string());
            assert_eq!(convert_to_roman(80).unwrap(), "LXXX".to_string());
            assert_eq!(convert_to_roman(90).unwrap(), "XC".to_string());
        }

        #[test]
        fn test_tens_and_units() {
            assert_eq!(convert_to_roman(14).unwrap(), "XIV".to_string());
            assert_eq!(convert_to_roman(49).unwrap(), "XLIX".to_string());
            assert_eq!(convert_to_roman(59).unwrap(), "LIX".to_string());
            assert_eq!(convert_to_roman(89).unwrap(), "LXXXIX".to_string());
            assert_eq!(convert_to_roman(91).unwrap(), "XCI".to_string());
            assert_eq!(convert_to_roman(98).unwrap(), "XCVIII".to_string());
            assert_eq!(convert_to_roman(99).unwrap(), "XCIX".to_string());
        }

        #[test]
        fn test_hundreds_tens_and_units() {
            assert_eq!(convert_to_roman(104).unwrap(), "CIV".to_string());
            assert_eq!(convert_to_roman(449).unwrap(), "CDXLIX".to_string());
            assert_eq!(convert_to_roman(500).unwrap(), "D".to_string());
            assert_eq!(convert_to_roman(644).unwrap(), "DCXLIV".to_string());
            assert_eq!(convert_to_roman(889).unwrap(), "DCCCLXXXIX".to_string());
            assert_eq!(convert_to_roman(991).unwrap(), "CMXCI".to_string());
            assert_eq!(convert_to_roman(998).unwrap(), "CMXCVIII".to_string());
            assert_eq!(convert_to_roman(999).unwrap(), "CMXCIX".to_string());
        }

        #[test]
        fn test_multiples_of_thousand_up_to_3000() {
            assert_eq!(convert_to_roman(1000).unwrap(), "M".to_string());
            assert_eq!(convert_to_roman(2000).unwrap(), "MM".to_string());
            assert_eq!(convert_to_roman(3000).unwrap(), "MMM".to_string());
        }

        #[test]
        fn test_thousands_hundreds_tens_and_units() {
            assert_eq!(convert_to_roman(1972).unwrap(), "MCMLXXII".to_string());
            assert_eq!(convert_to_roman(2999).unwrap(), "MMCMXCIX".to_string());
        }

        #[test]
        #[should_panic]
        fn test_zero_not_supported() {
            convert_to_roman(0).unwrap();
        }

        #[test]
        #[should_panic]
        fn test_above_3000_not_supported() {
            convert_to_roman(3001).unwrap();
        }
    }

    #[allow(non_snake_case)]
    mod from_roman {
        use super::{convert_from_roman, is_roman_numeral};

        #[test]
        fn test_below_four() {
            assert_eq!(convert_from_roman("I"), Ok(1));
            assert_eq!(convert_from_roman("II"), Ok(2));
            assert_eq!(convert_from_roman("III"), Ok(3));
        }

        #[test]
        fn test_converting_from_IIII() {
            assert!(convert_from_roman("IIII").is_err());
        }

        #[test]
        fn test_converting_from_IV() {
            assert_eq!(convert_from_roman("IV"), Ok(4));
        }

        #[test]
        fn test_converting_from_V() {
            assert_eq!(convert_from_roman("V"), Ok(5));
        }

        #[test]
        fn test_converting_from_VIII() {
            assert_eq!(convert_from_roman("VIII"), Ok(8));
        }

        #[test]
        fn test_converting_from_VIIII_fails() {
            assert!(convert_from_roman("VIIII").is_err());
        }

        #[test]
        fn test_converting_from_IX() {
            assert_eq!(convert_from_roman("IX"), Ok(9));
        }

        #[test]
        fn test_converting_from_X() {
            assert_eq!(convert_from_roman("X"), Ok(10));
        }

        #[test]
        fn test_converting_from_empty_string() {
            assert!(convert_from_roman("").is_err());
        }

        #[test]
        fn test_converting_from_XXIX() {
            assert_eq!(convert_from_roman("XXIX"), Ok(29));
        }

        #[test]
        fn test_converting_from_DCXLIV() {
            assert_eq!(convert_from_roman("DCXLIV"), Ok(644));
        }

        #[test]
        fn test_converting_from_MMCMLXXIV() {
            assert_eq!(convert_from_roman("MMCMLXXIV"), Ok(2974));
        }

        #[test]
        fn test_converting_from_MMMI_fails() {
            assert!(convert_from_roman("MMMI").is_err());
        }

        #[test]
        fn test_is_roman_numeral_on_empty_string() {
            assert!(!is_roman_numeral(""));
        }

        #[test]
        fn test_is_roman_numeral_on_IIII() {
            assert!(!is_roman_numeral("IIII"));
        }

        #[test]
        fn test_is_roman_numeral_on_IV() {
            assert!(is_roman_numeral("IV"));
        }

        #[test]
        fn test_is_roman_numeral_on_IXCM() {
            assert!(!is_roman_numeral("IXCM"));
        }

        #[test]
        fn test_is_roman_numeral_on_MMM() {
            assert!(is_roman_numeral("MMM"));
        }

        /// Test that Roman numerals above 3000 are not valid
        /// (since we wish to_roman_numeral and from_roman_numeral to be inverses of one another,
        /// so the domain of each must match the range of the other)
        #[test]
        fn test_is_roman_numeral_on_MMMI() {
            assert!(!is_roman_numeral("MMMI"));
        }
    }
}
