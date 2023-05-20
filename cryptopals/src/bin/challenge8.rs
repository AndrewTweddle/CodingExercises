use std::collections::HashMap;
use std::fs;

struct Candidate<'a> {
    index: usize,
    line: &'a str,
    score: i64,
    block_counts: HashMap<&'a str, usize>,
}

fn main() {
    let contents = fs::read_to_string("data/8.txt").expect("Unable to read file 8.txt");
    let mut candidates: Vec<Candidate> = contents
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let block_count = line.len() / 16;
            let mut block_counts = HashMap::<&str, usize>::with_capacity(block_count);
            let mut next_index: usize = 0;
            for _ in 0..block_count {
                let start_index = next_index;
                next_index += 16;
                let block = &line[start_index..next_index];
                let count = block_counts.entry(block).or_default();
                *count += 1;
            }
            // Any duplicate blocks are indicative of encryption with an ECB.
            // Use the square of the counts to place a greater weight on lots of duplicates.
            let sum_of_squared_counts: i64 =
                block_counts.values().map(|&cnt| (cnt * cnt) as i64).sum();
            Candidate {
                index,
                line,
                score: sum_of_squared_counts,
                block_counts,
            }
        })
        .collect();
    candidates.sort_by_key(|candidate| -candidate.score);

    if let Some(best_candidate) = candidates.first() {
        println!("Best candidate:");
        println!(
            "  Line #{}: {}",
            best_candidate.index + 1,
            best_candidate.line
        );
        println!("  Score:     {}", best_candidate.score);
        println!("  Blocks:    {:?}", best_candidate.block_counts);
    } else {
        println!("No encrypted AES-128-ECB text found!");
    }
}
