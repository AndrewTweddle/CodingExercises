fn main() {
    println!("Hello, world!");
}

pub fn convert_to_roman(num: u16) -> String {
    let mut roman = String::from("");
    if num < 4 {
        append_repeating_numerals(&mut roman, 'I', num as usize);
    } else if num == 4 {
        return "IV".to_string();
    } else if num < 9 {
        roman.reserve(num as usize - 4);
        roman.push('V');
        append_repeating_numerals(&mut roman, 'I', num as usize - 5);
    } else if num == 9 {
        return "IX".to_string();
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