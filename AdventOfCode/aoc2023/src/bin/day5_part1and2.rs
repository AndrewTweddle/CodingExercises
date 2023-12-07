use aoc2023::read_and_solve_and_time_more_runs;
use std::str::FromStr;

enum Part {
    One,
    Two,
}

fn main() {
    read_and_solve_and_time_more_runs(
        "data/day5_input.txt",
        "Day 5 part 1",
        |contents| solve(contents, Part::One),
        10_000,
    );
    read_and_solve_and_time_more_runs(
        "data/day5_input.txt",
        "Day 5 part 2",
        |contents| solve(contents, Part::Two),
        0,
    );
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

fn solve(contents: &str, part: Part) -> Id {
    let mut mappings = Vec::new();

    let mut line_iter = contents.lines();
    let seed_line = line_iter.next().unwrap();

    // Build up the mappings...
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

    let seed_line_iter = seed_line[7..]
        .split(' ')
        .map(|seed_str| Id::from_str(seed_str).expect("A seed was not a number"));

    match part {
        Part::One => get_min_location_id(seed_line_iter, &mappings),
        Part::Two => {
            // Form ranges from pairs of numbers in the seed line:
            let mut seed_ranges = Vec::new();
            let mut range_start: Id = 0;

            for (i, seed) in seed_line_iter.enumerate() {
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
    }
}

fn get_min_location_id(seed_iter: impl Iterator<Item = Id>, mappings: &[Mapping]) -> Id {
    seed_iter
        .map(|seed| mappings.iter().fold(seed, |id, mapping| mapping.map(id)))
        .min()
        .expect("No seed could be mapped to a location")
}

#[cfg(test)]
mod tests {
    use super::{solve, Part};

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
        let total = solve(EXAMPLE, Part::One);
        assert_eq!(total, 35);
    }

    #[test]
    fn test_part2_example() {
        let total = solve(EXAMPLE, Part::Two);
        assert_eq!(total, 46);
    }
}
