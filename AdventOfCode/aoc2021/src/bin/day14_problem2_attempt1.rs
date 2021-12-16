use std::fs;
use std::time::Instant;

type ElementCounts = [usize; 27]; // Map 'A'..'Z' to id's 1 to 26, reserving 0 for an invalid id

#[derive(Copy, Clone)]
struct RuleOutput {
    id: usize,
    left_pair_index: usize,
    right_pair_index: usize,
}
const RULE_ENTRY_COUNT: usize = 26 * 26 + 1;
type PairInsertionRules = [RuleOutput; RULE_ENTRY_COUNT]; // reserve 0 for the invalid index

fn char_to_id(ch: char) -> usize {
    ch as usize - b'A' as usize + 1
}
fn pair_to_index(left: usize, right: usize) -> usize {
    26 * (left - 1) + right
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day14_input.txt").unwrap();
    let (template, rules) = parse_file_contents(contents);

    let alg_start_time = Instant::now();
    let difference = get_max_min_count_difference(40, &template, &rules);
    let alg_duration = alg_start_time.elapsed();
    let duration = start_time.elapsed();
    println!(
        "Difference of most and least common counts is {}",
        difference
    );
    println!("Duration: {:?}", duration);
    println!("Algorithm duration: {:?}", alg_duration);
    // | Steps | Duration       | Alg. Duration  |
    // |-------|----------------|----------------|
    // | 10    | 121.623µs      |  77.812µs      |
    // | 20    |  74.166701ms   |  74.124786ms   |
    // | 30    |  71.968214229s |  71.968170869s |
    // Not worth trying with 40 steps. It will take about 20h30... back to the drawing board!
}

fn parse_file_contents(contents: String) -> (Vec<usize>, PairInsertionRules) {
    let mut line_iter = contents.lines();
    let template: Vec<_> = line_iter.next().unwrap().chars().map(char_to_id).collect();
    line_iter.next();
    let null_output = RuleOutput {
        id: 0,
        left_pair_index: 0,
        right_pair_index: 0,
    };
    let mut rules: PairInsertionRules = [null_output; RULE_ENTRY_COUNT];
    for line in line_iter {
        let (input_str, output_str) = line.split_once(" -> ").unwrap();
        let inputs: Vec<_> = input_str.chars().map(char_to_id).collect();
        let output = char_to_id(output_str.chars().next().unwrap());
        let left = inputs[0];
        let right = inputs[1];
        rules[pair_to_index(left, right)] = RuleOutput {
            id: output,
            left_pair_index: pair_to_index(left, output),
            right_pair_index: pair_to_index(output, right),
        };
    }
    (template, rules)
}

fn get_max_min_count_difference(
    steps: usize,
    template: &Vec<usize>,
    rules: &PairInsertionRules,
) -> usize {
    let mut counts: ElementCounts = [0_usize; 27];
    counts[template[0]] += 1;
    template.as_slice().windows(2).for_each(|elems| {
        let left = elems[0];
        let right = elems[1];
        counts[right as usize] += 1;
        let pair_index = pair_to_index(left, right);
        expand_elems_and_update_counts(pair_index, steps, &rules, &mut counts);
    });

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

fn expand_elems_and_update_counts(
    pair_index: usize,
    expansion_count: usize,
    rules: &PairInsertionRules,
    counts: &mut ElementCounts,
) {
    let output = &rules[pair_index];
    counts[output.id] += 1;
    if expansion_count > 1 {
        if output.left_pair_index != 0 {
            expand_elems_and_update_counts(
                output.left_pair_index,
                expansion_count - 1,
                rules,
                counts,
            );
        }
        if output.right_pair_index != 0 {
            expand_elems_and_update_counts(
                output.right_pair_index,
                expansion_count - 1,
                rules,
                counts,
            );
        }
    }
}
