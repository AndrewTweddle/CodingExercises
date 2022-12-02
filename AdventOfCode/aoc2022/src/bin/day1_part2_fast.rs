use std::fs;
use std::time::Instant;

const REPETITIONS: u32 = 10_000;

fn main() {
    let mut start_time = Instant::now();
    let contents = fs::read_to_string("data/day1_input.txt").unwrap();

    for rep in 0..=REPETITIONS {
        let mut max_inventories = [0_u32; 3];
        let inv_iter = contents.split("\n\n").map(|inventories_str| {
            inventories_str
                .lines()
                .map(|calories_str| calories_str.parse::<u32>().unwrap())
                .sum::<u32>()
        });
        for inv in inv_iter {
            if inv > max_inventories[0] {
                max_inventories[0] = inv;
                max_inventories.sort();
            }
        }

        let sum_of_top_3: u32 = max_inventories.iter().sum();

        if rep == 0 {
            println!("Day 1 part 2 answer: {sum_of_top_3}");
            println!(
                "Duration of iteration 0 including I/O: {:?}",
                start_time.elapsed()
            );

            // Restart timer, so that I/O is not being timed
            start_time = Instant::now();
        }
    }
    let duration = start_time.elapsed();
    println!(
        "Avg duration of {} repetitions (excluding I/O): {:?}",
        REPETITIONS,
        duration / REPETITIONS
    );
}
