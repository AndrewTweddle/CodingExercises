use std::time::Instant;

fn main() {
    let start = Instant::now();

    for i in 1..=3000_u16 {
        let roman = convert_to_roman(i).unwrap();
        println!("{} = {}", i, roman);

        // Check that the reverse conversion works too...
        let j = convert_from_roman(roman.as_str()).unwrap();
        if i != j {
            println!("ERROR during reverse conversion: expected {}, actual {}", i, j)
        }
    }

    let duration = start.elapsed();
    println!("Total duration: {:?}", duration);
    println!("Duration per number: {:?}", duration / 3000);

    let edge_case = "VIV";
    let value_of_edge_case = convert_from_roman(edge_case).unwrap();
    println!("Validation issue: conversion thinks that {} = {}", edge_case, value_of_edge_case);
}

struct Pattern {
    pattern: &'static str,
    value: u16,
    max_repetitions: usize
}

const PATTERNS: [Pattern; 13] = [
    Pattern{ pattern: "M",  value: 1000, max_repetitions: 3 },
    Pattern{ pattern: "CM", value:  900, max_repetitions: 1 },
    Pattern{ pattern: "D",  value:  500, max_repetitions: 1 },
    Pattern{ pattern: "CD", value:  400, max_repetitions: 1 },
    Pattern{ pattern: "C",  value:  100, max_repetitions: 3 },
    Pattern{ pattern: "XC", value:   90, max_repetitions: 1 },
    Pattern{ pattern: "L",  value:   50, max_repetitions: 1 },
    Pattern{ pattern: "XL", value:   40, max_repetitions: 1 },
    Pattern{ pattern: "X",  value:   10, max_repetitions: 3 },
    Pattern{ pattern: "IX", value:    9, max_repetitions: 1 },
    Pattern{ pattern: "V",  value:    5, max_repetitions: 1 },
    Pattern{ pattern: "IV", value:    4, max_repetitions: 1 },
    Pattern{ pattern: "I",  value:    1, max_repetitions: 3 },
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
    let mut start_pos = 0;
    'pat_loop: for pat in &PATTERNS {
        for _ in 0..pat.max_repetitions {
            let rem_str = &roman[start_pos..];
            if rem_str.starts_with(pat.pattern) {
                num += pat.value;
                start_pos += &pat.pattern.len();
                if start_pos == roman.len() { break 'pat_loop; }
            } else {
                break;
            }
        }
    }
    if start_pos == roman.len() {
        Ok(num)
    } else {
        Err("Invalid Roman number format")
    }
}
