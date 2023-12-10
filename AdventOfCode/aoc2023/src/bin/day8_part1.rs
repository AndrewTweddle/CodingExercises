use aoc2023::read_and_solve_and_time_more_runs;
use std::str::Lines;

fn main() {
    read_and_solve_and_time_more_runs("data/day8_input.txt", "Day 8 part 1", solve, 10_000);
}

#[repr(i32)]
#[derive(Copy, Clone)]
enum Instruction {
    Left = 0,
    Right = 1,
}

#[derive(Default, Copy, Clone)]
struct NavigationRule {
    left_index: i32,
    right_minus_left_index: i32,
}

fn solve(contents: &str) -> u32 {
    let mut line_iter = contents.lines();
    let instructions_str = line_iter
        .next()
        .expect("The line with instructions was not found");

    // Store the instructions in reverse order (for faster checking of whether to cycle around)
    let instructions = parse_instructions_in_reverse_order(instructions_str);

    // Skip blank line:
    line_iter.next();

    // Parse the nodes into a compact set of rules
    // Index 0 will correspond to "ZZZ", and the last index will correspond to
    let rules = parse_navigation_rules(&mut line_iter);

    // solve
    calculate_steps(&instructions, &rules)
}

fn calculate_steps(instructions: &[Instruction], rules: &[NavigationRule]) -> u32 {
    let index_count = rules.len();
    let instruction_count = instructions.len();

    let mut instruction_index = instruction_count;
    let mut rule_index = index_count - 1; // Start at "AAA"
    let mut step_count = 0;

    while rule_index != 0 {
        step_count += 1;

        // get next instruction, and reposition next instruction index
        instruction_index -= 1;
        let right_multiplier = instructions[instruction_index] as i32;
        if instruction_index == 0 {
            instruction_index = instruction_count;
        }

        // get the next rule, and determine the next index
        let rule = &rules[rule_index];

        rule_index = (rule.left_index + right_multiplier * rule.right_minus_left_index) as usize;
    }

    step_count
}

fn parse_instructions_in_reverse_order(instructions_str: &str) -> Vec<Instruction> {
    instructions_str
        .bytes()
        .rev()
        .map(|byte| match byte {
            b'L' => Instruction::Left,
            b'R' => Instruction::Right,
            _ => panic!("Unrecognized instructions: {}", byte as char),
        })
        .collect::<Vec<Instruction>>()
}

// Each triple of 3 letters will initially be treated as a 3 digit long base 26 number.
// This will be used as the id of the corresponding node.
const LETTER_TRIPLE_COUNT: usize = 26 * 26 * 26;

struct Node {
    id: i32,
    left_id: i32,
    right_id: i32,
}

fn letter_triple_to_id(letter_triple: &str) -> i32 {
    // We want ZZZ to be in position zero, for easy checking of whether we're done.
    // So treat Z as digit 0, Y as digit 1, ..., A as digit 25
    let bytes = letter_triple.as_bytes();
    assert_eq!(bytes.len(), 3);
    26 * 26 * (b'Z' - bytes[0]) as i32 + 26 * (b'Z' - bytes[1]) as i32 + (b'Z' - bytes[2]) as i32
}

#[derive(Default, Copy, Clone)]
struct NodeIdToIndexLookup {
    is_node: bool,
    index: i32,
}

fn parse_navigation_rules(line_iter: &mut Lines) -> Vec<NavigationRule> {
    // Parse nodes as a triple of id's
    let nodes: Vec<Node> = line_iter
        .map(|node_line| {
            let id = letter_triple_to_id(&node_line[0..3]);
            let left_id = letter_triple_to_id(&node_line[7..10]);
            let right_id = letter_triple_to_id(&node_line[12..15]);
            Node {
                id,
                left_id,
                right_id,
            }
        })
        .collect();

    // Work out which possible 3 letter codes are present (using their id's instead of strings).
    // We want to traverse a more compact structure, to avoid cache misses.

    // Instead of a hash table, just create a large array to track which 3-letter codes are present
    let mut id_to_index_lkp = [NodeIdToIndexLookup::default(); LETTER_TRIPLE_COUNT];

    // Work out which 3-letter codes are present
    for node in &nodes {
        id_to_index_lkp[node.id as usize].is_node = true;
    }

    // Map the id's of the 3 letter codes to indexes in a more compact structure
    for (index, lkp) in id_to_index_lkp
        .iter_mut()
        .filter(|lkp| lkp.is_node)
        .enumerate()
    {
        lkp.index = index as i32;
    }

    // The last index assigned is to "AAA"
    let index_count = id_to_index_lkp[LETTER_TRIPLE_COUNT - 1].index as usize + 1;

    // So create an array of navigation rules, where the index of each rule
    // corresponds to the index of the nodes.
    let mut rules: Vec<NavigationRule> = vec![NavigationRule::default(); index_count];
    for node in &nodes {
        let index = id_to_index_lkp[node.id as usize].index as usize;
        let left_index = id_to_index_lkp[node.left_id as usize].index;
        let right_index = id_to_index_lkp[node.right_id as usize].index;
        rules[index].left_index = left_index;
        rules[index].right_minus_left_index = right_index - left_index;
    }
    rules
}

#[cfg(test)]
mod tests {
    use super::solve;

    const EXAMPLE1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part1_example1() {
        let solution = solve(EXAMPLE1);
        assert_eq!(solution, 2);
    }

    #[test]
    fn test_part2_example1() {
        let solution = solve(EXAMPLE2);
        assert_eq!(solution, 6);
    }
}
