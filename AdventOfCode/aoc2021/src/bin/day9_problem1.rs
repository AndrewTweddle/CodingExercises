use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input_file = File::open("data/day9_input").unwrap();
    let br = BufReader::new(input_file);
    let height_map: Vec<Vec<u8>> = br
        .lines()
        .map(|line_result| line_result
            .unwrap()
            .as_bytes()
            .iter()
            .map(|byte| byte - b'0')
            .collect::<Vec<u8>>())
        .collect();
    let mut sum_of_risk_levels: u32 = 0;
    let row_count = height_map.len();
    let col_count = if row_count == 0 { 0 } else { height_map[0].len() };
    for row in 0..row_count {
        for col in 0..col_count {
            let height = (&height_map[row])[col];
            let left = if col == 0 { 9 } else { (&height_map[row])[col - 1] };
            if height >= left { continue; }
            let right = if col == col_count - 1 { 9 } else { (&height_map[row])[col + 1] };
            if height >= right { continue; }
            let up = if row == 0 { 9 } else { (&height_map[row - 1])[col] };
            if height >= up { continue; }
            let down = if row == row_count - 1 { 9 } else { (&height_map[row + 1])[col] };
            if height < down {
                sum_of_risk_levels += height as u32 + 1;
            }
        }
    }
    println!("The combined risk level is {}", sum_of_risk_levels);
}