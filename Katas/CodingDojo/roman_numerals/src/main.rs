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
    let re = Regex::new(r"^(I{1,3}|IV|VI{0,3})$").unwrap();

    if re.is_match(roman) {
        match roman {
            "IV" => Ok(4),
            five_up if five_up.starts_with('V') => Ok(4 + roman.len() as u16),
            _ => Ok(roman.len() as u16)
        }
    } else {
        Err("Invalid Roman numeral format")
    }
}

#[cfg(test)]
mod tests {
    use super::{convert_to_roman, convert_from_roman};

    mod from_roman {
        use super::convert_from_roman;

        #[test]
        fn test_below_four() {
            assert_eq!(convert_from_roman("I"), Ok(1));
            assert_eq!(convert_from_roman("II"), Ok(2));
            assert_eq!(convert_from_roman("III"), Ok(3));
        }

        #[test]
        #[allow(non_snake_case)]
        fn test_converting_from_IIII() {
            assert!(convert_from_roman("IIII").is_err());
        }

        #[test]
        #[allow(non_snake_case)]
        fn test_converting_from_IV() {
            assert_eq!(convert_from_roman("IV"), Ok(4));
        }

        #[test]
        #[allow(non_snake_case)]
        fn test_converting_from_V() {
            assert_eq!(convert_from_roman("V"), Ok(5));
        }

        #[test]
        #[allow(non_snake_case)]
        fn test_converting_from_VIII() {
            assert_eq!(convert_from_roman("VIII"), Ok(8));
        }
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
}