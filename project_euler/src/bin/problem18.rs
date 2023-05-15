use std::cmp::max;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let inputs: [Vec<u64>; 15] = [
        vec![75],
        vec![95, 64],
        vec![17, 47, 82],
        vec![18, 35, 87, 10],
        vec![20,  4, 82, 47, 65],
        vec![19,  1, 23, 75,  3, 34],
        vec![88,  2, 77, 73,  7, 63, 67],
        vec![99, 65,  4, 28,  6, 16, 70, 92],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48],
        vec![63, 66,  4, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31],
        vec![ 4, 62, 98, 27, 23,  9, 70, 98, 73, 93, 38, 53, 60,  4, 23],
    ];

    let num_repetitions = 1000;
    for rep in 0..num_repetitions {
        let max_of_max_totals = get_max_sum_of_any_pathway(&inputs);
        if rep == 0 {
            println!("Max total = {}", max_of_max_totals);
        }
    }

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
    println!("Avg Duration: {:?}", duration / num_repetitions)
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
