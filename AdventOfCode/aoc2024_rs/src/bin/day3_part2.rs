use aoc2024_rs::read_and_solve_and_time_more_runs;
use regex::Regex;

const INPUT_FILE_PATH: &str = "data/day3_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 3 part 2", solve, 1000);
}

fn solve(contents: &str) -> i64 {
    let re = Regex::new(r"(do\(\))|(don't\(\))|mul\((\d{1,3},\d{1,3})\)").unwrap();
    let mut enabled = true;
    re.captures_iter(contents)
        .filter_map(|instructions| {
            let (_, [instruction]) = instructions.extract();
            match instruction {
                "do()" => {
                    enabled = true;
                    None
                }
                "don't()" => {
                    enabled = false;
                    None
                }
                num_pair => {
                    if enabled {
                        let (num1_str, num2_str) = num_pair.split_once(",").unwrap();
                        let num1: i64 = num1_str.parse().unwrap();
                        let num2: i64 = num2_str.parse().unwrap();
                        Some(num1 * num2)
                    } else {
                        None
                    }
                }
            }
        })
        .sum::<i64>()
}
