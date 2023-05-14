use std::time::Instant;
use std::collections::HashSet;

#[derive(Default, Eq, Hash, PartialEq, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize
}

impl Pos {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn get_new_pos(&self, prev_knot_new_pos: Pos) -> Option<Pos> {
        match (prev_knot_new_pos.x, prev_knot_new_pos.y) {
            // prev knot moved right to be no longer adjacent
            (nx, ny) if self.x + 2 == nx && ny == self.y => Some(Pos::new(self.x + 1, ny)),
            (nx, ny) if self.x + 2 == nx && ny > self.y => Some(Pos::new(self.x + 1, self.y + 1)),
            (nx, ny) if self.x + 2 == nx && ny < self.y => Some(Pos::new(self.x + 1, self.y - 1)),
            // prev knot moved left to be no longer adjacent
            (nx, ny) if self.x - 2 == nx && ny == self.y => Some(Pos::new(self.x - 1, ny)),
            (nx, ny) if self.x - 2 == nx && ny > self.y => Some(Pos::new(self.x - 1, self.y + 1)),
            (nx, ny) if self.x - 2 == nx && ny < self.y => Some(Pos::new(self.x - 1, self.y - 1)),
            // prev knot moved up to be no longer adjacent
            (nx, ny) if self.y + 2 == ny && nx == self.x => Some(Pos::new(nx, self.y + 1)),
            (nx, ny) if self.y + 2 == ny && nx > self.x => Some(Pos::new(self.x + 1, self.y + 1)),
            (nx, ny) if self.y + 2 == ny && nx < self.x => Some(Pos::new(self.x - 1, self.y + 1)),
            // prev knot moved down to be no longer adjacent
            (nx, ny) if self.y - 2 == ny && nx == self.x => Some(Pos::new(nx, self.y - 1)),
            (nx, ny) if self.y - 2 == ny && nx > self.x => Some(Pos::new(self.x + 1, self.y - 1)),
            (nx, ny) if self.y - 2 == ny && nx < self.x => Some(Pos::new(self.x - 1, self.y - 1)),
            // prev knot is still adjacent to (including diagonally) or co-located with this knot
            _ => None
        }
    }
}

const REPETITIONS: u32 = 1000;

fn main() {
    let mut start_time = Instant::now();
    let contents = std::fs::read_to_string("data/day9_input.txt").unwrap();
    for rep in 0..=REPETITIONS {
        let mut tail_cells_visited = HashSet::<Pos>::new();
        let mut positions: [Pos; 10] = [Default::default(); 10];
        tail_cells_visited.insert(Default::default());
        for line in contents.lines() {
            let (dir_str, steps_str) = line.split_at(2);
            let dir = dir_str.chars().next().unwrap();
            let steps = steps_str.parse::<u8>().unwrap();
            for _ in 0..steps {
                let mut new_head = positions[0];
                match dir {
                    'R' => new_head.x += 1,
                    'L' => new_head.x -= 1,
                    'U' => new_head.y += 1,
                    'D' => new_head.y -= 1,
                    unknown_char => panic!("Unrecognized direction: {unknown_char}"),
                }
                slither(new_head, &mut positions, &mut tail_cells_visited);
            }
        }
        if rep == 0 {
            println!("AOC 2022: day 9 part 2: {}", tail_cells_visited.len());
            println!(
                "Duration of iteration 0 including I/O: {:?}",
                start_time.elapsed()
            );

            // Restart timer, so that I/O is not being timed
            start_time = Instant::now();
        }
    }
    let duration = start_time.elapsed();
    println!(
        "Avg duration of {} repetitions (excluding I/O): {:?}",
        REPETITIONS,
        duration / REPETITIONS
    );
}

fn slither(new_pos: Pos, rem_knots: &mut [Pos], tail_cells_visited: &mut HashSet<Pos>) {
    if let Some(&prev_pos) = rem_knots.get(0) {
        if prev_pos != new_pos {
            rem_knots[0] = new_pos;
            if let Some(next_knot_pos) = rem_knots.get(1) {
                if let Some(next_knot_new_pos) = next_knot_pos.get_new_pos(new_pos) {
                    slither(next_knot_new_pos, &mut rem_knots[1..], tail_cells_visited)
                }
            } else {
                // This is the tail, and it has reached a new position...
                tail_cells_visited.insert(new_pos);
            }
        } else {
            panic!("Unexpectedly found that previous knot didn't move");
        }
    } else {
        panic!("Unexpectedly reached the end of the rope!");
    }
}