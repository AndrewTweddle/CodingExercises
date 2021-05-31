use std::time::Instant;

fn main() {
    let start = Instant::now();

    for i in 1..=3000_u16 {
        let roman = convert_to_roman(i).unwrap();
        println!("{} = {}", i, roman);

        // Check that the reverse conversion works too...
        let is_valid = is_roman(roman.as_str());
        if !is_valid {
            println!("Invalid Roman number: {}", roman);
        }

        let j = convert_from_roman(roman.as_str()).unwrap();
        if i != j {
            println!("ERROR during reverse conversion: expected {}, actual {}", i, j)
        }
    }

    let duration = start.elapsed();
    println!("Total duration: {:?}", duration);
    println!("Duration per number: {:?}", duration / 3000);

    let edge_case = "VIV";
    let result_of_edge_case = convert_from_roman(edge_case);
    match result_of_edge_case {
        Ok(value_of_edge_case) =>
            println!("Validation issue: conversion thinks that {} = {} (for example)",
                     edge_case, value_of_edge_case),
        Err(e) => println!("As expected, an error occurred processing {}: {}", edge_case, e),
    }
    println!("is_roman(\"{}\") = {}", edge_case, is_roman(edge_case));
}

struct Pattern {
    pattern: &'static str,
    value: u16,
    max_repetitions: usize,
    steps_to_skip: usize,
}

const PATTERNS: [Pattern; 13] = [
    Pattern { pattern: "M",  value: 1000, max_repetitions: 3, steps_to_skip: 0 },
    Pattern { pattern: "CM", value:  900, max_repetitions: 1, steps_to_skip: 3 },
    Pattern { pattern: "D",  value:  500, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "CD", value:  400, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "C",  value:  100, max_repetitions: 3, steps_to_skip: 0 },
    Pattern { pattern: "XC", value:   90, max_repetitions: 1, steps_to_skip: 3 },
    Pattern { pattern: "L",  value:   50, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "XL", value:   40, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "X",  value:   10, max_repetitions: 3, steps_to_skip: 0 },
    Pattern { pattern: "IX", value:    9, max_repetitions: 1, steps_to_skip: 3 },
    Pattern { pattern: "V",  value:    5, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "IV", value:    4, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "I",  value:    1, max_repetitions: 3, steps_to_skip: 0 },
];

pub fn convert_to_roman(mut num: u16) -> Result<String, &'static str> {
    if num == 0 {
        return Err("Zero cannot be converted to a Roman numeral");
    }
    if num > 3000 {
        return Err("Roman numerals above 3000 are not supported");
    }
    let mut roman = String::new();
    for pat in &PATTERNS {
        while num >= pat.value {
            roman.push_str(pat.pattern);
            num -= pat.value;
        }
    }
    Ok(roman)
}

pub fn convert_from_roman(roman: &str) -> Result<u16, &'static str> {
    if roman.is_empty() {
        return Err("An empty string is not a Roman numeral")
    }
    let mut num = 0;
    let mut pat_index = 0;
    let mut rem_str = roman.get(..).unwrap();
    'pat_loop: while pat_index < PATTERNS.len() {
        let pat = &PATTERNS[pat_index];
        let mut pattern_matched = false;
        for _ in 0..pat.max_repetitions {
            if rem_str.starts_with(pat.pattern) {
                num += pat.value;
                rem_str = rem_str.get(pat.pattern.len()..).unwrap();
                if rem_str.is_empty() { break 'pat_loop; }
                pattern_matched = true;
            } else {
                break;
            }
        }
        // When certain patterns are matched,other patterns should be skipped over
        // e.g. if "XC" is matched, then "L", "XL" and "X" should all be skipped.
        pat_index += if pattern_matched { pat.steps_to_skip + 1 } else { 1 }
    }
    if !rem_str.is_empty() {
        Err("Invalid Roman number format")
    } else if num > 3000 {
        Err("Roman numerals above 3000 are not supported")
    } else {
        Ok(num)
    }
}

pub fn is_roman(roman: &str) -> bool {
    convert_from_roman(roman).is_ok()
}

// -----------------------------------------------------------------------------------------------
// Unit tests and property-based tests below copied verbatim from original roman_numerals project:

#[cfg(test)]
mod tests {
    use super::{convert_to_roman, convert_from_roman, is_roman};

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
        use super::{convert_from_roman, is_roman};

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
            assert!(!is_roman(""));
        }

        #[test]
        fn test_is_roman_numeral_on_IIII() {
            assert!(!is_roman("IIII"));
        }

        #[test]
        fn test_is_roman_numeral_on_IV() {
            assert!(is_roman("IV"));
        }

        #[test]
        fn test_is_roman_numeral_on_IXCM() {
            assert!(!is_roman("IXCM"));
        }

        #[test]
        fn test_is_roman_numeral_on_MMM() {
            assert!(is_roman("MMM"));
        }

        /// Test that Roman numerals above 3000 are not valid
        /// (since we wish to_roman_numeral and from_roman_numeral to be inverses of one another,
        /// so the domain of each must match the range of the other)
        #[test]
        fn test_is_roman_numeral_on_MMMI() {
            assert!(!is_roman("MMMI"));
        }

        /// This unit test covers a failing test case identified by the property-based tests.
        /// Since those are randomly generated and only run on demand,
        /// add a more traditional unit test to capture the failing case.
        /// Note: This is a new test added to this file, not copied from the original algorithm.
        #[test]
        fn test_is_roman_numeral_on_IVI() {
            assert!(!is_roman("IVI"));
        }
    }

    /// Create a normal test to exhaustively test that, for all valid inputs,
    /// test_convert_from_roman() is the left inverse of convert_to_roman().
    /// This is the same test as the disabled property-based test,
    /// check_convert_from_roman_is_left_inverse_of_convert_to_roman()
    #[test]
    fn test_converting_all_ints_to_roman_then_from_roman() {
        (1_u16..=3000).for_each(|i| {
            let roman = convert_to_roman(i).unwrap();
            let j = convert_from_roman(roman.as_str()).unwrap();
            assert_eq!(i, j);
        });
    }

    mod quicktest_property_based_tests {
        use super::{convert_to_roman, convert_from_roman, is_roman};
        use quickcheck::{TestResult, Arbitrary, Gen};
        use quickcheck_macros::quickcheck;

        /// Check that the function that converts from a Roman numeral
        /// is the left inverse of the function that converts to a Roman numeral
        #[quickcheck]
        #[ignore = "Property-based tests are not deterministic"]
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

            fn shrink(&self) -> Box<dyn Iterator<Item=Self>> {
                let vec: Vec<char> = self.0.as_str().chars().collect();
                Box::new(
                    vec.shrink()
                        .map(|v| {
                            let shrunk_string = v.into_iter().collect::<String>();
                            RomanString(shrunk_string)
                        }))
            }
        }

        /// Check that the function that converts to a Roman numeral
        /// is the inverse of the function that converts from a Roman numeral
        #[quickcheck]
        #[ignore = "Property-based tests are not deterministic. Runs slowly."]
        fn check_convert_to_roman_is_left_inverse_of_convert_from_roman(
            roman_numeral: RomanString) -> TestResult
        {
            let roman = roman_numeral.0;
            if !is_roman(roman.as_str()) {
                return TestResult::discard();
            }
            TestResult::from_bool(
                roman == convert_to_roman(
                    convert_from_roman(roman.as_str()).unwrap()
                ).unwrap()
            )
        }

        /// Check that the conversion from a Roman numeral succeeds
        /// iff the function to check validity of a Roman numeral passes
        #[quickcheck]
        #[ignore = "Property-based tests are not deterministic"]
        fn check_convert_from_roman_succeeds_iff_is_roman_numeral(
            roman_numeral: RomanString) -> bool
        {
            let roman_str = roman_numeral.0.as_str();
            is_roman(roman_str) == convert_from_roman(roman_str).is_ok()
        }
    }

    mod proptest_property_based_tests {
        use super::{convert_to_roman, convert_from_roman, is_roman};
        use proptest::prelude::*;

        proptest! {
            /// Check that the function that converts from a Roman numeral
            /// is the left inverse of the function that converts to a Roman numeral
            #[test]
            #[ignore = "This is covered by test_converting_all_ints_to_roman_then_from_roman"]
            fn test_convert_from_roman_is_left_inverse_of_convert_to_roman(num in 1..=3000_u16) {
                let reconverted_num = convert_from_roman(
                    convert_to_roman(num).unwrap().as_str()
                ).unwrap();
                assert_eq!(num, reconverted_num);
            }

            /// Check that the function that converts to a Roman numeral
            /// is the inverse of the function that converts from a Roman numeral
            #[test]
            fn test_convert_to_roman_is_left_inverse_of_convert_from_roman(
                roman in "(V?I{0,3}|IV|IX|L?X{0,3}|XL|XC|D?C{0,3}|CD|CM|M{0,3}|Z)+"
                    .prop_filter("ignore invalid Roman numerals", |r| is_roman(r.as_str())))
            {
                let roman_str = roman.as_str();
                let reconverted_roman = convert_to_roman(
                    convert_from_roman(roman_str).unwrap()
                ).unwrap();
                assert_eq!(roman, reconverted_roman);
            }

            /// Check that the conversion from a Roman numeral succeeds
            /// iff the function to check validity of a Roman numeral passes
            #[test]
            fn test_convert_from_roman_succeeds_iff_is_roman_numeral(
                roman in "(V?I{0,3}|IV|IX|L?X{0,3}|XL|XC|D?C{0,3}|CD|CM|M{0,3}|Z)+")
            {
                let roman_str = roman.as_str();
                assert_eq!(is_roman(roman_str), convert_from_roman(roman_str).is_ok());
            }
        }
    }
}
