use std::collections::HashSet;
use std::fs;
use std::time::Instant;

#[derive(Debug)]
enum Axis {
    X,
    Y,
}

type Pos = (u64, u64);
type Instruction = (Axis, u64);

fn main() {
    let start_time = Instant::now();
    let contents = fs::read_to_string("data/day13_input.txt").unwrap();
    let (coordinates, instructions) = parse_inputs(&contents);
    let instruction = instructions.first().unwrap();

    let transformed_coordinates: HashSet<Pos> = coordinates
        .iter()
        .filter_map(|pos| process_instruction(pos, &instruction))
        .collect();

    let visible_dots = transformed_coordinates.len();
    let duration = start_time.elapsed();

    println!("Part 1: visible dots = {}", visible_dots);
    println!("Duration: {:?}", duration);
}

fn parse_inputs(contents: &str) -> (Vec<Pos>, Vec<Instruction>) {
    let blank_line_index = contents
        .lines()
        .enumerate()
        .find(|(_, line)| line.is_empty())
        .unwrap()
        .0;

    let coordinates: Vec<Pos> = contents
        .lines()
        .take(blank_line_index)
        .map(|line| {
            let (x_str, y_str) = line.split_once(',').unwrap();
            (x_str.parse::<u64>().unwrap(), y_str.parse::<u64>().unwrap())
        })
        .collect();

    let instructions: Vec<Instruction> = contents
        .lines()
        .skip(blank_line_index + 1)
        .map(|line| {
            let (axis_type_str, axis_value_str) = line
                .trim_start_matches("fold along ")
                .split_once('=')
                .unwrap();
            let axis = match axis_type_str {
                "x" => Axis::X,
                "y" => Axis::Y,
                _ => panic!("Unexpected axis in instruction: {}", axis_type_str),
            };
            let axis_value = axis_value_str.parse::<u64>().unwrap();
            (axis, axis_value)
        })
        .collect();

    (coordinates, instructions)
}

fn process_instruction(coordinates: &Pos, instruction: &Instruction) -> Option<Pos> {
    match instruction {
        (Axis::X, x_val) => match coordinates {
            &(x, _) if x < *x_val => Some(*coordinates),
            &(x, y) if x > *x_val => Some((2 * x_val - x, y)),
            _ => None, // On the line
        },
        (Axis::Y, y_val) => match coordinates {
            &(_, y) if y < *y_val => Some(*coordinates),
            &(x, y) if y > *y_val => Some((x, 2 * y_val - y)),
            _ => None, // On the line
        },
    }
}
