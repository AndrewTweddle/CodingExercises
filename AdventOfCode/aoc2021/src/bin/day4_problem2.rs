use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

const BOARD_SIZE: usize = 5;

type Board = Vec<Vec<u32>>;

fn main() {
    let input_file = File::open("data/day4_input").unwrap();
    let br = BufReader::new(input_file);
    let mut line_iter = br.lines();
    let calls = line_iter
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|call_str| call_str.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut boards: Vec<Board> = Vec::default();
    while let Some(board) = read_board(&mut line_iter) {
        boards.push(board);
    }

    let (winning_call, sum_of_unfilled_cells) =
        get_last_winning_call_and_sum_of_unfilled_cells(&boards, &calls).unwrap();

    println!("last winning call: {}", winning_call);
    println!("product: {}", winning_call * sum_of_unfilled_cells);
}

fn read_board(line_iter: &mut Lines<BufReader<File>>) -> Option<Board> {
    // Skip blank line...
    line_iter.next();

    let board: Board = line_iter
        .take(BOARD_SIZE)
        .map(|ln| {
            ln.unwrap()
                .split_whitespace()
                .map(|num_str| num_str.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    if board.len() == 5 {
        Some(board)
    } else {
        None
    }
}

fn get_last_winning_call_and_sum_of_unfilled_cells(
    boards: &Vec<Board>,
    calls: &Vec<u32>,
) -> Option<(u32, u32)> {
    let mut prev_calls = HashSet::<u32>::new();
    for &call in calls {
        prev_calls.insert(call);
        if let Some(unfilled_cells_sum) =
            get_sum_of_unfilled_cells_in_last_winning_board(&boards, &prev_calls, call)
        {
            return Some((call, unfilled_cells_sum));
        }
    }
    None
}

fn get_sum_of_unfilled_cells_in_last_winning_board(
    boards: &Vec<Board>,
    prev_calls: &HashSet<u32>,
    last_call: u32,
) -> Option<u32> {
    // First check that all boards are winning
    if !boards.iter().all(|board| has_full_line(board, &prev_calls)) {
        return None;
    }

    // Find the first board that contains the last call in a winning row or column
    // and that wasn't already a winner before the last call
    let completed_board = boards.iter().find(|board| {
        has_full_line_with_last_call(board, &prev_calls, last_call)
            && !has_full_column_without_last_call(board, &prev_calls, last_call)
            && !has_full_row_without_last_call(board, &prev_calls, last_call)
    });

    if let Some(board) = completed_board {
        let unfilled_cells_sum: u32 = board
            .iter()
            .flatten()
            .filter_map(|&cell| {
                if prev_calls.contains(&cell) {
                    None
                } else {
                    Some(cell)
                }
            })
            .sum();
        return Some(unfilled_cells_sum);
    }
    None
}

fn has_full_line(board: &Board, calls: &HashSet<u32>) -> bool {
    has_full_row(board, calls) || has_full_column(board, calls)
}

fn has_full_row(board: &Board, calls: &HashSet<u32>) -> bool {
    board
        .iter()
        .any(|row| row.iter().all(|cell| calls.contains(cell)))
}

fn has_full_column(board: &Board, calls: &HashSet<u32>) -> bool {
    (0..BOARD_SIZE).any(|col_index| board.iter().all(|row| calls.contains(&row[col_index])))
}

fn has_full_line_with_last_call(board: &Board, prev_calls: &HashSet<u32>, last_call: u32) -> bool {
    has_full_row_with_last_call(board, &prev_calls, last_call)
        || has_full_column_with_last_call(board, &prev_calls, last_call)
}

fn has_full_row_with_last_call(board: &Board, calls: &HashSet<u32>, last_call: u32) -> bool {
    board.iter().any(|row| {
        row.iter().all(|cell| calls.contains(cell)) && row.iter().any(|&cell| cell == last_call)
    })
}

fn has_full_column_with_last_call(board: &Board, calls: &HashSet<u32>, last_call: u32) -> bool {
    (0..BOARD_SIZE).any(|col_index| {
        board.iter().all(|row| calls.contains(&row[col_index]))
            && board.iter().any(|row| row[col_index] == last_call)
    })
}

fn has_full_row_without_last_call(board: &Board, calls: &HashSet<u32>, last_call: u32) -> bool {
    board.iter().any(|row| {
        row.iter()
            .all(|&cell| cell != last_call && calls.contains(&cell))
    })
}

fn has_full_column_without_last_call(board: &Board, calls: &HashSet<u32>, last_call: u32) -> bool {
    (0..BOARD_SIZE).any(|col_index| {
        board.iter().all(|row| {
            let cell = row[col_index];
            cell != last_call && calls.contains(&cell)
        })
    })
}
