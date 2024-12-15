use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

const INPUT_FILE_PATH: &str = "data/day8_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 8 part 1", solve, 1000);
}

type Antenna = u8;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn is_valid(&self, max_x: i32, max_y: i32) -> bool {
        self.x >= 0 && self.x <= max_x && self.y >= 0 && self.y <= max_y
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Pos {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

fn solve(contents: &str) -> usize {
    let mut antenna_positions: HashMap<Antenna, Vec<Pos>> = HashMap::new();
    let lines: Vec<&str> = contents.lines().collect();
    let max_x = lines[0].len() as i32 - 1;
    let max_y = lines.len() as i32 - 1;

    // Group antennas
    lines.iter().enumerate().for_each(|(y, line)| {
        line.bytes()
            .enumerate()
            .filter(|&(_, antenna)| antenna != b'.')
            .for_each(|(x, antenna)| {
                antenna_positions
                    .entry(antenna)
                    .or_default()
                    .push(Pos::new(x as i32, y as i32))
            })
    });

    let antinode_locations: HashSet<Pos> = antenna_positions
        .iter()
        .flat_map(|(_, positions)| {
            positions.iter().enumerate().flat_map(|(index, &pos2)| {
                positions.iter().take(index).flat_map(move |&pos1| {
                    let mut antinode_locs: Vec<Pos> = Vec::with_capacity(4);
                    let offset = pos2 - pos1;
                    add_pos_if_valid(&mut antinode_locs, pos1 - offset, max_x, max_y);
                    add_pos_if_valid(&mut antinode_locs, pos2 + offset, max_x, max_y);
                    if offset.x % 3 == 0 && offset.y % 3 == 0 {
                        let inner_offset = Pos::new(offset.x / 3, offset.y / 3);
                        antinode_locs.push(pos1 + inner_offset);
                        antinode_locs.push(pos2 - inner_offset);
                    }
                    antinode_locs
                })
            })
        })
        .collect();

    antinode_locations.len()
}

#[inline(always)]
fn add_pos_if_valid(antinode_locs: &mut Vec<Pos>, candidate: Pos, max_x: i32, max_y: i32) {
    if candidate.is_valid(max_x, max_y) {
        antinode_locs.push(candidate)
    };
}
