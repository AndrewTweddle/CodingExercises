use std::cmp::max;
use std::fs;
use std::time::Instant;

fn main() {
    let mut start = Instant::now();
    let data_file_path = "data/0067_triangle.txt";
    let text = fs::read_to_string(data_file_path).unwrap();
    let inputs: Vec<Vec<u64>> = text
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();

    let num_repetitions = 1000;
    for rep in 0..=num_repetitions {
        let max_of_max_totals = get_max_sum_of_any_pathway(&inputs);
        if rep == 0 {
            println!("Max total = {max_of_max_totals}");
            println!("Duration (incl I/O): {:?}", start.elapsed());

            // Restart the timer, so further timings exclude I/O
            start = Instant::now();
        }
    }

    let duration = start.elapsed();
    println!("Avg Duration: {:?}", duration / num_repetitions);
    println!("Total duration over {num_repetitions} further repetitions: {duration:?}");
}

fn get_max_sum_of_any_pathway(inputs: &[Vec<u64>]) -> u64 {
    let row_count = inputs.len();
    let mut path_sums: Vec<Vec<u64>> = Vec::with_capacity(row_count);
    path_sums.push(vec![inputs[0][0]]);
    for row in 1..row_count {
        let mut row_of_sums = Vec::<u64>::with_capacity(row + 1);

        row_of_sums.push(path_sums[row - 1][0] + inputs[row][0]);
        for col in 1..row {
            let max_path =
                max(path_sums[row - 1][col - 1], path_sums[row - 1][col]) + inputs[row][col];
            row_of_sums.push(max_path);
        }
        row_of_sums.push(path_sums[row - 1][row - 1] + inputs[row][row]);
        path_sums.push(row_of_sums);
    }

    (0..row_count)
        .map(|col| path_sums[row_count - 1][col])
        .max()
        .unwrap()
}
