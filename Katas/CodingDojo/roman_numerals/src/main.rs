fn main() {
    println!("Hello, world!");
}

pub fn convert_to_roman(num: u16) -> String {
    if num < 4 {
        "I".to_string().repeat(num as usize)
    } else {
        "".to_string()
    }
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
    }
}