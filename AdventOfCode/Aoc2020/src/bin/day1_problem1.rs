use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "data/day1_input";
    let input = File::open(path).unwrap();
    let br = BufReader::new(input);

    let mut num_lkp = HashSet::<u32>::new();
    let mut pair = None;
    let numbers = br.lines().map(|ln| ln.unwrap().parse::<u32>().unwrap());
    for n in numbers {
        let other_term = 2020 - n;
        if num_lkp.contains(&other_term) {
            pair = Some((n, other_term));
            break;
        }
        num_lkp.insert(n);
    }
    let pair = pair.expect("No pair adding up to 2020 was found");
    println!("Product of {} and {} = {}", pair.0, pair.1, pair.0 * pair.1);
}
