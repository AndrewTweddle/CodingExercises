use std::time::Instant;

fn main() {
    let start = Instant::now();

    let row_count = 20;
    let col_count = 20;

    // Calculate the number of paths to each point on a diagonal from bottom left to top right.
    // The diagonals sweep down from the start to end position.
    let mut even_diagonal: Vec<u64> = vec![1];
    let mut odd_diagonal = Vec::<u64>::new();

    let max_diagonal_size = col_count.min(row_count);
    even_diagonal.reserve(max_diagonal_size);
    odd_diagonal.reserve(max_diagonal_size);

    // Track a pair of diagonals - the previous one and the current one (which is being calculated)
    let prev_diag = &mut odd_diagonal;
    let curr_diag = &mut even_diagonal;

    let mut prev_start_col = 0;

    // Note: The (row, col) points on each new diagonal share the same sum: row + col
    // So iterate over values of this sum to calculate the diagonals...
    for row_and_col_sum in 1..=40 {
        std::mem::swap(prev_diag, curr_diag);
        curr_diag.clear();

        let start_row = row_and_col_sum.min(row_count);
        let start_col = row_and_col_sum - start_row;
        let end_col = row_and_col_sum.min(col_count);
        let col_offset_to_prev = start_col - prev_start_col;

        for col in start_col..=end_col {
            let row = row_and_col_sum - col;
            let paths_from_the_left = if col == 0 {
                0
            } else {
                prev_diag[col - start_col + col_offset_to_prev - 1]
            };
            let paths_from_above = if row == 0 {
                0
            } else {
                prev_diag[col - start_col + col_offset_to_prev]
            };
            curr_diag.push(paths_from_the_left + paths_from_above);
        }

        prev_start_col = start_col;
    }

    let final_path_count = curr_diag[0];
    println!("Path count: {}", final_path_count);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}
