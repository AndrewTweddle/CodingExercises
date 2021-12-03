use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "data/day2_input";
    let input_file = File::open(path).unwrap();
    let br = BufReader::new(input_file);
    let (final_x, final_y, final_aim) = br
        .lines()
        .fold((0_i64, 0_i64, 0_i64), |(x, y, aim), ln| {
            let line = ln.unwrap();
            let mut split_line = line.split(' ');
            let direction = split_line.next().unwrap();
            let distance = split_line.next().unwrap().parse::<i64>().unwrap();
            match direction {
                "forward" => (x + distance, y + distance * aim, aim),
                "up" => (x, y, aim - distance),
                "down" => (x, y, aim + distance),
                _ => panic!("Unrecognized instruction: {}", direction),
            }
        });
    println!("Final position: ({}, {})", final_x, final_y);
    println!("Final aim: {}", final_aim);
    println!("Product: {}", final_x * final_y);
}