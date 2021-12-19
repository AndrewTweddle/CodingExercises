use std::fs;
use std::ops::{Index, IndexMut, Mul, MulAssign};
use std::time::Instant;

const ELEMENT_COUNT: usize = 26;
const PAIR_COUNT: usize = ELEMENT_COUNT * ELEMENT_COUNT;
const MATRIX_ENTRY_COUNT: usize = PAIR_COUNT * PAIR_COUNT;

fn char_to_id(ch: char) -> usize {
    ch as usize - b'A' as usize
}
fn pair_to_index(left: usize, right: usize) -> usize {
    26 * left + right
}

#[derive(Clone)]
struct RuleMatrix {
    elements: Vec<usize>,
}

impl RuleMatrix {
    fn get_index(row: usize, col: usize) -> usize {
        row * PAIR_COUNT + col
    }

    fn identity() -> Self {
        let elements = vec![0_usize; MATRIX_ENTRY_COUNT];
        let mut matrix = RuleMatrix { elements };
        for i in 0..PAIR_COUNT {
            matrix[(i, i)] = 1;
        }
        matrix
    }

    fn pow(self, exponent: usize) -> RuleMatrix {
        match exponent {
            0 => RuleMatrix::identity(),
            1 => self.clone(),
            exp => {
                let mut matrix = self.clone().pow(exp / 2);
                matrix = matrix.clone() * &matrix;
                if exp % 2 == 0 {
                    matrix
                } else {
                    self.clone() * &matrix
                }
            }
        }
    }
}

impl Index<(usize, usize)> for RuleMatrix {
    type Output = usize;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let vec_index = RuleMatrix::get_index(index.0, index.1);
        &self.elements[vec_index]
    }
}

impl IndexMut<(usize, usize)> for RuleMatrix {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let vec_index = RuleMatrix::get_index(index.0, index.1);
        &mut self.elements[vec_index]
    }
}

impl Mul<&Self> for RuleMatrix {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        let mut elements = vec![0_usize; PAIR_COUNT * PAIR_COUNT];
        for r in 0..PAIR_COUNT {
            for c in 0..PAIR_COUNT {
                // Calculate entry (r, c)
                let mut entry_value: usize = 0;
                for i in 0..PAIR_COUNT {
                    entry_value += self[(r, i)] * rhs[(i, c)];
                }
                let index = RuleMatrix::get_index(r, c);
                elements[index] = entry_value;
            }
        }
        Self { elements }
    }
}

impl MulAssign<&Self> for RuleMatrix {
    fn mul_assign(&mut self, rhs: &Self) {
        let new_elements = vec![0_usize; PAIR_COUNT * PAIR_COUNT];
        for r in 0..PAIR_COUNT {
            for c in 0..PAIR_COUNT {
                // Calculate entry (r, c)
                let mut entry_value: usize = 0;
                for i in 0..PAIR_COUNT {
                    entry_value += self[(r, i)] * rhs[(i, c)];
                }
                self[(r, c)] = entry_value;
            }
        }
        self.elements = new_elements;
    }
}

impl Mul<&Vec<usize>> for RuleMatrix {
    type Output = Vec<usize>;

    fn mul(self, rhs: &Vec<usize>) -> Self::Output {
        let mut output = vec![0_usize; PAIR_COUNT];
        for row in 0..PAIR_COUNT {
            output[row] = (0..PAIR_COUNT).map(|col| self[(row, col)] * rhs[col]).sum();
        }
        output
    }
}

struct Inputs {
    first_element: usize,
    template_vector: Vec<usize>,
    rules_matrix: RuleMatrix,
}

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day14_input.txt").unwrap();
    let inputs = parse_file_contents(contents);
    let difference = get_max_min_count_difference(40, &inputs);
    let duration = start_time.elapsed();
    println!("Part 2 difference: {}", difference);
    println!("Duration: {:?}", duration);
}

fn parse_file_contents(contents: String) -> Inputs {
    let (template_str, rules_str) = contents.split_once("\n\n").unwrap();

    let template: Vec<_> = template_str.chars().map(char_to_id).collect();
    let mut template_vector = vec![0_usize; PAIR_COUNT];
    template.as_slice().windows(2).for_each(|elems| {
        let left = elems[0];
        let right = elems[1];
        let pair_index = pair_to_index(left, right);
        template_vector[pair_index] += 1;
    });

    // By default each pair of elements maps to itself if there is no rule for it...
    let mut rules_matrix = RuleMatrix::identity();

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

fn get_max_min_count_difference(steps: usize, inputs: &Inputs) -> usize {
    let steps_matrix = inputs.rules_matrix.clone().pow(steps);
    let pair_counts_vector = steps_matrix * &inputs.template_vector;

    let mut element_counts_vector = vec![0_usize; ELEMENT_COUNT];
    for pair_index in 1..PAIR_COUNT {
        element_counts_vector[pair_index % ELEMENT_COUNT] += pair_counts_vector[pair_index];
    }

    // Don't forget the leftmost character...
    element_counts_vector[inputs.first_element] += 1;

    let mut min_count = usize::MAX;
    let mut max_count = 0_usize;
    for &count in &element_counts_vector {
        if count > max_count {
            max_count = count;
        }
        if count > 0 && count < min_count {
            min_count = count
        }
    }
    max_count - min_count
}
