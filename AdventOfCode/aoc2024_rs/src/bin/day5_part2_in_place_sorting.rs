use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::collections::HashSet;

const INPUT_FILE_PATH: &str = "data/day5_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 5 part 2", solve, 10);
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
            let mut pages: Vec<i64> = line
                .split(',')
                .map(|page_str| page_str.parse::<i64>().unwrap())
                .collect();
            let mut any_rules_violated = false;
            for i in 0..pages.len() {
                let mut page = pages[i];
                for j in 0..i {
                    let earlier_page = pages[j];
                    if rules.contains(&(page, earlier_page)) {
                        any_rules_violated = true;
                        pages.swap(i, j);
                        page = earlier_page;
                    }
                }
            }

            if any_rules_violated {
                Some(pages[pages.len() / 2])
            } else {
                None
            }
        })
        .sum()
}
