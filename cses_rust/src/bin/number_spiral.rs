use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let stdin_lock = stdin.lock();
    let mut line_iter = stdin_lock.lines();
    let t = line_iter.next().unwrap().unwrap().as_str().parse::<usize>().unwrap();
    for _ in 0..t {
        let next_line = line_iter.next().unwrap().unwrap();
        let (y_str, x_str) = next_line.split_once(' ').unwrap();
        let col = x_str.parse::<isize>().unwrap();
        let row = y_str.parse::<isize>().unwrap();
        let diag_row_col = col.max(row);
        let diag_val = 1 + (diag_row_col - 1) * diag_row_col;
        // When on an even numbered diagonal, values increase down the column
        // and to the left along the row.
        // Otherwise vice versa. Store this direction as a sign (1 or -1).
        let dir_sign = if diag_row_col % 2 == 0 { 1 } else { -1 };

        // calculate the offset from the diagonal value
        let offset_from_diagonal = dir_sign * if row <= col {
            // values decrease going up the column for even diagonals, else vice versa
            row - diag_row_col
        } else {
            // values increase going left along the row for even diagonals, else vice versa
            diag_row_col - col
        };
        let cell_val = diag_val + offset_from_diagonal;

        println!("{}", cell_val);
    }
}
