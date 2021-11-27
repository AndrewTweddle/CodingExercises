use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "data/day3_input";
    let input = File::open(path).unwrap();
    let br = BufReader::new(input);
    let rows = br
        .lines()
        .map(|ln| ln
            .unwrap()
            .as_bytes()
            .to_vec());

    let mut col: usize = 0;
    let mut tree_count = 0;
    for row in rows {
        if row[col % row.len()] == b'#' {
            tree_count += 1;
        }
        col += 3;
    }
    println!("# of trees: {}", tree_count);
}
