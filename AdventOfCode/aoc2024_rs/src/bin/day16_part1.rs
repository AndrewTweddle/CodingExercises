use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::Lines;

const INPUT_FILE_PATH: &str = "data/day16_input.txt";
const COST_TO_TURN: usize = 1000;

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 16 part 1", solve, 1000);
}

// Set up data for turning actions:
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum TurnDir {
    Left,
    Right,
}

// Set up the 4 movement directions
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Right,
            1 => Self::Down,
            2 => Self::Left,
            3 => Self::Up,
            _ => panic!("Out of range value for direction: {}", value),
        }
    }
}

impl Direction {
    #[inline(always)]
    fn leftwards_dir(&self) -> Direction {
        Direction::from(((*self as u8) + 3) % 4)
    }

    #[inline(always)]
    fn rightwards_dir(&self) -> Direction {
        Direction::from(((*self as u8) + 1) % 4)
    }

    fn turn_to_the(&mut self, turn: TurnDir) {
        let new_dir = match turn {
            TurnDir::Left => self.leftwards_dir(),
            TurnDir::Right => self.rightwards_dir(),
        };
        *self = new_dir;
    }

    fn cost_to_turn(&self, dest_dir: Direction) -> usize {
        let start_ix = *self as i8;
        let end_ix = dest_dir as i8;
        match (end_ix - start_ix).abs() {
            0 => 0,
            1 => COST_TO_TURN,
            2 => 2 * COST_TO_TURN,
            3 => COST_TO_TURN,
            _ => panic!("Invalid direction {:?} or {:?}!", *self, dest_dir),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
enum CellState {
    Wall,
    Empty,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn move_in_dir(&mut self, dir: Direction) {
        match dir {
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Up => self.y -= 1,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Position,
    dir: Direction,
}

impl State {
    fn advance(&mut self) {
        self.pos.move_in_dir(self.dir);
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct SearchNode {
    /// The current state
    state: State,

    ///  Let: f = g + h be the estimated total cost to reach the end state
    /// with: g being the cost incurred to reach the current state
    ///  and: h being a lower bound estimate of the cost to reach the end state
    ///       from the current state (and since it is a lower bound, it is admissible).
    f: usize,

    /// g is the cost incurred thus far to reach this state
    g: usize,
}

impl SearchNode {
    fn new(state: State, g: usize, end_pos: Position) -> Self {
        let f = Self::calculate_f(state, g, end_pos);
        Self { state, f, g }
    }

    fn advance(&mut self, end_pos: Position) {
        self.state.advance();
        self.g += 1;
        self.recalculate_f(end_pos);
    }

    fn turn(&mut self, turn_dir: TurnDir, end_pos: Position) {
        self.state.dir.turn_to_the(turn_dir);
        self.g += COST_TO_TURN;
        self.recalculate_f(end_pos);
    }

    fn recalculate_f(&mut self, end_pos: Position) {
        self.f = Self::calculate_f(self.state, self.g, end_pos);
    }

    fn calculate_f(mut state: State, g: usize, end_pos: Position) -> usize {
        if state.pos == end_pos {
            return g;
        }

        // The next move will be a turn to the left or a turn to the right,
        // followed by advancing at least 1 space (but assume 1 space for calculation purposes).
        // Calculate f after each of these two moves and choose the smaller value of f.
        let mut left_turn_state = state;
        left_turn_state.dir.turn_to_the(TurnDir::Left);
        left_turn_state.advance();
        let left_turn_f = Self::calculate_f_after_next_move(state, g, end_pos);

        state.dir.turn_to_the(TurnDir::Right);
        state.advance();
        let right_turn_f = Self::calculate_f_after_next_move(state, g, end_pos);

        1 + COST_TO_TURN + left_turn_f.min(right_turn_f)
    }

    fn calculate_f_after_next_move(state: State, g: usize, end_pos: Position) -> usize {
        let x_steps = end_pos.x.abs_diff(state.pos.x);
        let y_steps = state.pos.y.abs_diff(end_pos.y);

        // This could give the wrong answer when over the top or right border,
        // but that state is invalid anyway, so will soon be rejected...
        let cost_of_turns = match (x_steps, y_steps) {
            (0, 0) => 0,
            (0, _) => state.dir.cost_to_turn(Direction::Up),
            (_, 0) => state.dir.cost_to_turn(Direction::Right),
            _ => {
                // Either move right, then turn, then move up, or vice versa.
                // Either way 1 turn will be required in the middle.
                if state.dir == Direction::Down || state.dir == Direction::Left {
                    // First turn right (if facing down) or up (if facing left)
                    2 * COST_TO_TURN
                } else {
                    COST_TO_TURN
                }
            }
        };

        // f = g + h = distance already moved + cost to move to target + cost of # of turns needed
        g + x_steps + y_steps + cost_of_turns
    }
}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // We will put the SearchNode in a max-heap, so smaller values of f (better solutions)
        // should be treated as "greater" than search nodes with larger values of f...
        match self.f.cmp(&other.f) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            // When two search nodes are equal on f, choose the one with the higher value of g.
            // This will bias the search towards searching deeply, to find the first solution,
            // rather than broadly, which might wastefully advance all equally good alternate
            // solutions one by one.
            Ordering::Equal => self.g.cmp(&other.g),
        }
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn solve(contents: &str) -> Option<usize> {
    let mut line_iter = contents.lines();
    let (grid, start_pos, end_pos) = parse_grid_and_get_start_and_end_positions(&mut line_iter);
    get_min_cost_path_using_astar_algorithm(&grid, start_pos, end_pos)
}

fn parse_grid_and_get_start_and_end_positions(
    line_iter: &mut Lines,
) -> (Vec<Vec<CellState>>, Position, Position) {
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let grid: Vec<Vec<CellState>> = line_iter
        .take_while(|line| !line.is_empty())
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, byte)| match byte {
                    b'#' => CellState::Wall,
                    b'.' => CellState::Empty,
                    b'S' => {
                        start_pos = (col, row);
                        CellState::Empty
                    }
                    b'E' => {
                        end_pos = (col, row);
                        CellState::Empty
                    }
                    _ => panic!("Unrecognized map character {}", char::from(byte)),
                })
                .collect::<Vec<CellState>>()
        })
        .collect();
    (
        grid,
        Position::new(start_pos.0, start_pos.1),
        Position::new(end_pos.0, end_pos.1),
    )
}

type DirCost = [Option<usize>; 4];
type ColumnSearchCost = Vec<DirCost>;
type MinSearchCostGrid = Vec<ColumnSearchCost>;

fn get_min_cost_path_using_astar_algorithm(
    grid: &[Vec<CellState>],
    start_pos: Position,
    end_pos: Position,
) -> Option<usize> {
    // Track the shortest path to reach each state (being a combination of position and direction)
    let mut min_cost_to_reach_state: MinSearchCostGrid =
        vec![vec![[None; 4]; grid[0].len()]; grid.len()];

    let mut frontier = BinaryHeap::<SearchNode>::new();

    // Each move will consist of turning either right or left by 90 degrees,
    // then advancing 1 or more steps to a reachable empty space.
    // However, on the very first turn, don't turn first, just step 0 or more spaces forwards.
    // This will seed the frontier of initial states to traverse.
    let start_state = State {
        pos: start_pos,
        dir: Direction::Right,
    };

    // Populate initial states (this will at least add the current search node
    repeatedly_add_search_node_to_frontier_and_advance_until_reaching_wall(
        SearchNode::new(start_state, 0, end_pos),
        &mut frontier,
        &mut min_cost_to_reach_state,
        grid,
        end_pos,
    );

    while let Some(mut curr_node) = frontier.pop() {
        if curr_node.state.pos == end_pos {
            return Some(curr_node.f);
        }
        // We consider 2 options:
        // a) turning left and stepping forward
        let mut left_node = curr_node.clone();
        left_node.turn(TurnDir::Left, end_pos);
        left_node.advance(end_pos);

        // b) turning right and stepping forward
        curr_node.turn(TurnDir::Right, end_pos);
        curr_node.advance(end_pos);

        for next_node in [left_node, curr_node] {
            repeatedly_add_search_node_to_frontier_and_advance_until_reaching_wall(
                next_node,
                &mut frontier,
                &mut min_cost_to_reach_state,
                grid,
                end_pos,
            );
        }
    }
    None
}

fn repeatedly_add_search_node_to_frontier_and_advance_until_reaching_wall(
    mut next_node: SearchNode,
    frontier: &mut BinaryHeap<SearchNode>,
    min_cost_to_reach_state: &mut MinSearchCostGrid,
    grid: &[Vec<CellState>],
    end_pos: Position,
) {
    while get_cell_state_at_pos(next_node.state.pos, grid) == CellState::Empty {
        // Add this node to the frontier, but only if the state has not yet been visited
        let visited_state_node =
            get_mut_cost_to_reach_state(next_node.state, min_cost_to_reach_state);
        if visited_state_node.is_none() || visited_state_node.unwrap() > next_node.g {
            *visited_state_node = Some(next_node.g);
            frontier.push(next_node.clone());
        }
        // Advance again
        next_node.advance(end_pos);
    }
}

#[inline(always)]
fn get_cell_state_at_pos(pos: Position, grid: &[Vec<CellState>]) -> CellState {
    grid[pos.y][pos.x]
}

#[inline(always)]
fn get_mut_cost_to_reach_state(
    state: State,
    min_grid_search_cost: &mut [ColumnSearchCost],
) -> &mut Option<usize> {
    &mut min_grid_search_cost[state.pos.y][state.pos.x][state.dir as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let contents = "###############\n\
                        #.......#....E#\n\
                        #.#.###.#.###.#\n\
                        #.....#.#...#.#\n\
                        #.###.#####.#.#\n\
                        #.#.#.......#.#\n\
                        #.#.#####.###.#\n\
                        #...........#.#\n\
                        ###.#.#####.#.#\n\
                        #...#.....#.#.#\n\
                        #.#.#.###.#.#.#\n\
                        #.....#...#.#.#\n\
                        #.###.#.#.#.#.#\n\
                        #S..#.....#...#\n\
                        ###############";

        assert_eq!(solve(contents), Some(7036));
    }

    #[test]
    fn test_example_2() {
        let contents = "#################\n\
                        #...#...#...#..E#\n\
                        #.#.#.#.#.#.#.#.#\n\
                        #.#.#.#...#...#.#\n\
                        #.#.#.#.###.#.#.#\n\
                        #...#.#.#.....#.#\n\
                        #.#.#.#.#.#####.#\n\
                        #.#...#.#.#.....#\n\
                        #.#.#####.#.###.#\n\
                        #.#.#.......#...#\n\
                        #.#.###.#####.###\n\
                        #.#.#...#.....#.#\n\
                        #.#.#.#####.###.#\n\
                        #.#.#.........#.#\n\
                        #.#.#.#########.#\n\
                        #S#.............#\n\
                        #################";

        assert_eq!(solve(contents), Some(11048));
    }
}
