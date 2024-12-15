use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Mul, Sub};

const INPUT_FILE_PATH: &str = "data/day8_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 8 part 2", solve, 1000);
}

type Antenna = u8;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Pos {
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

    fn get_smallest_factor(&self) -> Self {
        let x_abs = self.x.unsigned_abs();
        let y_abs = self.y.unsigned_abs();
        let divisor = gcd(x_abs.min(y_abs), x_abs.max(y_abs)) as i32;
        Self::new(self.x / divisor, self.y / divisor)
    }
}

fn gcd(small: u32, big: u32) -> u32 {
    if small == 0 {
        big
    } else {
        gcd(big % small, small)
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

impl Mul<Pos> for i32 {
    type Output = Pos;
    fn mul(self, rhs: Pos) -> Self::Output {
        Pos::new(self * rhs.x, self * rhs.y)
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
                    let offset = (pos2 - pos1).get_smallest_factor();
                    (0..)
                        .map(move |i| pos1 - i * offset)
                        .take_while(move |&pos| pos.is_valid(max_x, max_y))
                        .chain(
                            (1..)
                                .map(move |i| pos1 + i * offset)
                                .take_while(move |&pos| pos.is_valid(max_x, max_y)),
                        )
                })
            })
        })
        .collect();

    antinode_locations.len()
}
