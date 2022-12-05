#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Instruction {
    crate_count: usize,
    src_stack: usize,
    dst_stack: usize,
}

type Stack = Vec<char>;
type Stacks = Vec<Stack>;
type CrateMover = fn(&mut Stacks, &Instruction) -> ();

fn main() {
    let contents = std::fs::read_to_string("data/day5_input.txt").unwrap();
    let (stacks_str, instructions_str) = contents.split_once("\n\n").unwrap();
    let instructions: Vec<Instruction> = instructions_str.lines().map(parse_instruction).collect();

    let mut stacks1 = build_stacks(stacks_str);
    let mut stacks2 = stacks1.clone();

    let part1 = solve(
        &instructions,
        &mut stacks1,
        process_crate_mover_9000_instruction,
    );

    let part2 = solve(
        &instructions,
        &mut stacks2,
        process_crate_mover_9001_instruction,
    );

    println!("2022 day 5 part 1 answer: {}", part1);
    println!("2022 day 5 part 2 answer: {}", part2);
}

fn build_stacks(stacks_str: &str) -> Stacks {
    let mut stack_lines: Vec<&str> = stacks_str.lines().collect();
    let last_stack_line = stack_lines.pop().unwrap();
    let stack_count = last_stack_line.split_ascii_whitespace().count();
    let max_stack_height = stack_lines.len();
    let mut stacks: Vec<Vec<char>> = vec![Vec::with_capacity(max_stack_height); stack_count];
    for stack_line in stack_lines.iter().rev() {
        let stack_line_bytes = stack_line.as_bytes();
        for (i, stack) in stacks.iter_mut().enumerate() {
            let byte_index = 1 + 4 * i;
            if byte_index >= stack_line_bytes.len() {
                break;
            }
            let crate_byte = stack_line_bytes[byte_index];
            if crate_byte != b' ' {
                stack.push(crate_byte as char);
            }
        }
    }
    stacks
}

fn solve(
    instructions: &[Instruction],
    stacks: &mut Stacks,
    process_instruction: CrateMover,
) -> String {
    for instruction in instructions {
        process_instruction(stacks, instruction);
    }
    // Get the last crate on each stack (assuming there is always at least one per stack)...
    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}

fn parse_instruction(instruction_str: &str) -> Instruction {
    let words: Vec<&str> = instruction_str.split_ascii_whitespace().collect();
    let crate_count = words[1].parse::<usize>().unwrap();
    let src_stack = words[3].parse::<usize>().unwrap();
    let dst_stack = words[5].parse::<usize>().unwrap();
    Instruction {
        crate_count,
        src_stack,
        dst_stack,
    }
}

fn process_crate_mover_9000_instruction(stacks: &mut Stacks, instruction: &Instruction) {
    for _ in 0..instruction.crate_count {
        let crate_to_move = stacks[instruction.src_stack - 1].pop().unwrap();
        stacks[instruction.dst_stack - 1].push(crate_to_move);
    }
}

fn process_crate_mover_9001_instruction(stacks: &mut Stacks, instruction: &Instruction) {
    let mut intermediate_stack = Stack::with_capacity(instruction.crate_count);
    for _ in 0..instruction.crate_count {
        let crate_to_move = stacks[instruction.src_stack - 1].pop().unwrap();
        intermediate_stack.push(crate_to_move);
    }
    while let Some(crate_to_move) = intermediate_stack.pop() {
        stacks[instruction.dst_stack - 1].push(crate_to_move);
    }
}

#[cfg(test)]
mod tests {
    use crate::{build_stacks, parse_instruction, Instruction};

    const FIRST_INSTRUCTION: &str = "move 1 from 2 to 1";
    const SECOND_INSTRUCTION: &str = "move 3 from 1 to 3";

    const INITIAL_STACKS: &str = "    [D]    \n\
                                  [N] [C]    \n\
                                  [Z] [M] [P]\n\
                                   1   2   3 ";

    const STACKS_AFTER_FIRST_INSTRUCTION: &str = "[D]\n\
                                                  [N] [C]\n\
                                                  [Z] [M] [P]\n\
                                                   1   2   3 ";

    #[test]
    fn test_parsing_stacks() {
        let stacks = build_stacks(INITIAL_STACKS);
        let expected_stacks = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(stacks, expected_stacks);
    }

    #[test]
    fn test_parsing_instruction() {
        let instruction = parse_instruction("move 3 from 1 to 2");
        let expected_instruction = Instruction {
            crate_count: 3,
            src_stack: 1,
            dst_stack: 2,
        };
        assert_eq!(instruction, expected_instruction);
    }

    mod part1_tests {
        use super::{
            build_stacks, parse_instruction, FIRST_INSTRUCTION, INITIAL_STACKS, SECOND_INSTRUCTION,
            STACKS_AFTER_FIRST_INSTRUCTION,
        };
        use crate::process_crate_mover_9000_instruction;

        const STACKS_AFTER_SECOND_INSTRUCTION: &str = "        [Z]\n
        [N]\n
    [C] [D]\n
    [M] [P]\n
 1   2   3";

        #[test]
        fn test_single_crate_instruction() {
            let mut stacks = build_stacks(INITIAL_STACKS);
            let instruction = parse_instruction(FIRST_INSTRUCTION);
            process_crate_mover_9000_instruction(&mut stacks, &instruction);
            let expected_stacks = build_stacks(STACKS_AFTER_FIRST_INSTRUCTION);
            assert_eq!(stacks, expected_stacks);
        }

        #[test]
        fn test_multiple_crate_instruction() {
            let mut stacks = build_stacks(STACKS_AFTER_FIRST_INSTRUCTION);
            let instruction = parse_instruction(SECOND_INSTRUCTION);
            process_crate_mover_9000_instruction(&mut stacks, &instruction);
            let expected_stacks = build_stacks(STACKS_AFTER_SECOND_INSTRUCTION);
            assert_eq!(stacks, expected_stacks);
        }
    }

    mod part2_tests {
        use super::{
            build_stacks, parse_instruction, FIRST_INSTRUCTION, INITIAL_STACKS, SECOND_INSTRUCTION,
            STACKS_AFTER_FIRST_INSTRUCTION,
        };
        use crate::process_crate_mover_9001_instruction;

        const STACKS_AFTER_SECOND_INSTRUCTION: &str = "        [D]\n
        [N]\n
    [C] [Z]\n
    [M] [P]\n
 1   2   3";

        #[test]
        fn test_single_crate_instruction() {
            let mut stacks = build_stacks(INITIAL_STACKS);
            let instruction = parse_instruction(FIRST_INSTRUCTION);
            process_crate_mover_9001_instruction(&mut stacks, &instruction);
            let expected_stacks = build_stacks(STACKS_AFTER_FIRST_INSTRUCTION);
            assert_eq!(stacks, expected_stacks);
        }

        #[test]
        fn test_multiple_crate_instruction() {
            let mut stacks = build_stacks(STACKS_AFTER_FIRST_INSTRUCTION);
            let instruction = parse_instruction(SECOND_INSTRUCTION);
            process_crate_mover_9001_instruction(&mut stacks, &instruction);
            let expected_stacks = build_stacks(STACKS_AFTER_SECOND_INSTRUCTION);
            assert_eq!(stacks, expected_stacks);
        }
    }
}
