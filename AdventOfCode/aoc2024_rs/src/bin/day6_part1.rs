use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day6_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 6 part 1", solve, 1000);
}

// Set up the 4 movement directions
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DIR_CHARS: &str = ">v<^";

#[derive(Copy, Clone)]
enum CellState {
    Unvisited,
    Visited,
    Obstructed,
    OutOfBounds,
}

fn solve(contents: &str) -> usize {
    let mut curr_pos: (i32, i32) = (0, 0);
    let mut curr_dir: usize = 0;
    let mut grid: Vec<Vec<CellState>> = contents
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, ch)| match ch {
                    b'.' => CellState::Unvisited,
                    b'#' => CellState::Obstructed,
                    _ => {
                        curr_dir = DIR_CHARS.find(ch as char).unwrap();
                        curr_pos = (col as i32, row as i32);
                        CellState::Visited
                    }
                })
                .collect::<Vec<CellState>>()
        })
        .collect();

    // Patrol the grid until out-of-bounds
    let mut visit_count: usize = 1;
    loop {
        let next_pos = get_next_pos(curr_pos, curr_dir);
        match get_state_at_pos(next_pos, &grid) {
            CellState::Unvisited => {
                grid[next_pos.1 as usize][next_pos.0 as usize] = CellState::Visited;
                visit_count += 1;
                curr_pos = next_pos;
            }
            CellState::Visited => {
                curr_pos = next_pos;
            }
            CellState::Obstructed => {
                turn_right(&mut curr_dir);
            }
            CellState::OutOfBounds => break,
        }
    }
    visit_count
}

#[inline(always)]
fn get_next_pos(curr_pos: (i32, i32), dir: usize) -> (i32, i32) {
    let offset = DIRS[dir];
    (curr_pos.0 + offset.0, curr_pos.1 + offset.1)
}

#[inline(always)]
fn get_state_at_pos(pos: (i32, i32), grid: &[Vec<CellState>]) -> CellState {
    let row_count = grid.len() as i32;
    let col_count = grid[0].len() as i32;

    if pos.0 < 0 || pos.0 >= col_count || pos.1 < 0 || pos.1 >= row_count {
        CellState::OutOfBounds
    } else {
        grid[pos.1 as usize][pos.0 as usize]
    }
}

#[inline(always)]
fn turn_right(dir: &mut usize) {
    *dir = (*dir + 1) % 4;
}
