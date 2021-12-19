use std::fs;
use std::time::Instant;

const ELEMENT_COUNT: usize = 26; // Map 'A'..'Z' to id's 0 to 25
const PAIR_COUNT: usize = ELEMENT_COUNT * ELEMENT_COUNT;

// Each pair is either not expanded (no rule provided),
// or it is a pair of identical elements giving rise to another element of the same type
//    i.e. the original pair is duplicated
// or it expands into two new pairs (the new element pairs with each element of the original pair)
#[derive(Copy, Clone)]
enum Rule {
    NoExpansion,
    DuplicatePair,
    TwoPairs(usize, usize),
}

type PairInsertionRules = [Rule; PAIR_COUNT];
type PairCounts = [usize; PAIR_COUNT];
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
    println!("Duration: {:?}", duration); // 81.989µs

    // Solve part 1 again...
    let difference = get_max_min_count_difference(10, &inputs);
    println!("Part 1 difference: {}", difference); // Duration: 64.72µs
}

fn parse_file_contents(contents: String) -> Inputs {
    let (template_str, rules_str) = contents.split_once("\n\n").unwrap();

    let mut pair_counts = [0_usize; PAIR_COUNT];
    let template: Vec<_> = template_str.chars().map(char_to_id).collect();
    template.as_slice().windows(2).for_each(|elems| {
        let pair_index = pair_to_index(elems[0], elems[1]);
        pair_counts[pair_index] += 1;
    });

    // By default, a pair is not expanded unless a rule has been provided...
    let mut rules: PairInsertionRules = [Rule::NoExpansion; PAIR_COUNT];
    for line in rules_str.lines() {
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
        rules[id] = rule;
    }
    Inputs {
        first_element: template[0],
        pair_counts,
        rules,
    }
}

fn get_max_min_count_difference(steps: usize, inputs: &Inputs) -> usize {
    let mut even_pair_counts = inputs.pair_counts.clone();
    let mut odd_pair_counts: PairCounts = [0; PAIR_COUNT];

    for step in 1..=steps {
        let (src_counts, dest_counts): (&PairCounts, &mut PairCounts) = if step % 2 == 0 {
            (&odd_pair_counts, &mut even_pair_counts)
        } else {
            (&even_pair_counts, &mut odd_pair_counts)
        };
        dest_counts
            .iter_mut()
            .for_each(|dest_count| *dest_count = 0);
        for pair_index in 0..PAIR_COUNT {
            let src_count = src_counts[pair_index];
            match inputs.rules[pair_index] {
                Rule::NoExpansion => {
                    dest_counts[pair_index] += src_count;
                }
                Rule::DuplicatePair => {
                    dest_counts[pair_index] += 2 * src_count;
                }
                Rule::TwoPairs(pair1_index, pair2_index) => {
                    dest_counts[pair1_index] += src_count;
                    dest_counts[pair2_index] += src_count;
                }
            }
        }
    }

    let final_pair_counts: &PairCounts = if steps % 2 == 0 {
        &even_pair_counts
    } else {
        &odd_pair_counts
    };

    let mut counts: ElementCounts = [0_usize; ELEMENT_COUNT];
    // extract and count the right element of each pair in the final pair counts
    final_pair_counts
        .iter()
        .enumerate()
        .for_each(|(pair_index, &count)| counts[pair_index % ELEMENT_COUNT] += count);

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
    let difference = max_count - min_count;
    difference
}
