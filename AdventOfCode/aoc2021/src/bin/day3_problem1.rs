use std::fs::File;
use std::io::{BufRead, BufReader};

const BIT_COUNT: usize = 12;

fn main() {
    let input_file = File::open("data/day3_input").unwrap();
    let br = BufReader::new(input_file);
    let bytes: Vec<u32> = br
        .lines()
        .map(|ln| u32::from_str_radix(ln.unwrap().as_str(), 2).unwrap())
        .collect();
    let cut_off = bytes.len() / 2;
    let mut gamma_rate: u32 = 0;
    let mut xor_mask = 0;
    for byte_pos in 0..BIT_COUNT {
        let mask = 1_u32 << byte_pos;
        xor_mask |= mask;
        let one_count = bytes.iter().filter(|&byte| byte & mask != 0).count();
        if one_count > cut_off {
            gamma_rate |= mask;
        }
    }
    let epsilon_rate = gamma_rate ^ xor_mask;
    println!("gamma rate x epsilon rate = {}", gamma_rate * epsilon_rate);
}
