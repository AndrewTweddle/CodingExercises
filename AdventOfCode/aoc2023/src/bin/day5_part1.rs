use aoc2023::read_and_solve_and_time_more_runs;
use std::str::FromStr;

fn main() {
    read_and_solve_and_time_more_runs("data/day5_input.txt", "Day 5 part 1", solve, 10_000);
}

struct MappingRule {
    dest_start_id: u64,
    src_start_id: u64,
    count: u64,
}

impl MappingRule {
    fn map(&self, input: u64) -> Option<u64> {
        if self.src_start_id <= input && self.src_start_id + self.count >= input {
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
    fn map(&self, input: u64) -> u64 {
        self.mapping_rules
            .iter()
            .filter_map(|rule| rule.map(input))
            .next()
            .unwrap_or(input)
    }
}

fn solve(contents: &str) -> u64 {
    let mut mappings = Vec::new();

    let mut line_iter = contents.lines();
    let seed_line = line_iter.next().unwrap();

    let seeds: Vec<u64> = seed_line[7..]
        .split(' ')
        .map(|seed_str| u64::from_str(seed_str).expect("A seed was not a number"))
        .collect();

    // Build up the mappings...
    for ln in line_iter {
        if ln.is_empty() {
            continue;
        } else if ln.contains("map:") {
            mappings.push(Mapping {
                mapping_rules: Vec::new(),
            });
        } else {
            let rule_params: Vec<u64> = ln
                .split(' ')
                .map(|id_str| u64::from_str(id_str).expect("Id's should be numeric"))
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

    seeds
        .iter()
        .map(|seed| mappings.iter().fold(*seed, |id, mapping| mapping.map(id)))
        .min()
        .expect("No seed could be mapped to a location")
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn test_part1_example() {
        let contents = "seeds: 79 14 55 13

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
        let total = solve(contents);
        assert_eq!(total, 35);
    }
}
