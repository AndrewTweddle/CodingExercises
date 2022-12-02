use std::fs;

fn main() {
    let contents = fs::read_to_string("data/day1_input.txt").unwrap();
    let max_sum: u32 = contents
        .split("\n\n")
        .map(|inventories_str| inventories_str
            .lines()
            .map(|calories_str| calories_str.parse::<u32>().unwrap())
            .max()
            .unwrap()
        )
        .max()
        .unwrap();

    println!("Day 1 part 1 answer: {max_sum}");
}