use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::iter;

const INPUT_FILE_PATH: &str = "data/day12_input.txt";

struct Region {
    id: usize,
    plant_type: u8,
    area: usize,
    number_of_sides: usize,
    sub_regions: Vec<usize>,
}

impl Region {
    fn new(id: usize, plant_type: u8, area: usize) -> Self {
        Region {
            id,
            plant_type,
            area,
            number_of_sides: 0,
            sub_regions: vec![],
        }
    }

    fn get_fence_price(&self) -> usize {
        self.area * self.number_of_sides
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
        self.area += other.area;

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

    build_sub_regions_and_regions(
        &mut grid,
        &mut regions,
        &mut sub_region_map,
        &mut sub_region_to_region_map,
    );

    // Turn sub_region_map into a region_map to simplify further calculations:
    for row in sub_region_map.iter_mut() {
        for cell in row.iter_mut() {
            if let Some(sub_region) = *cell {
                *cell = Some(sub_region_to_region_map[sub_region]);
            }
        }
    }

    // Count top and bottom edges of each region
    sub_region_map
        .iter()
        .skip(1)
        .zip(sub_region_map.iter())
        .for_each(|(row_below, row_above)| {
            update_side_counts_of_both_adjacent_lines(
                row_below.iter().cloned(),
                row_above.iter().cloned(),
                &mut regions,
            );
        });

    // Count left and right edges of each region
    let row_count = sub_region_map.len();
    let col_count = sub_region_map[0].len();

    (0..col_count).skip(1).for_each(|c| {
        let left_col_iter = (0..row_count).map(|r| sub_region_map[r][c - 1]);
        let right_col_iter = (0..row_count).map(|r| sub_region_map[r][c]);

        update_side_counts_of_both_adjacent_lines(
            right_col_iter.clone(),
            left_col_iter.clone(),
            &mut regions,
        );
    });

    regions.iter().map(|rgn| rgn.get_fence_price()).sum()
}

fn build_sub_regions_and_regions(
    grid: &mut [Vec<u8>],
    regions: &mut Vec<Region>,
    sub_region_map: &mut [Vec<Option<usize>>],
    sub_region_to_region_map: &mut Vec<usize>,
) {
    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, &byte)| {
            if byte == b' ' {
                sub_region_map[r].push(None);
            } else {
                let left_byte = grid[r][c - 1];
                let top_byte = grid[r - 1][c];

                match (left_byte, byte, top_byte) {
                    (lb, cb, tb) if lb == cb && tb == cb => {
                        merge_left_and_top_region(
                            r,
                            c,
                            regions,
                            sub_region_map,
                            sub_region_to_region_map,
                        );
                    }
                    (lb, cb, _) if lb == cb => {
                        add_cell_to_left_region(
                            r,
                            c,
                            regions,
                            sub_region_map,
                            sub_region_to_region_map,
                        );
                    }
                    (_, cb, tb) if tb == cb => {
                        add_cell_to_top_region(
                            r,
                            c,
                            regions,
                            sub_region_map,
                            sub_region_to_region_map,
                        );
                    }
                    _ => {
                        add_cell_to_new_region(
                            r,
                            byte,
                            regions,
                            sub_region_map,
                            sub_region_to_region_map,
                        );
                    }
                }
            }
        })
    });
}

fn merge_left_and_top_region(
    r: usize,
    c: usize,
    regions: &mut [Region],
    sub_region_map: &mut [Vec<Option<usize>>],
    sub_region_to_region_map: &mut [usize],
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
    regions: &mut [Region],
    sub_region_map: &mut [Vec<Option<usize>>],
    sub_region_to_region_map: &mut [usize],
) {
    // expand the left region to include the new cell
    let left_sub_region_id = sub_region_map[r][c - 1].unwrap();
    let left_region = &mut regions[sub_region_to_region_map[left_sub_region_id]];
    left_region.area += 1;
    sub_region_map[r].push(Some(left_sub_region_id));
}

fn add_cell_to_top_region(
    r: usize,
    c: usize,
    regions: &mut [Region],
    sub_region_map: &mut [Vec<Option<usize>>],
    sub_region_to_region_map: &mut [usize],
) {
    // expand the top region to include the new cell
    let top_sub_region_id = sub_region_map[r - 1][c].unwrap();
    let top_region = &mut regions[sub_region_to_region_map[top_sub_region_id]];
    top_region.area += 1;
    sub_region_map[r].push(Some(top_sub_region_id));
}

fn add_cell_to_new_region(
    r: usize,
    byte: u8,
    regions: &mut Vec<Region>,
    sub_region_map: &mut [Vec<Option<usize>>],
    sub_region_to_region_map: &mut Vec<usize>,
) {
    // Start a new sub-region and region at the current cell
    let new_region_id = regions.len();
    let mut new_region = Region::new(new_region_id, byte, 1);
    sub_region_map[r].push(Some(new_region_id));
    new_region.sub_regions.push(new_region_id);
    regions.push(new_region);
    sub_region_to_region_map.push(new_region_id);
}

fn update_side_counts_of_both_adjacent_lines(
    line1_iter: impl Iterator<Item = Option<usize>> + Clone,
    line2_iter: impl Iterator<Item = Option<usize>> + Clone,
    regions: &mut [Region],
) {
    update_side_counts_of_regions_in_target_line_compared_to_adjacent_line(
        line1_iter.clone(),
        line2_iter.clone(),
        regions,
    );
    update_side_counts_of_regions_in_target_line_compared_to_adjacent_line(
        line2_iter, line1_iter, regions,
    );
}

fn update_side_counts_of_regions_in_target_line_compared_to_adjacent_line(
    target_region_line: impl Iterator<Item = Option<usize>>,
    adjacent_region_line: impl Iterator<Item = Option<usize>>,
    regions: &mut [Region],
) {
    let mut last_rgn: Option<usize> = None;
    target_region_line
        .zip(adjacent_region_line)
        .for_each(|(target_rgn, adjacent_rgn)| {
            let curr_rgn = match (target_rgn, adjacent_rgn) {
                (Some(rgn1), Some(rgn2)) if rgn1 == rgn2 => None,
                (Some(rgn1), Some(_)) => Some(rgn1),
                (Some(rgn1), None) => Some(rgn1),
                (None, _) => None,
            };
            match (curr_rgn, last_rgn) {
                (Some(cr), Some(lr)) if cr == lr => {}
                (Some(cr), _) => {
                    regions[cr].number_of_sides += 1;
                }
                _ => {}
            }
            last_rgn = curr_rgn;
        });
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
        let a_price = 4 * 4;
        let b_price = 4 * 4;
        let c_price = 4 * 8;
        let d_price = 1 * 4;
        let e_price = 3 * 4;
        let fence_price = a_price + b_price + c_price + d_price + e_price;
        assert_eq!(solve(contents), fence_price);
    }
}
