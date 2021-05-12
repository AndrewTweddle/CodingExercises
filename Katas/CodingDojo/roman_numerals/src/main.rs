fn main() {
    println!("Hello, world!");
}

pub fn convert_to_roman(num: u16) -> String {
    if num < 10 {
        convert_units_to_roman(num)
    } else {
        convert_tens_to_roman(num / 10)
    }
}

fn convert_tens_to_roman(num_tens: u16) -> String {
    let mut roman = String::from("");

    let mod_5 = num_tens % 5;

    if mod_5 == 4 {
        roman.push('X');
        roman.push(if num_tens < 5 { 'L' } else { 'C' });
    } else {
        if num_tens >= 5 { roman.push('L'); }
        append_repeating_numerals(&mut roman, 'X', mod_5 as usize);
    }
    roman
}

fn convert_units_to_roman(num: u16) -> String {
    let mut roman = String::from("");

    let mod_5 = num % 5;

    if mod_5 == 4 {
        roman.push('I');
        roman.push(if num < 5 { 'V' } else { 'X' });
    } else {
        if num >= 5 { roman.push('V'); }
        append_repeating_numerals(&mut roman, 'I', mod_5 as usize);
    }
    roman
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
    }
}