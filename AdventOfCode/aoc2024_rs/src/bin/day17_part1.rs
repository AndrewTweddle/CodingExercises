use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::str::Lines;

const INPUT_FILE_PATH: &str = "data/day17_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 17 part 1", solve, 1000);
}

fn solve(contents: &str) -> String {
    let mut line_iter = contents.lines();

    let a = parse_register_value(&mut line_iter);
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
    let mut computer = Computer::new(a, b, c, program);
    computer.run();
    computer
        .outputs
        .iter()
        .map(|byte| byte.to_string())
        .reduce(|str1, str2| str1 + "," + str2.as_str())
        .unwrap()
        .to_string()
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

struct Computer {
    a: Word,
    b: Word,
    c: Word,
    ip: Word,
    program: Vec<u8>,
    outputs: Vec<u8>,
}

impl Computer {
    fn new(a: Word, b: Word, c: Word, program: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            program,
            outputs: Vec::new(),
        }
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
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
                    self.outputs.push(output);
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
    fn test_example_1() {
        let mut computer = Computer::new(0, 0, 9, vec![2, 6]);
        computer.run();
        assert_eq!(computer.b, 1);
    }

    #[test]
    fn test_example_2() {
        let mut computer = Computer::new(10, 0, 0, vec![5, 0, 5, 1, 5, 4]);
        computer.run();
        assert_eq!(&computer.outputs, &[0, 1, 2]);
    }

    #[test]
    fn test_example_3() {
        let mut computer = Computer::new(2024, 0, 0, vec![0, 1, 5, 4, 3, 0]);
        computer.run();
        assert_eq!(&computer.outputs, &[4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.a, 0);
    }

    #[test]
    fn test_example_4() {
        let mut computer = Computer::new(0, 29, 0, vec![1, 7]);
        computer.run();
        assert_eq!(computer.b, 26);
    }

    #[test]
    fn test_example_5() {
        let mut computer = Computer::new(0, 2024, 43690, vec![4, 0]);
        computer.run();
        assert_eq!(computer.b, 44354);
    }

    #[test]
    fn test_full_example() {
        let contents = "Register A: 729\n\
                        Register B: 0\n\
                        Register C: 0\n\
                        \n\
                        Program: 0,1,5,4,3,0\n";
        let result = solve(contents);
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }
}
