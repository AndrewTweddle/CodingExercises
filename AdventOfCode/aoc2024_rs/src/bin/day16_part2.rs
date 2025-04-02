use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::Lines;

const INPUT_FILE_PATH: &str = "data/day16_input.txt";
const COST_TO_TURN: usize = 1000;

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 16 part 2", solve, 1000);
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

    fn calculate_f(state: State, g: usize, end_pos: Position) -> usize {
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
    get_cells_on_any_min_cost_path_using_astar_algorithm(&grid, start_pos, end_pos)
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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct PathNode {
    min_cost: usize,
    prev_states: Vec<State>,
    is_visited: bool,
}

impl PathNode {
    fn new(min_cost: usize, prev_state: Option<State>) -> Self {
        Self {
            min_cost,
            prev_states: if let Some(prev) = prev_state {
                vec![prev]
            } else {
                Vec::new()
            },
            is_visited: false,
        }
    }
}

type DirPath = [Option<PathNode>; 4];
type ColumnPath = Vec<DirPath>;
type PathGrid = Vec<ColumnPath>;

fn get_cells_on_any_min_cost_path_using_astar_algorithm(
    grid: &[Vec<CellState>],
    start_pos: Position,
    end_pos: Position,
) -> Option<usize> {
    // Track the shortest path to reach each state (being a combination of position and direction)
    let mut path_grid: PathGrid = vec![vec![[const { None }; 4]; grid[0].len()]; grid.len()];

    // Also add a virtual PathNode for the target, since it is independent of the direction
    let mut final_node: Option<PathNode> = None;
    find_best_paths_using_astar_and_get_min_cost(
        grid,
        start_pos,
        end_pos,
        &mut path_grid,
        &mut final_node,
    );

    final_node.map(|node| get_number_of_cells_on_any_shortest_path_from_path_grid(
        &mut path_grid,
        node,
    ))
}

fn find_best_paths_using_astar_and_get_min_cost(
    grid: &[Vec<CellState>],
    start_pos: Position,
    end_pos: Position,
    path_grid: &mut PathGrid,
    final_node: &mut Option<PathNode>,
) -> Option<usize> {
    let mut frontier = BinaryHeap::<SearchNode>::new();

    // Each move will consist of turning either right or left by 90 degrees,
    // then advancing 1 or more steps to a reachable empty space.
    // However, on the very first turn, don't turn first, just step 0 or more spaces forwards.
    // This will seed the frontier of initial states to traverse.
    let start_state = State {
        pos: start_pos,
        dir: Direction::Right,
    };

    // Track the best solution found thus far, to prune search paths that can never beat it.
    let mut min_cost: Option<usize> = None;

    // Seed frontier with a search node for the initial position
    frontier.push(SearchNode::new(start_state, 0, end_pos));

    while let Some(mut curr_node) = frontier.pop() {
        if curr_node.state.pos == end_pos {
            update_final_node(final_node, &curr_node);

            // The min cost solution might have been updated
            min_cost = if let Some(fin) = final_node {
                Some(fin.min_cost)
            } else {
                None
            };
            continue;
        }

        let prev_state = curr_node.state;

        // We consider 3 options:
        // a) turn left and step forward
        let mut left_node = curr_node.clone();
        left_node.turn(TurnDir::Left, end_pos);
        left_node.advance(end_pos);

        // b) turn right and step forward
        let mut right_node = curr_node.clone();
        right_node.turn(TurnDir::Right, end_pos);
        right_node.advance(end_pos);

        // c) step forward:
        curr_node.advance(end_pos);

        // Add any of these 3 search nodes to the frontier if they are at empty spaces
        for next_node in [left_node, right_node, curr_node] {
            add_search_node_to_frontier_if_not_over_a_wall(
                next_node,
                Some(prev_state),
                min_cost,
                &mut frontier,
                path_grid,
                grid,
            );
        }
    }

    min_cost
}

fn update_final_node(final_node: &mut Option<PathNode>, curr_node: &SearchNode) {
    if let Some(final_node) = final_node {
        match final_node.min_cost.cmp(&curr_node.g) {
            Ordering::Less => {
                // Ignore
            }
            Ordering::Equal => {
                // This is a different way of reaching the same min cost
                final_node.prev_states.push(curr_node.state);
            }
            Ordering::Greater => {
                // This is a shorter path, found later. This should not be possible,
                // since the heuristic function is admissible.
                panic!("Unexpectedly found shorter path to final node");
            }
        }
    } else {
        *final_node = Some(PathNode::new(curr_node.g, Some(curr_node.state)));
    }
}

fn add_search_node_to_frontier_if_not_over_a_wall(
    next_node: SearchNode,
    prev_state: Option<State>,
    min_cost: Option<usize>,
    frontier: &mut BinaryHeap<SearchNode>,
    path_grid: &mut PathGrid,
    grid: &[Vec<CellState>],
) {
    if get_cell_state_at_pos(next_node.state.pos, grid) != CellState::Empty {
        return;
    }
    // Add this node to the frontier, but only if the state has not yet been visited.
    // If it has, and this is an alternate shortest path to the state, then add its
    // previous state, so that all best paths can be found.
    let path_node = get_mut_path_node_to_state(next_node.state, path_grid);

    match path_node {
        None => {
            *path_node = Some(PathNode::new(next_node.g, prev_state));
            if min_cost.is_none() || next_node.f <= min_cost.unwrap() {
                // Add it to the frontier, but only because a min cost solution either hasn't
                // been found yet, or this path has a chance of equalling the min cost solution
                frontier.push(next_node);
            }
        }
        Some(ref mut pn) => {
            match pn.min_cost.cmp(&next_node.g) {
                Ordering::Less => {
                    // Ignore worse path to the node
                }
                Ordering::Equal => {
                    // This is another way of reaching the same search node,
                    // so add the previous search node's state to the previous states
                    if let Some(prev) = prev_state {
                        pn.prev_states.push(prev);
                    }
                }
                Ordering::Greater => {
                    pn.min_cost = next_node.g;
                    pn.prev_states = vec![prev_state.unwrap()];
                    if min_cost.is_none() || next_node.f <= min_cost.unwrap() {
                        // Add it to the frontier, but only because a min cost solution either hasn't
                        // been found yet, or this path has a chance of equalling the min cost solution
                        frontier.push(next_node);
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn get_cell_state_at_pos(pos: Position, grid: &[Vec<CellState>]) -> CellState {
    grid[pos.y][pos.x]
}

fn get_number_of_cells_on_any_shortest_path_from_path_grid(
    path_grid: &mut PathGrid,
    mut final_node: PathNode,
) -> usize {
    // Traverse the path grid backwards from any of the 4 states (one per direction)
    // at the target position. Count the number of cells on each path, including start & end.
    let mut is_on_shortest_path_grid: Vec<Vec<bool>> =
        vec![vec![false; path_grid[0].len()]; path_grid.len()];

    let mut states_to_visit: Vec<State> = final_node.prev_states.drain(0..).collect();
    let mut cells_on_a_shortest_path: usize = 0;

    while let Some(next_state) = states_to_visit.pop() {
        let is_on_shortest_path = &mut is_on_shortest_path_grid[next_state.pos.y][next_state.pos.x];
        if !*is_on_shortest_path {
            cells_on_a_shortest_path += 1;
            *is_on_shortest_path = true;
        }

        if let &mut Some(ref mut path_node) = get_mut_path_node_to_state(next_state, path_grid) {
            if !path_node.is_visited {
                states_to_visit.append(&mut path_node.prev_states);
                path_node.is_visited = true;
            }
        }
    }

    cells_on_a_shortest_path
}

#[inline(always)]
fn get_mut_path_node_to_state(state: State, path_grid: &mut [ColumnPath]) -> &mut Option<PathNode> {
    &mut path_grid[state.pos.y][state.pos.x][state.dir as usize]
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

        assert_eq!(solve(contents), Some(45));
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

        assert_eq!(solve(contents), Some(64));
    }
}
