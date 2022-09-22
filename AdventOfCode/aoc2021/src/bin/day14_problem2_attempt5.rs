use std::collections::BTreeMap;
use std::fs;
use std::time::Instant;

const ELEMENT_COUNT: usize = 26; // Map 'A'..'Z' to id's 0 to 25

// Each pair is either a pair of identical elements giving rise to
// another element of the same type (i.e. the original pair is duplicated)
// or it expands into two new pairs (the new element pairs with each element of the original pair)
#[derive(Copy, Clone)]
enum Rule {
    DuplicatePair,
    TwoPairs(usize, usize),
}

type PairInsertionRules = BTreeMap<usize, Rule>;
type PairCounts = BTreeMap<usize, usize>;
type ElementCounts = [usize; ELEMENT_COUNT];

fn char_to_id(ch: char) -> usize {
    ch as usize - b'A' as usize
}

fn pair_to_index(left: usize, right: usize) -> usize {
    26 * left + right
}

struct Inputs {
    first_element: usize,
    pair_counts: PairCounts,
    rules: PairInsertionRules,
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day14_input.txt").unwrap();
    let inputs = parse_file_contents(contents);
    let difference = get_max_min_count_difference(40, &inputs);
    let duration = start_time.elapsed();
    println!("Part 2 difference: {}", difference);
    println!("Duration: {:?}", duration); // 445.868µs

    // Solve part 1 again...
    let difference = get_max_min_count_difference(10, &inputs);
    println!("Part 1 difference: {}", difference); // Duration: 133.241µs
}

fn parse_file_contents(contents: String) -> Inputs {
    let (template_str, rules_str) = contents.split_once("\n\n").unwrap();

    let template: Vec<_> = template_str.chars().map(char_to_id).collect();
    let mut pair_counts: PairCounts = BTreeMap::new();
    template.as_slice().windows(2).for_each(|elems| {
        let pair_index = pair_to_index(elems[0], elems[1]);
        *pair_counts.entry(pair_index).or_insert(0) += 1;
    });

    // By default, a pair is not expanded unless a rule has been provided...
    let rules: PairInsertionRules = rules_str
        .lines()
        .map(|line| {
            let (input_str, output_str) = line.split_once(" -> ").unwrap();
            let inputs: Vec<_> = input_str.chars().map(char_to_id).collect();
            let output = char_to_id(output_str.chars().next().unwrap());
            let left = inputs[0];
            let right = inputs[1];
            let id = pair_to_index(left, right);

            let rule: Rule = if (left == right) && (left == id) {
                Rule::DuplicatePair
            } else {
                let left_pair = pair_to_index(left, output);
                let right_pair = pair_to_index(output, right);
                Rule::TwoPairs(left_pair, right_pair)
            };
            (id, rule)
        })
        .collect();
    Inputs {
        first_element: template[0],
        pair_counts,
        rules,
    }
}

fn get_max_min_count_difference(steps: usize, inputs: &Inputs) -> usize {
    let mut pair_counts: PairCounts = inputs.pair_counts.clone();
    for _ in 0..steps {
        pair_counts = step(&pair_counts, &inputs.rules);
    }

    let mut counts: ElementCounts = [0_usize; ELEMENT_COUNT];

    // extract and count the right element of each pair in the final pair counts
    pair_counts
        .iter()
        .for_each(|(&pair_index, &count)| counts[pair_index % ELEMENT_COUNT] += count);

    // Don't forget to count the initial element in the final string
    counts[inputs.first_element] += 1;

    let mut min_count = usize::MAX;
    let mut max_count: usize = 0;
    for count in counts {
        if count > max_count {
            max_count = count;
        }
        if count > 0 && count < min_count {
            min_count = count
        }
    }
    max_count - min_count
}

fn step(pair_counts: &PairCounts, rules: &PairInsertionRules) -> PairCounts {
    let mut new_pair_counts: PairCounts = BTreeMap::new();
    for (&pair_index, &count) in pair_counts.iter() {
        if let Some(rule) = rules.get(&pair_index) {
            match rule {
                Rule::DuplicatePair => {
                    *new_pair_counts.entry(pair_index).or_insert(0) += 2 * count;
                }
                Rule::TwoPairs(pair_index1, pair_index2) => {
                    *new_pair_counts.entry(*pair_index1).or_insert(0) += count;
                    *new_pair_counts.entry(*pair_index2).or_insert(0) += count;
                }
            }
        } else {
            *new_pair_counts.entry(pair_index).or_insert(0) += count;
        }
    }
    new_pair_counts
}
