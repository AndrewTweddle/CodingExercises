use std::collections::BTreeSet;
use std::fs;
use std::ops::RangeInclusive;
use std::time::Instant;

#[derive(PartialEq)]
enum Instruction {
    On,
    Off,
}

struct RebootStep {
    instruction: Instruction,
    ranges: [RangeInclusive<i64>; 3],
}

const NUM_REPETITIONS: u32 = 10;

fn main() {
    let mut start_time = Instant::now();
    let contents = fs::read_to_string("data/day22_input.txt").unwrap();
    for rep in 0..=NUM_REPETITIONS {
        // Parse the file contents
        let steps: Vec<RebootStep> = contents
            .lines()
            .map(|line| {
                let (instruction_str, coords_str) = line.split_once(' ').unwrap();
                let instruction = if instruction_str == "on" {
                    Instruction::On
                } else {
                    Instruction::Off
                };
                let ranges: Vec<RangeInclusive<i64>> = coords_str
                    .split(',')
                    .map(|part_str| {
                        let (start_str, end_str) = part_str
                            .split_once('=')
                            .unwrap()
                            .1
                            .split_once("..")
                            .unwrap();
                        RangeInclusive::new(
                            start_str.parse::<i64>().unwrap(),
                            end_str.parse::<i64>().unwrap(),
                        )
                    })
                    .collect();
                RebootStep {
                    instruction,
                    ranges: [ranges[0].clone(), ranges[1].clone(), ranges[2].clone()],
                }
            })
            .collect();

        // Find all x, y and z boundary values across all the steps
        let dim_vals: Vec<BTreeSet<i64>> = (0..3)
            .map(|dim| {
                steps
                    .iter()
                    .flat_map(|step| {
                        let range = &step.ranges[dim];
                        [*range.start(), *range.end() + 1]
                    })
                    .collect::<BTreeSet<i64>>()
            })
            .collect::<Vec<_>>();

        // Determine the indices for each dimension (x, y and z).
        // The cross join of these defines all block of cuboids
        // that definitely share the same on/off status...
        let x_count = dim_vals[0].len();
        let y_count = dim_vals[1].len();
        let z_count = dim_vals[2].len();
        let mut cuboids = vec![vec![vec![false; z_count]; y_count]; x_count];

        for step in steps {
            let dim_indices: Vec<(usize, usize)> = (0..3)
                .map(|dim| {
                    let start_index = dim_vals[dim]
                        .iter()
                        .position(|&val| *step.ranges[dim].start() == val)
                        .unwrap();
                    let end_index = dim_vals[dim]
                        .iter()
                        .position(|&val| *step.ranges[dim].end() + 1 == val)
                        .unwrap();
                    (start_index, end_index)
                })
                .collect();

            let (from_x, to_x) = dim_indices[0];
            let (from_y, to_y) = dim_indices[1];
            let (from_z, to_z) = dim_indices[2];
            for x in from_x..to_x {
                for y in from_y..to_y {
                    for z in from_z..to_z {
                        cuboids[x][y][z] = step.instruction == Instruction::On;
                    }
                }
            }
        }

        // Iterate through all blocks of cuboids calculating both their intersection
        // with the -50..=50 range (for each dimension) for part 1,
        // and their total volume for part 2, but only for blocks of cuboids that are switched on.
        let x_vals: Vec<i64> = dim_vals[0].iter().cloned().collect();
        let y_vals: Vec<i64> = dim_vals[1].iter().cloned().collect();
        let z_vals: Vec<i64> = dim_vals[2].iter().cloned().collect();

        let part1_min_val = -50;
        let part1_max_val = 50;

        let mut part1_total_on: i64 = 0;
        let mut part2_total_on: i64 = 0;

        for (i, window_x) in x_vals.as_slice().windows(2).enumerate() {
            let start_x = window_x[0];
            let next_x = window_x[1];
            let part1_x_width = next_x.min(part1_max_val + 1) - start_x.max(part1_min_val);
            let part2_x_width = next_x - start_x;

            for (j, window_y) in y_vals.as_slice().windows(2).enumerate() {
                let start_y = window_y[0];
                let next_y = window_y[1];
                let part1_y_width = next_y.min(part1_max_val + 1) - start_y.max(part1_min_val);
                let part2_y_width = next_y - start_y;

                for (k, window_z) in z_vals.as_slice().windows(2).enumerate() {
                    if cuboids[i][j][k] {
                        let start_z = window_z[0];
                        let next_z = window_z[1];

                        // Part 1: calculate the intersection of the block of cubes
                        // with the (-50..50, -50..=50, -50..=50) range.
                        // Add this to the number of cubes that are switched on.
                        if part1_x_width > 0 && part1_y_width > 0 {
                            let part1_z_width = next_z.min(part1_max_val + 1) - start_z.max(part1_min_val);
                            if part1_z_width > 0 {
                                part1_total_on += part1_x_width * part1_y_width * part1_z_width;
                            }
                        }

                        // part 2: add the volume of the current block to the total
                        let part2_z_width = next_z - start_z;
                        part2_total_on += part2_x_width * part2_y_width * part2_z_width;
                    }
                }
            }
        }

        if rep == 0 {
            println!("Part 1 answer: {}", part1_total_on);
            println!("Part 2 answer: {}", part2_total_on);
            println!(
                "Duration of initial run (including I/O): {:?}",
                start_time.elapsed()
            );
            println!();

            // Reset the timer so that subsequent iterations exclude I/O...
            start_time = Instant::now();
        }
    }

    let avg_duration = start_time.elapsed() / NUM_REPETITIONS;
    println!(
        "Avg duration over {} further repetitions: {:?}",
        NUM_REPETITIONS, avg_duration
    );
}
