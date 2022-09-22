use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs;
use std::time::Instant;

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    row: usize,
    col: usize,
    total_risk: usize,
}

// Order so that lower risk levels are greater since BinaryHeap is a max-heap
impl PartialOrd<Self> for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.total_risk.cmp(&other.total_risk) {
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Equal => None,
            Ordering::Greater => Some(Ordering::Less),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if let Some(order) = self.partial_cmp(other) {
            order
        } else if self.row < other.row {
            Ordering::Greater
        } else if self.row > other.row {
            Ordering::Less
        } else {
            self.col.cmp(&other.col).reverse()
        }
    }
}

const OFFSETS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day15_input.txt").unwrap();
    let risk_levels: Vec<Vec<_>> = contents
        .lines()
        .map(|line| line.chars().map(|ch| ch as u8 - b'0').collect::<Vec<u8>>())
        .collect();
    let part2_answer = get_lowest_risk::<5>(&risk_levels);

    let duration = start_time.elapsed();
    println!("Part 2 answer: {}", part2_answer);
    println!("Duration: {:?}", duration); // 35.313251ms

    let part1_answer = get_lowest_risk::<1>(&risk_levels);
    println!("Part 1 answer: {}", part1_answer); // Duration: 1.389793ms
}

fn get_lowest_risk<const M: usize>(risk_levels: &Vec<Vec<u8>>) -> usize {
    let orig_row_count = risk_levels.len();
    let orig_col_count = risk_levels[0].len();
    let row_count = M * orig_row_count;
    let col_count = M * orig_col_count;
    let mut visited: Vec<Vec<bool>> = vec![vec![false; col_count]; row_count];
    let mut node_queue: BinaryHeap<Node> = BinaryHeap::with_capacity(row_count.max(col_count));
    node_queue.push(Node {
        row: 0,
        col: 0,
        total_risk: 0,
    });

    loop {
        let next_node = node_queue
            .pop()
            .expect("The queue of nodes is empty, surprisingly!");
        let is_visited = &mut visited[next_node.row][next_node.col];
        if !*is_visited {
            if next_node.row == row_count - 1 && next_node.col == col_count - 1 {
                break next_node.total_risk;
            }
            *is_visited = true;

            for (row_offset, col_offset) in OFFSETS {
                let new_row = next_node.row as isize + row_offset;
                if new_row < 0 {
                    continue;
                }

                let new_col = next_node.col as isize + col_offset;
                if new_col < 0 {
                    continue;
                }

                let row = new_row as usize;
                if row >= row_count {
                    continue;
                }

                let col = new_col as usize;
                if col >= col_count {
                    continue;
                }

                if visited[row][col] {
                    continue;
                }

                let orig_cell_risk_level =
                    risk_levels[row % orig_row_count][col % orig_col_count] as usize;
                let cell_risk_level =
                    (orig_cell_risk_level + (row / orig_row_count) + (col / orig_col_count) - 1)
                        % 9
                        + 1;

                node_queue.push(Node {
                    row,
                    col,
                    total_risk: next_node.total_risk + cell_risk_level,
                })
            }
        }
    }
}
