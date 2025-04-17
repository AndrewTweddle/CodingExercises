use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::str::Lines;

const INPUT_FILE_PATH: &str = "data/day17_input.txt";

// Stop the program running indefinitely by giving up after a certain value...
// Warning: will run for a VERY long time unless the upper bound is set.
// It runs for about 8 seconds when set to 1_000_000_000
const MAX_A_TO_ATTEMPT: Word = 1_000_000_000;

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 17 part 2", solve, 0);
}

fn solve(contents: &str) -> Option<Word> {
    let mut line_iter = contents.lines();

    let _ = parse_register_value(&mut line_iter);
    let b = parse_register_value(&mut line_iter);
    let c = parse_register_value(&mut line_iter);
    line_iter.next();
    let program: Vec<u8> = line_iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|num_str| num_str.parse::<u8>().unwrap())
        .collect();
    let mut computer = Computer::new(0, b, c, &program);

    for a in 0..=MAX_A_TO_ATTEMPT {
        computer.a = a;
        computer.run();
        if computer.matched_output_count == computer.program.len() {
            return Some(a);
        }
        computer.reset();
        computer.b = b;
        computer.c = c;
    }
    None
}

fn parse_register_value(line_iter: &mut Lines) -> Word {
    line_iter
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .parse::<Word>()
        .unwrap()
}

type Word = usize;

struct Computer<'a> {
    a: Word,
    b: Word,
    c: Word,
    ip: Word,
    program: &'a [u8],
    matched_output_count: usize,
}

impl<'a> Computer<'a> {
    fn new(a: Word, b: Word, c: Word, program: &'a [u8]) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            program,
            matched_output_count: 0,
        }
    }
    
    #[inline(always)]
    fn reset(&mut self) {
        self.matched_output_count = 0;
        self.ip = 0;
    }

    fn run(&mut self) {
        while self.ip < self.program.len() - 1 {
            let cmd = self.program[self.ip];
            let operand = self.program[self.ip + 1];
            match cmd {
                0 => {
                    // adv combo
                    self.a >>= self.get_combo(operand);
                }
                1 => {
                    // bxl literal
                    self.b ^= operand as usize;
                }
                2 => {
                    // bst combo
                    self.b = self.get_combo(operand) % 8;
                }
                3 => {
                    // jnz literal
                    if self.a != 0 {
                        self.ip = operand as usize;
                        continue;
                    }
                }
                4 => {
                    // bxc ignore
                    self.b ^= self.c;
                }
                5 => {
                    // out
                    let output = (self.get_combo(operand) % 8) as u8;

                    // Terminate the program if the outputs can't match the program...
                    if self.matched_output_count == self.program.len() {
                        self.matched_output_count += 1;
                        return;
                    }
                    if self.program[self.matched_output_count] != output {
                        return;
                    }

                    // Still matching, so add the next output and continue
                    self.matched_output_count += 1;
                }
                6 => {
                    // bdv combo
                    self.b = self.a >> self.get_combo(operand);
                }
                7 => {
                    // bdv combo
                    self.c = self.a >> self.get_combo(operand);
                }
                _ => {
                    panic!("Unknown command: {cmd}");
                }
            }
            self.ip += 2;
        }
    }

    #[inline(always)]
    fn get_combo(&self, operand: u8) -> Word {
        match operand {
            0..4 => operand as Word,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid operand {operand}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_example() {
        let contents = "Register A: 2024\n\
                        Register B: 0\n\
                        Register C: 0\n\
                        \n\
                        Program: 0,3,5,4,3,0\n";
        let result = solve(contents);
        assert_eq!(result, Some(117440));
    }
}
