use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type ElementPair = (u8, u8);
type ElementCounts = [usize; 26];
type PairInsertionRules = HashMap<ElementPair, u8>;

fn expand_elems_and_update_counts(
    left: u8,
    right: u8,
    expansion_count: usize,
    rules: &PairInsertionRules,
    counts: &mut ElementCounts,
) {
    let maybe_output = rules.get(&(left, right));
    if let Some(&output) = maybe_output {
        counts[output as usize] += 1;
        if expansion_count > 1 {
            expand_elems_and_update_counts(left, output, expansion_count - 1, rules, counts);
            expand_elems_and_update_counts(output, right, expansion_count - 1, rules, counts);
        }
    }
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day14_input.txt").unwrap();
    let mut line_iter = contents.lines();
    let template: Vec<u8> = line_iter
        .next()
        .unwrap()
        .chars()
        .map(|ch| ch as u8 - b'A')
        .collect();
    line_iter.next();
    let rules: PairInsertionRules = line_iter
        .map(|line| {
            let (input_str, output_str) = line.split_once(" -> ").unwrap();
            let inputs: Vec<_> = input_str.chars().map(|ch| ch as u8 - b'A').collect();
            let output = output_str.chars().next().unwrap() as u8 - b'A';
            ((inputs[0], inputs[1]), output)
        })
        .collect();
    let mut counts: ElementCounts = [0_usize; 26];
    counts[template[0] as usize] += 1;
    template.as_slice().windows(2).for_each(|elems| {
        let left = elems[0];
        let right = elems[1];
        counts[right as usize] += 1;
        expand_elems_and_update_counts(left, right, 10, &rules, &mut counts);
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
    let duration = start_time.elapsed();
    println!(
        "Difference of most and least common counts is {}",
        difference
    );
    println!("Duration: {:?}", duration);   // 621.835µs
}
