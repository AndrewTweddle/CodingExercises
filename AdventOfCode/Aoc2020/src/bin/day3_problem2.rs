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
            .into_bytes())
        .collect::<Vec<Vec<u8>>>();

    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ];
    let tree_count_product = slopes
        .iter()
        .map(|&(right, down)| count_trees_by_slope(rows.iter(), right, down))
        .product::<usize>();

    println!("Product of # of trees for each slope: {}", tree_count_product);
}

fn count_trees_by_slope<'a, I>(rows: I, right: usize, down: usize) -> usize
    where I: Iterator<Item=&'a Vec<u8>>
{
    let mut col: usize = 0;
    let mut tree_count = 0;
    for row in rows.step_by(down) {
        if row[col % row.len()] == b'#' {
            tree_count += 1;
        }
        col += right;
    }
    tree_count
}
