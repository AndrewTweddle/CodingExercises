use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::collections::hash_map::Entry::Occupied;
use std::collections::{HashMap, HashSet};

const INPUT_FILE_PATH: &str = "data/day5_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 5 part 2", solve, 100);
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
            let any_rules_violated = pages.iter().enumerate().any(|(i, &page)| {
                pages[0..i]
                    .iter()
                    .any(|&other_page| rules.contains(&(page, other_page)))
            });
            if any_rules_violated {
                // Set up a pair of mappings between each page and its preceding pages
                // and each page and its successor pages (based on the rules):
                let mut preceders: HashMap<i64, Vec<i64>> =
                    pages.iter().map(|page| (*page, Vec::new())).collect();
                let mut successors = preceders.clone();

                for &(earlier_page, later_page) in &rules {
                    // Only consider a rule if both of its pages are part of the current "update":
                    if let Occupied(mut precs_of_later) = preceders.entry(later_page) {
                        if let Occupied(mut succs_of_earlier) = successors.entry(earlier_page) {
                            precs_of_later.get_mut().push(earlier_page);
                            succs_of_earlier.get_mut().push(later_page);
                        }
                    }
                }

                // Build up the sequence using a topological sort:
                pages.clear();

                while let Some((&page, _)) = preceders.iter().find(|(_, precs)| precs.is_empty()) {
                    pages.push(page);

                    // Remove the page from both maps, as its precedence constraints are satisfied:
                    for successor in successors.get(&page).unwrap().iter() {
                        let succ_precs = preceders.get_mut(successor).unwrap();
                        succ_precs.retain(|&pg| pg != page);
                    }
                    preceders.remove(&page);
                }

                // Return the middle element of the sorted pages
                Some(pages[pages.len() / 2])
            } else {
                None
            }
        })
        .sum()
}
