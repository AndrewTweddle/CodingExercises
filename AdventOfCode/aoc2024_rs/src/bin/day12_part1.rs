use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::iter;

const INPUT_FILE_PATH: &str = "data/day12_input.txt";

struct Region {
    id: usize,
    plant_type: u8,
    area: usize,
    perimeter: usize,
    sub_regions: Vec<usize>,
}

impl Region {
    fn new(id: usize, plant_type: u8, area: usize, perimeter: usize) -> Self {
        Region {
            id,
            plant_type,
            area,
            perimeter,
            sub_regions: vec![],
        }
    }

    fn get_fence_price(&self) -> usize {
        self.area * self.perimeter
    }

    fn merge(&mut self, other: &mut Self, sub_region_to_region_map: &mut [usize]) {
        assert_eq!(self.plant_type, other.plant_type);

        let sub_regions_to_remap = if self.id < other.id {
            &other.sub_regions
        } else {
            std::mem::swap(&mut self.id, &mut other.id);
            &self.sub_regions
        };

        for &i in sub_regions_to_remap {
            sub_region_to_region_map[i] = self.id;
        }

        self.sub_regions.append(&mut other.sub_regions);
        self.perimeter += other.perimeter;
        self.area += other.area;

        other.perimeter = 0;
        other.area = 0;
    }
}

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 12 part 1", solve, 1000);
}

fn solve(contents: &str) -> usize {
    // Create a grid of bytes, padded with 1 extra line on each of the 4 sides,
    // to avoid needing to do bounds checking:
    let mut grid: Vec<Vec<u8>> = [const { Vec::<u8>::new() }; 1]
        .iter()
        .cloned()
        .chain(contents.lines().map(|line| {
            iter::once(b' ')
                .chain(line.bytes())
                .chain(iter::once(b' '))
                .collect::<Vec<u8>>()
        }))
        .chain([vec![]])
        .collect();

    let padded_row_count = grid.len();

    // Fill in the first and last row to match the length of the other rows
    let padded_col_count = grid[1].len();
    grid[0].extend(iter::repeat(b' ').take(padded_col_count));
    grid[padded_col_count - 1].extend(iter::repeat(b' ').take(padded_col_count));

    // Track regions and sub-regions. Each region can have one or more sub-regions.
    // The sub-regions are marked on the board. However, apparently separate sub-regions
    // can later be discovered to be part of the same region.
    // Identify each region by the smallest numbered sub-region in that region.
    let mut regions: Vec<Region> = vec![];
    let mut sub_region_to_region_map: Vec<usize> = vec![]; // The index is the sub-region
    let mut sub_region_map: Vec<Vec<Option<usize>>> =
        vec![Vec::<Option<usize>>::with_capacity(padded_col_count); padded_row_count];

    // Build up regions and sub-regions
    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, &byte)| {
            if byte == b' ' {
                sub_region_map[r].push(None);
                if r > 0 && c > 0 && c < padded_col_count - 1 {
                    // This is the last row, so update the perimeter of the region above
                    if let Some(sub_region_id) = sub_region_map[r - 1][c] {
                        regions[sub_region_to_region_map[sub_region_id]].perimeter += 1;
                    }
                } else if c > 0 {
                    // This is the last column, so update the region to the left
                    if let Some(sub_region_id) = sub_region_map[r][c - 1] {
                        regions[sub_region_to_region_map[sub_region_id]].perimeter += 1;
                    }
                }
            } else {
                let left_byte = grid[r][c - 1];
                let top_byte = grid[r - 1][c];

                match (left_byte, byte, top_byte) {
                    (lb, cb, tb) if lb == cb && tb == cb => {
                        merge_left_and_top_region(
                            r,
                            c,
                            &mut regions,
                            &mut sub_region_map,
                            &mut sub_region_to_region_map,
                        );
                    }
                    (lb, cb, tb) if lb == cb => {
                        add_cell_to_left_region(
                            r,
                            c,
                            &mut regions,
                            &mut sub_region_map,
                            &mut sub_region_to_region_map,
                        );

                        increase_perimeter_of_top_region(
                            r,
                            c,
                            tb,
                            &mut regions,
                            &mut sub_region_map,
                            &mut sub_region_to_region_map,
                        );
                    }
                    (lb, cb, tb) if tb == cb => {
                        add_cell_to_top_region(
                            r,
                            c,
                            &mut regions,
                            &mut sub_region_map,
                            &mut sub_region_to_region_map,
                        );

                        increase_perimeter_of_left_region(
                            r,
                            c,
                            lb,
                            &mut regions,
                            &mut sub_region_map,
                            &mut sub_region_to_region_map,
                        );
                    }
                    (lb, _, tb) => {
                        add_cell_to_new_region(
                            r,
                            byte,
                            &mut regions,
                            &mut sub_region_map,
                            &mut sub_region_to_region_map,
                        );

                        increase_perimeter_of_left_region(
                            r,
                            c,
                            lb,
                            &mut regions,
                            &mut sub_region_map,
                            &mut sub_region_to_region_map,
                        );

                        increase_perimeter_of_top_region(
                            r,
                            c,
                            tb,
                            &mut regions,
                            &mut sub_region_map,
                            &mut sub_region_to_region_map,
                        );
                    }
                }
            }
        })
    });

    regions.iter().map(|rgn| rgn.get_fence_price()).sum()
}

fn merge_left_and_top_region(
    r: usize,
    c: usize,
    regions: &mut Vec<Region>,
    sub_region_map: &mut Vec<Vec<Option<usize>>>,
    sub_region_to_region_map: &mut Vec<usize>,
) {
    // merge the left and top regions
    let left_sub_region_id = sub_region_map[r][c - 1].unwrap();

    // Add this cell to the left region
    let left_region = &mut regions[sub_region_to_region_map[left_sub_region_id]];
    left_region.area += 1;
    sub_region_map[r].push(Some(left_sub_region_id));

    // merge the left and top regions
    let top_sub_region_id = sub_region_map[r - 1][c].unwrap();
    merge_sub_regions(
        left_sub_region_id,
        top_sub_region_id,
        sub_region_to_region_map,
        regions,
    );
}

fn merge_sub_regions(
    sub_region1_id: usize,
    sub_region2_id: usize,
    sub_region_to_region_map: &mut [usize],
    regions: &mut [Region],
) {
    if sub_region1_id == sub_region2_id {
        return;
    }
    let region1_id = sub_region_to_region_map[sub_region1_id];
    let region2_id = sub_region_to_region_map[sub_region2_id];
    if region1_id == region2_id {
        return;
    }
    let (src_region_id, dst_region_id) = if region1_id < region2_id {
        (region2_id, region1_id)
    } else {
        (region1_id, region2_id)
    };

    let (dst_range, src_range) = regions.split_at_mut(src_region_id);
    let dst_region = &mut dst_range[dst_region_id];
    let src_region = &mut src_range[0];
    dst_region.merge(src_region, sub_region_to_region_map);
}

fn add_cell_to_left_region(
    r: usize,
    c: usize,
    regions: &mut Vec<Region>,
    sub_region_map: &mut Vec<Vec<Option<usize>>>,
    sub_region_to_region_map: &mut Vec<usize>,
) {
    // expand the left region to include the new cell
    let left_sub_region_id = sub_region_map[r][c - 1].unwrap();
    let left_region = &mut regions[sub_region_to_region_map[left_sub_region_id]];
    left_region.area += 1;
    left_region.perimeter += 1;
    sub_region_map[r].push(Some(left_sub_region_id));
}

fn add_cell_to_top_region(
    r: usize,
    c: usize,
    regions: &mut Vec<Region>,
    sub_region_map: &mut Vec<Vec<Option<usize>>>,
    sub_region_to_region_map: &mut Vec<usize>,
) {
    // expand the top region to include the new cell
    let top_sub_region_id = sub_region_map[r - 1][c].unwrap();
    let top_region = &mut regions[sub_region_to_region_map[top_sub_region_id]];
    top_region.area += 1;
    top_region.perimeter += 1;
    sub_region_map[r].push(Some(top_sub_region_id));
}

fn add_cell_to_new_region(
    r: usize,
    byte: u8,
    regions: &mut Vec<Region>,
    sub_region_map: &mut Vec<Vec<Option<usize>>>,
    sub_region_to_region_map: &mut Vec<usize>,
) {
    // Start a new sub-region and region at the current cell
    let new_region_id = regions.len();
    let mut new_region = Region::new(new_region_id, byte, 1, 2);
    sub_region_map[r].push(Some(new_region_id));
    new_region.sub_regions.push(new_region_id);
    regions.push(new_region);
    sub_region_to_region_map.push(new_region_id);
}

fn increase_perimeter_of_left_region(
    r: usize,
    c: usize,
    left_byte: u8,
    regions: &mut Vec<Region>,
    sub_region_map: &mut Vec<Vec<Option<usize>>>,
    sub_region_to_region_map: &mut Vec<usize>,
) {
    if left_byte != b' ' {
        let left_sub_region_id = sub_region_map[r][c - 1].unwrap();
        regions[sub_region_to_region_map[left_sub_region_id]].perimeter += 1;
    }
}

fn increase_perimeter_of_top_region(
    r: usize,
    c: usize,
    top_byte: u8,
    regions: &mut Vec<Region>,
    sub_region_map: &mut Vec<Vec<Option<usize>>>,
    sub_region_to_region_map: &mut Vec<usize>,
) {
    if top_byte != b' ' {
        let top_sub_region_id = sub_region_map[r - 1][c].unwrap();
        regions[sub_region_to_region_map[top_sub_region_id]].perimeter += 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn small_example() {
        let contents = "AAAA
BBCD
BBCC
EEEC";
        let a_price = 4 * 10;
        let b_price = 4 * 8;
        let c_price = 4 * 10;
        let d_price = 1 * 4;
        let e_price = 3 * 8;
        let fence_price = a_price + b_price + c_price + d_price + e_price;
        assert_eq!(solve(contents), fence_price);
    }
}
