use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};

type VisitedGrid = Vec<Vec<bool>>;
type Pos = (usize, usize);

fn main() {
    let input_file = File::open("data/day9_input").unwrap();
    let br = BufReader::new(input_file);
    let mut visited_grid: VisitedGrid = br
        .lines()
        .map(|line_result| {
            line_result
                .unwrap()
                .as_bytes()
                .iter()
                .map(|&byte| byte == b'9')
                .collect::<Vec<bool>>()
        })
        .collect();

    let mut basin_sizes = BinaryHeap::<usize>::new(); // NB: this is a max-heap

    while let Some(start_pos) = find_next_unvisited_cell(&mut visited_grid) {
        let basin_size = visit_all_cells_in_basin_and_get_count(start_pos, &mut visited_grid);
        basin_sizes.push(basin_size);
    }

    let basin_product: usize = basin_sizes.iter().take(3).product();
    println!(
        "The product of the sizes of the biggest 3 basins is {}",
        basin_product
    );
}

fn find_next_unvisited_cell(visited_grid: &mut VisitedGrid) -> Option<Pos> {
    visited_grid
        .iter()
        .enumerate()
        .filter_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &visited)| if visited { None } else { Some(j) })
                .next()
                .map(|j| (i, j))
        })
        .next()
}

fn visit_all_cells_in_basin_and_get_count(start_pos: Pos, visited_grid: &mut VisitedGrid) -> usize {
    let row_count = visited_grid.len();

    // Assume there is at least 1 row and that all rows are the same size:
    let col_count = visited_grid[0].len();

    let mut basin_size = 0;
    let mut basin_cells: Vec<Pos> = Vec::with_capacity(row_count * col_count);
    basin_cells.push(start_pos);

    while !basin_cells.is_empty() {
        let (i, j) = basin_cells.pop().unwrap();
        if visited_grid[i][j] {
            continue;
        }
        basin_size += 1;
        visited_grid[i][j] = true;

        // Add adjacent cells to basin_cells (if unvisited)
        if j > 0 {
            mark_to_visit_if_unvisited((i, j - 1), visited_grid, &mut basin_cells);
        }
        if j < col_count - 1 {
            mark_to_visit_if_unvisited((i, j + 1), visited_grid, &mut basin_cells);
        }
        if i > 0 {
            mark_to_visit_if_unvisited((i - 1, j), visited_grid, &mut basin_cells);
        }
        if i < row_count - 1 {
            mark_to_visit_if_unvisited((i + 1, j), visited_grid, &mut basin_cells);
        }
    }
    basin_size
}

fn mark_to_visit_if_unvisited(
    pos_to_visit: Pos,
    visited_grid: &VisitedGrid,
    basin_cells: &mut Vec<Pos>,
) {
    if !visited_grid[pos_to_visit.0][pos_to_visit.1] {
        basin_cells.push(pos_to_visit);
    }
}
