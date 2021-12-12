use std::fmt::{Debug, Formatter};
use std::fs;
use std::ops::{Index, IndexMut};

type Grid = Vec<Vec<u64>>;
type Pos = (usize, usize);

struct Cavern {
    grid: Grid,
    row_count: usize,
    col_count: usize,
    flash_count_in_last_step: usize,
    total_flash_count: usize,
}

impl Debug for Cavern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                write!(f, "{}", cell)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Index<Pos> for Cavern {
    type Output = u64;

    fn index(&self, index: Pos) -> &Self::Output {
        let row: &Vec<u64> = self.grid.get(index.0).unwrap();
        row.get(index.1).unwrap()
    }
}

impl IndexMut<Pos> for Cavern {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        let row: &mut Vec<u64> = self.grid.get_mut(index.0).unwrap();
        row.get_mut(index.1).unwrap()
    }
}

impl Cavern {
    fn new(text: &str) -> Self {
        let grid: Grid = text
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|ch| ch as u64 - b'0' as u64)
                    .collect::<Vec<u64>>()
            })
            .collect();
        let row_count = grid.len();
        let col_count = grid[0].len();
        Self {
            grid,
            row_count,
            col_count,
            flash_count_in_last_step: 0,
            total_flash_count: 0,
        }
    }

    fn get_neighbours(&self, pos: Pos) -> Vec<Pos> {
        let offsets: [(isize, isize); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        offsets
            .iter()
            .filter_map(|(row_offset, col_offset)| {
                let row = pos.0 as isize + row_offset;
                let col = pos.1 as isize + col_offset;
                (row >= 0
                    && col >= 0
                    && (row as usize) < self.row_count
                    && (col as usize) < self.col_count)
                    .then(|| (row as usize, col as usize))
            })
            .collect::<Vec<Pos>>()
    }

    fn get_cells(&self) -> Vec<Pos> {
        (0..self.row_count)
            .flat_map(|row| (0..self.col_count).map(move |col| (row, col)))
            .collect::<Vec<Pos>>()
    }

    fn step(&mut self) {
        let mut cells_to_increment: Vec<Pos> = self.get_cells();
        let mut increment_if_zero: bool = true;
        self.flash_count_in_last_step = 0;

        while !cells_to_increment.is_empty() {
            let cells_to_flash: Vec<Pos> = cells_to_increment
                .iter()
                .filter_map(|&pos| {
                    if increment_if_zero || self[pos] > 0 {
                        self[pos] += 1;
                    }
                    (self[pos] > 9).then(|| pos)
                })
                .collect();

            cells_to_increment = cells_to_flash
                .iter()
                .flat_map(|&pos| {
                    if self[pos] > 0 {
                        self[pos] = 0;
                        self.flash_count_in_last_step += 1;
                        let neighbours = self.get_neighbours(pos);
                        neighbours
                            .into_iter()
                            .filter(|&adj_pos| self[adj_pos] > 0)
                            .collect::<Vec<Pos>>()
                    } else {
                        Vec::<Pos>::new()
                    }
                })
                .collect();

            increment_if_zero = false;
        }
        self.total_flash_count += self.flash_count_in_last_step;
    }

    fn flash_synchronized_in_last_step(&self) -> bool {
        self.flash_count_in_last_step == self.row_count * self.col_count
    }
}

fn main() {
    let contents = fs::read_to_string("data/day11_input.txt").unwrap();
    let mut cavern = Cavern::new(contents.as_str());
    let mut part1_total_flash_count = 0;
    let mut part2_first_synchronized_step = 0;
    for step in 1.. {
        cavern.step();
        if step == 100 {
            part1_total_flash_count = cavern.total_flash_count;
        }
        if cavern.flash_synchronized_in_last_step() {
            part2_first_synchronized_step = step;
        }
        if part1_total_flash_count > 0 && part2_first_synchronized_step > 0 {
            break;
        }
    }
    println!("Part 1: flashed {} times", part1_total_flash_count);
    println!(
        "Part 2: first synchronized on step {}",
        part2_first_synchronized_step
    );
}

#[cfg(test)]
mod tests {
    use crate::{Cavern, Pos};

    const EXAMPLE_SMALL_CAVERN: &str = "11111\n\
         19991\n\
         19191\n\
         19991\n\
         11111\n";

    const STEP1_SMALL_CAVERN: &str = "34543\n\
         40004\n\
         50005\n\
         40004\n\
         34543\n";

    const EXAMPLE_LARGE_CAVERN: &str = "5483143223\n\
         2745854711\n\
         5264556173\n\
         6141336146\n\
         6357385478\n\
         4167524645\n\
         2176841721\n\
         6882881134\n\
         4846848554\n\
         5283751526\n";

    const STEP1_LARGE_CAVERN: &str = "6594254334\n\
         3856965822\n\
         6375667284\n\
         7252447257\n\
         7468496589\n\
         5278635756\n\
         3287952832\n\
         7993992245\n\
         5957959665\n\
         6394862637\n";

    const STEP2_LARGE_CAVERN: &str = "8807476555\n\
         5089087054\n\
         8597889608\n\
         8485769600\n\
         8700908800\n\
         6600088989\n\
         6800005943\n\
         0000007456\n\
         9000000876\n\
         8700006848\n";

    const STEP10_LARGE_CAVERN: &str = "0481112976\n\
         0031112009\n\
         0041112504\n\
         0081111406\n\
         0099111306\n\
         0093511233\n\
         0442361130\n\
         5532252350\n\
         0532250600\n\
         0032240000\n";

    #[test]
    fn test_neighbours_in_middle() {
        let cavern = Cavern::new(EXAMPLE_SMALL_CAVERN);
        let neighbours = cavern.get_neighbours((2, 3));
        let expected_neighbours: Vec<Pos> = vec![
            (1, 2),
            (1, 3),
            (1, 4),
            (2, 2),
            (2, 4),
            (3, 2),
            (3, 3),
            (3, 4),
        ];
        assert_eq!(neighbours, expected_neighbours);
    }

    #[test]
    fn test_neighbours_of_top_left_corner() {
        let cavern = Cavern::new(EXAMPLE_SMALL_CAVERN);
        let neighbours = cavern.get_neighbours((0, 0));
        let expected_neighbours: Vec<Pos> = vec![(0, 1), (1, 0), (1, 1)];
        assert_eq!(neighbours, expected_neighbours);
    }

    #[test]
    fn test_neighbours_of_bottom_right_corner() {
        let cavern = Cavern::new(EXAMPLE_SMALL_CAVERN);
        let neighbours = cavern.get_neighbours((cavern.row_count - 1, cavern.col_count - 1));
        let expected_neighbours: Vec<Pos> = vec![
            (cavern.row_count - 2, cavern.col_count - 2),
            (cavern.row_count - 2, cavern.col_count - 1),
            (cavern.row_count - 1, cavern.col_count - 2),
        ];
        assert_eq!(neighbours, expected_neighbours);
    }

    #[test]
    fn test_small_cavern_one_step() {
        let mut cavern = Cavern::new(EXAMPLE_SMALL_CAVERN);
        cavern.step();
        let cavern_step1_str = format!("{:?}", cavern);
        assert_eq!(cavern_step1_str, STEP1_SMALL_CAVERN);
    }

    #[test]
    fn test_large_cavern_one_step() {
        let mut cavern = Cavern::new(EXAMPLE_LARGE_CAVERN);
        cavern.step();
        let cavern_step1_str = format!("{:?}", cavern);
        assert_eq!(cavern_step1_str, STEP1_LARGE_CAVERN);
    }

    #[test]
    fn test_large_cavern_two_steps_flash_count() {
        let mut cavern = Cavern::new(EXAMPLE_LARGE_CAVERN);
        cavern.step();
        cavern.step();
        let cavern_step2_str = format!("{:?}", cavern);
        assert_eq!(cavern_step2_str, STEP2_LARGE_CAVERN);
    }

    #[test]
    fn test_large_cavern_ten_steps_flash_count() {
        let mut cavern = Cavern::new(EXAMPLE_LARGE_CAVERN);
        for _ in 0..10 {
            cavern.step();
        }
        let cavern_step10_str = format!("{:?}", cavern);
        assert_eq!(cavern_step10_str, STEP10_LARGE_CAVERN);
        assert_eq!(cavern.total_flash_count, 204);
    }
}
