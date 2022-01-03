use std::iter;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let inputs: [Vec<u64>; 15] = [
        vec![75,],
        vec![95, 64,],
        vec![17, 47, 82,],
        vec![18, 35, 87, 10,],
        vec![20, 04, 82, 47, 65,],
        vec![19, 01, 23, 75, 03, 34,],
        vec![88, 02, 77, 73, 07, 63, 67,],
        vec![99, 65, 04, 28, 06, 16, 70, 92,],
        vec![41, 41, 26, 56, 83, 40, 80, 70, 33,],
        vec![41, 48, 72, 33, 47, 32, 37, 16, 94, 29,],
        vec![53, 71, 44, 65, 25, 43, 91, 52, 97, 51, 14,],
        vec![70, 11, 33, 28, 77, 73, 17, 78, 39, 68, 17, 57,],
        vec![91, 71, 52, 38, 17, 14, 91, 43, 58, 50, 27, 29, 48,],
        vec![63, 66, 04, 68, 89, 53, 67, 30, 73, 16, 69, 87, 40, 31,],
        vec![04, 62, 98, 27, 23, 09, 70, 98, 73, 93, 38, 53, 60, 04, 23,],
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
    let mut paths = Vec::<u64>::with_capacity(row_count);
    // each row of path sums should be bracketed by zeroes,
    // so that the first and last item in each row partners with a zero when paired
    paths.push(0);
    paths.push(0);
    for row in 0..row_count {
        paths = iter::once(0_u64)
            .chain( paths
                .windows(2)
                .map(|pair| pair.iter().max().unwrap())
                .zip(&inputs[row])
                .map(|(max, input)| max + input)
            )
            .chain(iter::once(0_u64))
            .collect();
    }

    let max_of_max_totals = paths.iter().max();
    *max_of_max_totals.unwrap()
}
