use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::str::Lines;

const INPUT_FILE_PATH: &str = "data/day15_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 15 part 1", solve, 1000);
}

// Set up the 4 movement directions
const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DIR_CHARS: [u8; 4] = [b'>', b'v', b'<', b'^'];

#[derive(Copy, Clone)]
enum CellState {
    Wall,
    Empty,
    Robot,
    Box,
}

fn solve(contents: &str) -> usize {
    let mut line_iter = contents.lines();
    let (mut grid, curr_pos) = parse_grid_and_get_robot_position(&mut line_iter);
    parse_and_process_instructions(curr_pos, &mut line_iter, &mut grid);
    get_sum_of_box_coordinates(&grid)
}

fn parse_grid_and_get_robot_position(
    line_iter: &mut Lines,
) -> (Vec<Vec<CellState>>, (usize, usize)) {
    let mut curr_pos: (usize, usize) = (0, 0);
    let grid: Vec<Vec<CellState>> = line_iter
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, byte)| match byte {
                    b'#' => CellState::Wall,
                    b'.' => CellState::Empty,
                    b'@' => {
                        curr_pos = (col, row);
                        CellState::Robot
                    }
                    b'O' => CellState::Box,
                    _ => panic!("Unrecognized map character {}", char::from(byte)),
                })
                .collect::<Vec<CellState>>()
        })
        .collect();
    (grid, curr_pos)
}

fn parse_and_process_instructions(
    mut curr_pos: (usize, usize),
    line_iter: &mut Lines,
    grid: &mut [Vec<CellState>],
) {
    for instruction in line_iter.flat_map(|line| line.bytes()) {
        let dir_index = DIR_CHARS.iter().position(|&r| r == instruction).unwrap();
        let offset = DIRS[dir_index];
        let first_pos = get_next_pos(curr_pos, offset);
        let first_state = get_mut_state_at_pos(first_pos, grid);
        match first_state {
            CellState::Wall => {}
            CellState::Empty => {
                *first_state = CellState::Robot;
                *get_mut_state_at_pos(curr_pos, grid) = CellState::Empty;
                curr_pos = first_pos;
            }
            CellState::Robot => {
                panic!("Another robot was found unexpectedly at {first_pos:?}");
            }
            CellState::Box => {
                for num_steps in 1.. {
                    let last_pos = get_pos_in_line(first_pos, offset, num_steps);
                    let last_state = get_mut_state_at_pos(last_pos, grid);
                    match last_state {
                        CellState::Wall => {
                            break;
                        }
                        CellState::Empty => {
                            *last_state = CellState::Box;
                            *get_mut_state_at_pos(first_pos, grid) = CellState::Robot;
                            *get_mut_state_at_pos(curr_pos, grid) = CellState::Empty;
                            curr_pos = first_pos;
                            break;
                        }
                        CellState::Robot => {
                            panic!("Another robot was found unexpectedly at {last_pos:?}");
                        }
                        CellState::Box => {}
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn get_next_pos(start_pos: (usize, usize), offset: (isize, isize)) -> (usize, usize) {
    get_pos_in_line(start_pos, offset, 1)
}

#[inline(always)]
fn get_pos_in_line(
    start_pos: (usize, usize),
    offset: (isize, isize),
    step_count: isize,
) -> (usize, usize) {
    (
        (start_pos.0 as isize + step_count * offset.0) as usize,
        (start_pos.1 as isize + step_count * offset.1) as usize,
    )
}

#[inline(always)]
fn get_mut_state_at_pos(pos: (usize, usize), grid: &mut [Vec<CellState>]) -> &mut CellState {
    let row_count = grid.len();
    let col_count = grid[0].len();

    if pos.0 >= col_count || pos.1 >= row_count {
        panic!("Position out of bounds: {pos:?}");
    }
    &mut grid[pos.1][pos.0]
}

fn get_sum_of_box_coordinates(grid: &[Vec<CellState>]) -> usize {
    let sum_of_box_coordinates: usize = grid
        .iter()
        .enumerate()
        .map(|(c, row)| {
            let row_sum: usize = row.iter().enumerate().map(|(r, &cell_state)|
                if let CellState::Box = cell_state {
                    r + 100 * c
                } else {
                    0
                }
            ).sum();
            row_sum
        })
        .sum();
    sum_of_box_coordinates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smaller_example() {
        let contents = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

        assert_eq!(solve(contents), 2028);
    }
    
    #[test]
    fn test_larger_example() {
        let contents = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

        assert_eq!(solve(contents), 10092);
    }
}
