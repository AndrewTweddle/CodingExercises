use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;
const USE_BINARY_SEARCH: bool = true; // This works better. It takes about 2.8 ms instead of 9 ms.

fn main() {
    let input_file = File::open("data/day7_input").unwrap();
    let br = BufReader::new(input_file);
    let first_line = br.lines().next().unwrap().unwrap();

    let mut answer: (i64, i64) = (0, 0);

    let start = Instant::now();
    for _ in 0..NUM_REPETITIONS {
        let mut positions: Vec<i64> = first_line
            .split(',')
            .map(|num_str| num_str.parse::<i64>().unwrap())
            .collect();
        positions.sort();

        answer = if USE_BINARY_SEARCH {
            calculate_position_and_minimum_fuel_required_using_binary_search(
                &positions,
                *positions.first().unwrap(),
                *positions.last().unwrap(),
            )
        } else {
            // Start from the median again (hopefully it's a decent guess)
            let median = positions[positions.len() / 2];
            calculate_position_and_minimum_fuel_required_using_deltas(&positions, median)
        };
    }
    let total_duration = start.elapsed();

    println!(
        "Minimum fuel required at position {}: {}",
        answer.0, answer.1
    );
    println!(
        "Average duration of {} calculations: {:?}",
        NUM_REPETITIONS,
        total_duration / NUM_REPETITIONS
    );
}

fn calculate_position_and_minimum_fuel_required_using_binary_search(
    positions: &Vec<i64>,
    min_pos: i64,
    max_pos: i64,
) -> (i64, i64) {
    let mid_pos = (min_pos + max_pos) / 2;

    let fuel: i64 = get_fuel_for_position(&positions, mid_pos);
    let left_fuel = get_fuel_for_position(&positions, mid_pos - 1);

    if left_fuel < fuel {
        if min_pos == mid_pos - 1 {
            (min_pos, left_fuel)
        } else {
            calculate_position_and_minimum_fuel_required_using_binary_search(
                positions,
                min_pos,
                mid_pos - 1,
            )
        }
    } else {
        let right_fuel = get_fuel_for_position(&positions, mid_pos + 1);
        if right_fuel >= fuel {
            (mid_pos, fuel)
        } else if max_pos == mid_pos + 1 {
            (max_pos, right_fuel)
        } else {
            calculate_position_and_minimum_fuel_required_using_binary_search(
                positions,
                mid_pos + 1,
                max_pos,
            )
        }
    }
}

fn get_fuel_for_position(positions: &Vec<i64>, target: i64) -> i64 {
    // Calculate the required fuel for the target position, using the triangular number formula
    positions
        .iter()
        .map(|&pos| {
            let distance = (pos - target).abs();
            distance * (distance + 1) / 2
        })
        .sum()
}

// ------------------------------------------------------------------------------------------
// The first method I tried. This can't do a binary search.
// It takes longer, at around 9 ms for the calculations...

fn calculate_position_and_minimum_fuel_required_using_deltas(
    positions: &Vec<i64>,
    start_pos: i64,
) -> (i64, i64) {
    let mut target = start_pos;

    let mut fuel: i64 = get_fuel_for_position(&positions, target);

    // First test if the fuel requirement decreases if the target is moved left.
    let mut fuel_delta: i64 = get_fuel_delta_if_target_decremented(&positions, target);

    if fuel_delta < 0 {
        // Decrease the target position incrementally until there is no more reduction in fuel
        while fuel_delta < 0 {
            target -= 1;
            fuel += fuel_delta;
            fuel_delta = get_fuel_delta_if_target_decremented(&positions, target);
        }
    } else {
        // In the target position incrementally until there is no more reduction in fuel
        fuel_delta = get_fuel_delta_if_target_incremented(&positions, target);

        while fuel_delta < 0 {
            target += 1;
            fuel += fuel_delta;
            fuel_delta = get_fuel_delta_if_target_incremented(&positions, target);
        }
    }

    (target, fuel)
}

fn get_fuel_delta_if_target_decremented(positions: &Vec<i64>, target: i64) -> i64 {
    positions
        .iter()
        .map(|&pos| {
            let offset = pos - target;
            if offset < 0 {
                offset
            } else {
                offset + 1
            }
        })
        .sum()
}

fn get_fuel_delta_if_target_incremented(positions: &Vec<i64>, target: i64) -> i64 {
    positions
        .iter()
        .map(|&pos| {
            let offset = pos - target;
            if offset <= 0 {
                1 - offset
            } else {
                -offset
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    mod tests_using_binary_search {
        use super::super::calculate_position_and_minimum_fuel_required_using_binary_search;

        #[test]
        fn test_example_using_triangular_numbers() {
            let mut positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            positions.sort();
            let (pos, fuel) =
                calculate_position_and_minimum_fuel_required_using_binary_search(&positions, 0, 16);
            assert_eq!(pos, 5);
            assert_eq!(fuel, 168);
        }
    }

    mod tests_using_fuel_deltas {
        use super::super::calculate_position_and_minimum_fuel_required_using_deltas;

        #[test]
        fn test_example_from_below() {
            let mut positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            positions.sort();
            let (pos, fuel) =
                calculate_position_and_minimum_fuel_required_using_deltas(&positions, 0);
            assert_eq!(pos, 5);
            assert_eq!(fuel, 168);
        }

        #[test]
        fn test_example_from_above() {
            let mut positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            positions.sort();
            let (pos, fuel) =
                calculate_position_and_minimum_fuel_required_using_deltas(&positions, 12);
            assert_eq!(pos, 5);
            assert_eq!(fuel, 168);
        }

        #[test]
        fn test_example_from_target() {
            let mut positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            positions.sort();
            let (pos, fuel) =
                calculate_position_and_minimum_fuel_required_using_deltas(&positions, 5);
            assert_eq!(pos, 5);
            assert_eq!(fuel, 168);
        }

        #[test]
        fn test_example_from_below_minimum() {
            let mut positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            positions.sort();
            let (pos, fuel) =
                calculate_position_and_minimum_fuel_required_using_deltas(&positions, -1);
            assert_eq!(pos, 5);
            assert_eq!(fuel, 168);
        }

        #[test]
        fn test_example_from_above_maximum() {
            let mut positions = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
            positions.sort();
            let (pos, fuel) =
                calculate_position_and_minimum_fuel_required_using_deltas(&positions, 100);
            assert_eq!(pos, 5);
            assert_eq!(fuel, 168);
        }
    }
}
