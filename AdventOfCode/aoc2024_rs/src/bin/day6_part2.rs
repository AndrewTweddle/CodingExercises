use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day6_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 6 part 2", solve, 10);
}

// Set up the 4 movement directions
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DIR_CHARS: &str = ">v<^";

type VisitedFromDir = [bool; 4];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum CellState {
    Unvisited,
    Visited(VisitedFromDir),
    Obstructed,
    OutOfBounds,
    Cycled,
}

fn solve(contents: &str) -> usize {
    let mut curr_pos: (i32, i32) = (0, 0);
    let mut curr_dir: usize = 0;
    let starting_grid: Vec<Vec<CellState>> = contents
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
                        CellState::Visited([false; 4])
                    }
                })
                .collect::<Vec<CellState>>()
        })
        .collect();

    let mut grid = starting_grid.clone();
    assert_eq!(
        patrol(&mut grid, curr_pos, curr_dir),
        CellState::OutOfBounds
    );

    // Since the grid has been modified, we can use it to find all cells that were visited.
    // These are the only candidates for a new obstacle (excluding the starting position).
    let potential_new_obstacles_iter = grid.iter().enumerate().flat_map(|(y, row)| {
        row.iter()
            .enumerate()
            .filter(move |&(x, cell)| {
                (x as i32, y as i32) != curr_pos && matches!(cell, CellState::Visited(_))
            })
            .map(move |(x, _)| (x, y))
    });
    potential_new_obstacles_iter
        .filter(|&(x, y)| {
            let mut grid_with_new_obstacle = starting_grid.clone();
            grid_with_new_obstacle[y][x] = CellState::Obstructed;
            patrol(&mut grid_with_new_obstacle, curr_pos, curr_dir) == CellState::Cycled
        })
        .count()
}

fn patrol(grid: &mut [Vec<CellState>], mut curr_pos: (i32, i32), mut curr_dir: usize) -> CellState {
    // Patrol the grid until out-of-bounds or a cycle is encountered
    loop {
        let next_pos = get_next_pos(curr_pos, curr_dir);
        let next_cell_state = get_state_at_pos(next_pos, grid);
        match next_cell_state {
            CellState::Unvisited => {
                let mut src_dirs = [false; 4];
                src_dirs[curr_dir] = true;
                grid[next_pos.1 as usize][next_pos.0 as usize] = CellState::Visited(src_dirs);
                curr_pos = next_pos;
            }
            CellState::Visited(mut src_dirs) => {
                if src_dirs[curr_dir] {
                    return CellState::Cycled;
                }
                src_dirs[curr_dir] = true;
                grid[next_pos.1 as usize][next_pos.0 as usize] = CellState::Visited(src_dirs);
                curr_pos = next_pos;
            }
            CellState::Obstructed => {
                turn_right(&mut curr_dir);
            }
            _ => return next_cell_state,
        }
    }
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
