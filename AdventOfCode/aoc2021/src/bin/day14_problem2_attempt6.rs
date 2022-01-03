use nalgebra::{DMatrix, DVector};
use std::fs;
use std::time::Instant;

// Map 'A'..'Z' to id's 1 to 26
const ELEMENT_COUNT: usize = 26;

// Map pairs of elements: first elements are rows, second elements are columns
const PAIR_COUNT: usize = 26 * 26;

fn char_to_id(ch: char) -> usize {
    ch as usize - b'A' as usize
}
fn pair_to_index(left: usize, right: usize) -> usize {
    26 * left + right
}

type ElementToPairMatrix = DMatrix<usize>; // Dimensions: ELEMENT_COUNT x PAIR_COUNT
type TemplateVector = DVector<usize>; // Dimension: PAIR_COUNT
type RuleMatrix = DMatrix<usize>; // Dimensions: PAIR_COUNT x PAIR_COUNT

struct Inputs {
    first_element: usize,
    template_vector: TemplateVector,
    rules_matrix: RuleMatrix,
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day14_input.txt").unwrap();
    let inputs = parse_file_contents(contents);
    let difference = get_max_min_count_difference(40, &inputs);
    let duration = start_time.elapsed();
    println!(
        "Part 2: biggest difference after 40 steps is {}",
        difference
    );
    println!("Duration: {:?}", duration); // Duration: 1.25090696s

    // Solve part 1 again:
    let difference = get_max_min_count_difference(10, &inputs);
    println!(
        "Part 1: biggest difference after 10 steps is {}",
        difference
    );
}

fn parse_file_contents(contents: String) -> Inputs {
    let (template_str, rules_str) = contents.split_once("\n\n").unwrap();

    let template: Vec<_> = template_str.chars().map(char_to_id).collect();
    let mut template_vector = TemplateVector::from_element(PAIR_COUNT, 0);
    template.as_slice().windows(2).for_each(|elems| {
        let left = elems[0];
        let right = elems[1];
        let pair_index = pair_to_index(left, right);
        template_vector[pair_index] += 1;
    });

    // By default each pair of elements maps to itself if there is no rule for it...
    let mut rules_matrix = DMatrix::<usize>::identity(PAIR_COUNT, PAIR_COUNT);

    for line in rules_str.lines() {
        let (input_str, output_str) = line.split_once(" -> ").unwrap();
        let inputs: Vec<_> = input_str.chars().map(char_to_id).collect();
        let output = char_to_id(output_str.chars().next().unwrap());
        let left = inputs[0];
        let right = inputs[1];
        let id = pair_to_index(left, right);
        // Since there is a rule for this pair of elements, remove the identity mapping.
        // Do this first in case the pair generates an element that matches either original element.
        rules_matrix[(id, id)] = 0;
        rules_matrix[(pair_to_index(left, output), id)] += 1;
        rules_matrix[(pair_to_index(output, right), id)] += 1;
    }

    Inputs {
        first_element: template[0],
        template_vector,
        rules_matrix,
    }
}

fn get_max_min_count_difference(steps: u32, inputs: &Inputs) -> usize {
    let mut right_element_to_pair_matrix =
        ElementToPairMatrix::from_element(ELEMENT_COUNT, PAIR_COUNT, 0);
    for pair_index in 1..PAIR_COUNT {
        right_element_to_pair_matrix[(pair_index % 26, pair_index)] = 1;
    }

    let steps_matrix = inputs.rules_matrix.pow(steps);
    let pair_counts_vector = steps_matrix * &inputs.template_vector;
    let mut element_counts_vector = right_element_to_pair_matrix * pair_counts_vector;

    // Don't forget the leftmost character...
    element_counts_vector[inputs.first_element] += 1;

    let mut min_count = usize::MAX;
    let mut max_count = 0;
    for &count in &element_counts_vector {
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
