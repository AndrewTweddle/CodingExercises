use std::time::Instant;

fn main() {
    let start = Instant::now();

    for i in 1..=3000 {
        println!("{} = {}", i, convert_to_roman(i))
    }

    let duration = start.elapsed();
    println!("Duration to print all Roman numbers from 1 to 3000: {:?}", duration);
    print!("Average duration per number: {:?}", duration / 3000);
}

pub fn convert_to_roman(mut num: u16) -> String {
    let mut roman = String::from("");

    let thousands_digit = num / 1000;
    num %= 1000;
    let hundreds_digit = num / 100;
    num %= 100;
    let tens_digit = num / 10;
    let units_digit = num % 10;

    append_repeating_numerals(&mut roman, 'M', thousands_digit as usize);
    append_decimal_digit_to_roman_numeral_representation(&mut roman, hundreds_digit, 'C', 'D', 'M');
    append_decimal_digit_to_roman_numeral_representation(&mut roman, tens_digit, 'X', 'L', 'C');
    append_decimal_digit_to_roman_numeral_representation(&mut roman, units_digit, 'I', 'V', 'X');

    roman
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

#[cfg(test)]
mod tests {
    use super::convert_to_roman;

    mod from_roman {

    }

    mod to_roman {
        use super::convert_to_roman;

        #[test]
        fn test_below_four() {
            assert_eq!(convert_to_roman(1), "I".to_string());
            assert_eq!(convert_to_roman(2), "II".to_string());
            assert_eq!(convert_to_roman(3), "III".to_string());
        }

        #[test]
        fn test_four() {
            assert_eq!(convert_to_roman(4), "IV".to_string());
        }

        #[test]
        fn test_five_to_eight() {
            assert_eq!(convert_to_roman(5), "V".to_string());
            assert_eq!(convert_to_roman(6), "VI".to_string());
            assert_eq!(convert_to_roman(7), "VII".to_string());
            assert_eq!(convert_to_roman(8), "VIII".to_string());
        }

        #[test]
        fn test_nine() {
            assert_eq!(convert_to_roman(9), "IX".to_string());
        }

        #[test]
        fn test_multiples_of_tens() {
            assert_eq!(convert_to_roman(10), "X".to_string());
            assert_eq!(convert_to_roman(20), "XX".to_string());
            assert_eq!(convert_to_roman(30), "XXX".to_string());
            assert_eq!(convert_to_roman(40), "XL".to_string());
            assert_eq!(convert_to_roman(50), "L".to_string());
            assert_eq!(convert_to_roman(60), "LX".to_string());
            assert_eq!(convert_to_roman(70), "LXX".to_string());
            assert_eq!(convert_to_roman(80), "LXXX".to_string());
            assert_eq!(convert_to_roman(90), "XC".to_string());
        }

        #[test]
        fn test_tens_and_units() {
            assert_eq!(convert_to_roman(14), "XIV".to_string());
            assert_eq!(convert_to_roman(49), "XLIX".to_string());
            assert_eq!(convert_to_roman(59), "LIX".to_string());
            assert_eq!(convert_to_roman(89), "LXXXIX".to_string());
            assert_eq!(convert_to_roman(91), "XCI".to_string());
            assert_eq!(convert_to_roman(98), "XCVIII".to_string());
            assert_eq!(convert_to_roman(99), "XCIX".to_string());
        }

        #[test]
        fn test_hundreds_tens_and_units() {
            assert_eq!(convert_to_roman(104), "CIV".to_string());
            assert_eq!(convert_to_roman(449), "CDXLIX".to_string());
            assert_eq!(convert_to_roman(500), "D".to_string());
            assert_eq!(convert_to_roman(644), "DCXLIV".to_string());
            assert_eq!(convert_to_roman(889), "DCCCLXXXIX".to_string());
            assert_eq!(convert_to_roman(991), "CMXCI".to_string());
            assert_eq!(convert_to_roman(998), "CMXCVIII".to_string());
            assert_eq!(convert_to_roman(999), "CMXCIX".to_string());
        }

        #[test]
        fn test_multiples_of_thousand_up_to_3000() {
            assert_eq!(convert_to_roman(1000), "M".to_string());
            assert_eq!(convert_to_roman(2000), "MM".to_string());
            assert_eq!(convert_to_roman(3000), "MMM".to_string());
        }

        #[test]
        fn test_thousands_hundreds_tens_and_units() {
            assert_eq!(convert_to_roman(1972), "MCMLXXII".to_string());
            assert_eq!(convert_to_roman(2999), "MMCMXCIX".to_string());
        }
    }
}