fn main() {
    println!("Hello, world!");
}

pub fn convert_to_roman(num: u16) -> String {
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
    }
}