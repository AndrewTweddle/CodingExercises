use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::collections::HashSet;

const INPUT_FILE_PATH: &str = "data/day5_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 5 part 1", solve, 10);
}

fn solve(contents: &str) -> i64 {
    let mut line_iter = contents.lines();
    let rules: HashSet<(i64, i64)> = (&mut line_iter)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (lval_str, rval_str) = line.split_once('|').unwrap();
            let left = lval_str.trim().parse::<i64>().unwrap();
            let right = rval_str.trim().parse::<i64>().unwrap();
            (left, right)
        })
        .collect();

    line_iter
        .filter_map(|line| {
            let pages: Vec<i64> = line
                .split(',')
                .map(|page_str| page_str.parse::<i64>().unwrap())
                .collect();
            let no_rules_violated = pages.iter().enumerate().all(|(i, &page)| {
                (&pages[0..i])
                    .iter()
                    .all(|&other_page| !rules.contains(&(page, other_page)))
            });
            no_rules_violated.then(|| pages[pages.len() / 2])
        })
        .sum()
}
