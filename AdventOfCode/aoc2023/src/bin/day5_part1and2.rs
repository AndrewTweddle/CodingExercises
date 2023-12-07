use aoc2023::read_and_solve_and_time_more_runs;
use std::str::{FromStr, Lines};

fn main() {
    read_and_solve_and_time_more_runs(
        "data/day5_input.txt",
        "Day 5 part 1",
        solve_part_one,
        10_000,
    );
    read_and_solve_and_time_more_runs(
        "data/day5_input.txt",
        "Day 5 part 2",
        solve_part_two,
        0,
    );
}

fn solve_part_one(contents: &str) -> Id {
    let mut line_iter = contents.lines();
    let seeds = get_numbers_in_seed_line(&mut line_iter);
    let mappings = get_mappings(&mut line_iter);
    get_min_location_id(seeds.iter().copied(), &mappings)
}

fn solve_part_two(contents: &str) -> Id {
    let mut line_iter = contents.lines();
    let seed_line_numbers = get_numbers_in_seed_line(&mut line_iter);
    let mappings = get_mappings(&mut line_iter);

    // Form ranges from pairs of numbers in the seed line:
    let mut seed_ranges = Vec::new();
    let mut range_start: Id = 0;

    for (i, &seed) in seed_line_numbers.iter().enumerate() {
        if i % 2 == 0 {
            range_start = seed;
        } else {
            seed_ranges.push(range_start..(range_start + seed));
        }
    }

    seed_ranges
        .into_iter()
        .map(|range| get_min_location_id(range.into_iter(), &mappings))
        .min()
        .expect("A minimum location id could be found from the seed ranges")
}

// ------------------------------------------------------------------------------------------------
// Shared logic...
// ------------------------------------------------------------------------------------------------

fn get_numbers_in_seed_line(line_iter: &mut Lines) -> Vec<Id> {
    let seed_line = line_iter.next().unwrap();
    seed_line[7..]
        .split(' ')
        .map(|seed_str| Id::from_str(seed_str).expect("A seed was not a number"))
        .collect::<Vec<Id>>()
}

type Id = u64;

struct MappingRule {
    dest_start_id: Id,
    src_start_id: Id,
    count: Id,
}

impl MappingRule {
    fn map(&self, input: Id) -> Option<Id> {
        if self.src_start_id <= input && self.src_start_id + self.count > input {
            Some(input + self.dest_start_id - self.src_start_id)
        } else {
            None
        }
    }
}

struct Mapping {
    mapping_rules: Vec<MappingRule>,
}

impl Mapping {
    fn map(&self, input: Id) -> Id {
        self.mapping_rules
            .iter()
            .filter_map(|rule| rule.map(input))
            .next()
            .unwrap_or(input)
    }
}

fn get_mappings(line_iter: &mut Lines) -> Vec<Mapping> {
    let mut mappings = Vec::new();
    for ln in line_iter {
        if ln.is_empty() {
            continue;
        } else if ln.contains("map:") {
            mappings.push(Mapping {
                mapping_rules: Vec::new(),
            });
        } else {
            let rule_params: Vec<Id> = ln
                .split(' ')
                .map(|id_str| Id::from_str(id_str).expect("Id's should be numeric"))
                .collect();

            assert_eq!(
                rule_params.len(),
                3,
                "Exactly 3 parameters are expected to each mapping"
            );

            mappings
                .last_mut()
                .expect("A current mapping was not available")
                .mapping_rules
                .push(MappingRule {
                    dest_start_id: rule_params[0],
                    src_start_id: rule_params[1],
                    count: rule_params[2],
                });
        }
    }
    mappings
}

fn get_min_location_id(seed_iter: impl Iterator<Item = Id>, mappings: &[Mapping]) -> Id {
    seed_iter
        .map(|seed| mappings.iter().fold(seed, |id, mapping| mapping.map(id)))
        .min()
        .expect("No seed could be mapped to a location")
}

#[cfg(test)]
mod tests {
    use super::{solve_part_one, solve_part_two};

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part1_example() {
        let total = solve_part_one(EXAMPLE);
        assert_eq!(total, 35);
    }

    #[test]
    fn test_part2_example() {
        let total = solve_part_two(EXAMPLE);
        assert_eq!(total, 46);
    }
}
