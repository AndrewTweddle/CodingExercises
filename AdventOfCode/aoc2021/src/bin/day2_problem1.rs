use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "data/day2_input";
    let input_file = File::open(path).unwrap();
    let br = BufReader::new(input_file);
    let (final_x, final_y) = br.lines().fold((0_i32, 0_i32), |(x, y), ln| {
        let line = ln.unwrap();
        let mut line_fields_iter = line.split(' ');
        let direction = line_fields_iter.next().unwrap();
        let distance = line_fields_iter.next().unwrap().parse::<i32>().unwrap();
        match direction {
            "forward" => (x + distance, y),
            "up" => (x, y - distance),
            "down" => (x, y + distance),
            _ => panic!("Unrecognized instruction: {}", direction),
        }
    });
    println!("Final position: ({}, {})", final_x, final_y);
    println!("Product: {}", final_x * final_y);
}
