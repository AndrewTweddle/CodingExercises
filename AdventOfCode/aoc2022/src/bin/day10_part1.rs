use std::fs;

const ADDX_STR: &str = "addx ";
const NOOP_STR: &str = "noop";

fn main() {
    let contents = fs::read_to_string("data/day10_input.txt").unwrap();
    let mut sum_of_signal_strengths = 0;
    let mut cycle = 0;
    let mut x = 1;
    for line in contents.lines() {
        if line.starts_with(ADDX_STR) {
            let value = line[ADDX_STR.len()..]
                .parse::<i64>()
                .expect(format!("addx not a followed by an integer: {}", line).as_str());
            increment_cycle_and_update_sum(&x, &mut cycle, &mut sum_of_signal_strengths);
            increment_cycle_and_update_sum(&x, &mut cycle, &mut sum_of_signal_strengths);
            x += value;
        } else if line == NOOP_STR {
            increment_cycle_and_update_sum(&x, &mut cycle, &mut sum_of_signal_strengths);
        } else {
            panic!("Unexpected instruction {line}");
        }
    }
    println!("Part 1 answer: {sum_of_signal_strengths}")
}

fn increment_cycle_and_update_sum(x: &i64, cycle: &mut i64, sum_of_signal_strengths: &mut i64) {
    *cycle += 1;
    if *cycle % 40 == 20 {
        *sum_of_signal_strengths += *cycle * x;
    }
}
