use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::cmp::PartialEq;
use std::ops::RangeInclusive;
use std::str::Lines;

const INPUT_FILE_PATH: &str = "data/day15_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 15 part 2", solve, 1000);
}

// Set up the 4 movement directions
const DIRS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const DIR_CHARS: [u8; 4] = [b'>', b'v', b'<', b'^'];

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CellState {
    Wall,
    Empty,
    Robot,
    BoxLeft,
    BoxRight,
}

fn solve(contents: &str) -> usize {
    let grid = generate_final_grid(contents);
    get_sum_of_box_coordinates(&grid)
}

fn generate_final_grid(contents: &str) -> Vec<Vec<CellState>> {
    let mut line_iter = contents.lines();
    let (mut grid, curr_pos) = parse_grid_and_get_robot_position(&mut line_iter);
    parse_and_process_instructions(curr_pos, &mut line_iter, &mut grid);
    grid
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
                .flat_map(|(col, byte)| match byte {
                    b'#' => [CellState::Wall, CellState::Wall],
                    b'.' => [CellState::Empty, CellState::Empty],
                    b'@' => {
                        curr_pos = (2 * col, row);
                        [CellState::Robot, CellState::Empty]
                    }
                    b'O' => [CellState::BoxLeft, CellState::BoxRight],
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
            CellState::BoxLeft | CellState::BoxRight if offset.1 == 0 => {
                try_to_push_box_horizontally(&mut curr_pos, first_pos, offset, grid);
            }
            CellState::BoxLeft | CellState::BoxRight => {
                try_to_push_box_vertically(&mut curr_pos, first_pos, offset.1, grid);
            }
        }
    }
}

fn try_to_push_box_horizontally(
    curr_pos: &mut (usize, usize),
    first_pos: (usize, usize),
    offset: (isize, isize),
    grid: &mut [Vec<CellState>],
) {
    for num_steps in 1.. {
        let last_pos = get_pos_in_line(first_pos, offset, num_steps);
        let last_state = get_mut_state_at_pos(last_pos, grid);
        match last_state {
            CellState::Wall => {
                break;
            }
            CellState::Empty => {
                // Shift all items along one position, from last to first
                for step_index in (0..=num_steps).rev() {
                    let dest_pos = get_pos_in_line(first_pos, offset, step_index);
                    let src_pos = get_pos_in_line(first_pos, offset, step_index - 1);
                    *get_mut_state_at_pos(dest_pos, grid) = get_state_at_pos(src_pos, grid);
                }
                *get_mut_state_at_pos(*curr_pos, grid) = CellState::Empty;
                *curr_pos = first_pos;
                break;
            }
            CellState::Robot => {
                panic!("Another robot was found unexpectedly at {last_pos:?}");
            }
            CellState::BoxLeft | CellState::BoxRight => {}
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct RowSelection {
    y: usize,
    x_ranges: Vec<RangeInclusive<usize>>,
}

impl RowSelection {
    fn new(y: usize, x_ranges: Vec<RangeInclusive<usize>>) -> Self {
        Self { y, x_ranges }
    }

    fn from_pos((x, y): (usize, usize)) -> Self {
        Self::new(y, vec![x..=x])
    }
}

#[derive(PartialEq, Eq)]
enum RowPushResult {
    Blocked,
    Pushed,
    AnotherRowToPush(RowSelection),
}

fn try_to_push_box_vertically(
    robot_pos: &mut (usize, usize),
    first_pos: (usize, usize),
    y_offset: isize,
    grid: &mut [Vec<CellState>],
) {
    let mut rows_to_push: Vec<RowSelection> = Vec::new();
    let mut push_result = RowPushResult::AnotherRowToPush(RowSelection::from_pos(*robot_pos));

    while let RowPushResult::AnotherRowToPush(new_row) = push_result {
        push_result = push_next_row_vertically(&new_row, y_offset, grid);
        rows_to_push.push(new_row);
    }

    if let RowPushResult::Pushed = push_result {
        while let Some(row_sel) = rows_to_push.pop() {
            let dest_y = (row_sel.y as isize + y_offset) as usize;
            let (src_row, dest_row) = if y_offset == 1 {
                let (before, after) = grid.split_at_mut(dest_y);
                (before.last_mut().unwrap(), after.first_mut().unwrap())
            } else {
                let (before, after) = grid.split_at_mut(row_sel.y);
                (after.first_mut().unwrap(), before.last_mut().unwrap())
            };

            for x_range in row_sel.x_ranges {
                for x in x_range {
                    dest_row[x] = src_row[x];
                    src_row[x] = CellState::Empty;
                }
            }
        }
        *robot_pos = first_pos;
    }
}

fn push_next_row_vertically(
    last_row_pushed: &RowSelection,
    y_offset: isize,
    grid: &mut [Vec<CellState>],
) -> RowPushResult {
    let y = (last_row_pushed.y as isize + y_offset) as usize;
    let grid_row = &grid[y];
    let mut curr_range: Option<RangeInclusive<usize>> = None;

    let mut new_row_selection: Vec<RangeInclusive<usize>> = vec![];
    for range in last_row_pushed.x_ranges.iter() {
        for x in range.clone() {
            match grid_row[x] {
                CellState::Wall => {
                    return RowPushResult::Blocked;
                }
                CellState::Empty => {
                    if let Some(last_range_selection) = curr_range {
                        new_row_selection.push(last_range_selection);
                        curr_range = None;
                    }
                }
                CellState::Robot => {
                    panic!("Another robot was found unexpectedly at ({x}, {y})");
                }
                CellState::BoxLeft => {
                    update_range_selections_with_next_range(
                        &mut curr_range,
                        &mut new_row_selection,
                        x,
                        x + 1,
                    );
                }
                CellState::BoxRight => {
                    update_range_selections_with_next_range(
                        &mut curr_range,
                        &mut new_row_selection,
                        x - 1,
                        x,
                    );
                }
            }
        }
    }

    // Don't forget to include the last range
    if let Some(curr_range) = curr_range {
        new_row_selection.push(curr_range);
    }

    if new_row_selection.is_empty() {
        RowPushResult::Pushed
    } else {
        RowPushResult::AnotherRowToPush(RowSelection::new(y, new_row_selection))
    }
}

fn update_range_selections_with_next_range(
    curr_range: &mut Option<RangeInclusive<usize>>,
    new_row_selection: &mut Vec<RangeInclusive<usize>>,
    box_start: usize,
    box_end: usize,
) {
    if let Some(curr_range_selection) = curr_range {
        let curr_range_end = *curr_range_selection.end();
        if curr_range_end + 1 < box_start {
            new_row_selection.push(curr_range_selection.clone());
            *curr_range = Some(box_start..=box_end);
        } else if curr_range_end < box_end {
            *curr_range = Some(*curr_range_selection.start()..=box_end);
        }
    } else {
        *curr_range = Some(box_start..=box_end);
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

#[inline(always)]
fn get_state_at_pos(pos: (usize, usize), grid: &mut [Vec<CellState>]) -> CellState {
    let row_count = grid.len();
    let col_count = grid[0].len();

    if pos.0 >= col_count || pos.1 >= row_count {
        panic!("Position out of bounds: {pos:?}");
    }
    grid[pos.1][pos.0]
}

fn get_sum_of_box_coordinates(grid: &[Vec<CellState>]) -> usize {
    let sum_of_box_coordinates: usize = grid
        .iter()
        .enumerate()
        .map(|(c, row)| {
            let row_sum: usize = row
                .iter()
                .enumerate()
                .map(|(r, &cell_state)| {
                    if let CellState::BoxLeft = cell_state {
                        r + 100 * c
                    } else {
                        0
                    }
                })
                .sum();
            row_sum
        })
        .sum();
    sum_of_box_coordinates
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_SCENARIO: &str = "##########
#........#
#O..#..OO#
#.OO#.OO@#
#.OO.OO..#
#..OOO...#
#...OO...#
#...O....#
#........#
##########

";

    #[test]
    fn test_pushing_boxes_left() {
        let mut contents = String::from(BASIC_SCENARIO);
        contents.push_str("<");
        let grid = generate_final_grid(&contents);
        let expected_cells: [CellState; 7] = [
            CellState::Empty,
            CellState::BoxLeft,
            CellState::BoxRight,
            CellState::BoxLeft,
            CellState::BoxRight,
            CellState::Robot,
            CellState::Empty,
        ];
        assert_eq!(&grid[3][10..17], expected_cells);
    }

    #[test]
    fn test_pushing_boxes_up() {
        let mut contents = String::from(BASIC_SCENARIO);
        contents.push_str("<vvv<<<<vv<<^");
        let grid = generate_final_grid(&contents);
        let robot_count = grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&cell_state| cell_state == &CellState::Robot)
            .count();
        assert_eq!(robot_count, 1);
    }

    #[test]
    fn test_pushing_boxes_down() {
        let mut contents = String::from(BASIC_SCENARIO);
        contents.push_str("^<^<v");
        let grid = generate_final_grid(&contents);
        let robot_count = grid
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&cell_state| cell_state == &CellState::Robot)
            .count();
        assert_eq!(robot_count, 1);
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

        assert_eq!(solve(contents), 9021);
    }
}
