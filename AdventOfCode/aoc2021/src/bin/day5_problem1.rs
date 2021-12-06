use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Copy, Clone)]
struct Line {
    from: Point,
    to: Point,
}

fn main() {
    let lines: Vec<Line> = get_lines()
        .iter()
        .filter(|line| line.from.x == line.to.x || line.from.y == line.to.y)
        .map(|&line| line)
        .collect();

    let max_x = lines.iter().map(|line| line.to.x).max().unwrap();
    let max_y = lines.iter().map(|line| line.to.y).max().unwrap();
    let mut grid = vec![vec![0_usize; max_y + 1]; max_x + 1];
    let mut overlap_count = 0;
    for line in &lines {
        for x in line.from.x..=line.to.x {
            for y in line.from.y..=line.to.y {
                let cell = &mut grid[x][y];
                *cell += 1;
                if *cell == 2 {
                    overlap_count += 1;
                }
            }
        }
    }

    println!("Overlap count: {}", overlap_count);
}

fn get_lines() -> Vec<Line> {
    let input_file = File::open("data/day5_input").unwrap();
    let br = BufReader::new(input_file);
    br.lines()
        .map(|ln| {
            let line = ln.unwrap();
            let mut parts_iter = line.split(" -> ").map(|part| {
                let (x_string, y_string) = part.split_once(",").unwrap();
                Point {
                    x: x_string.parse::<usize>().unwrap(),
                    y: y_string.parse::<usize>().unwrap(),
                }
            });
            let point1 = parts_iter.next().unwrap();
            let point2 = parts_iter.next().unwrap();

            // Sort the points so that x's increase, and if they are the same, y's increase...
            if point1.x > point2.x || (point1.x == point2.x && point1.y > point2.y) {
                Line {
                    from: point2,
                    to: point1,
                }
            } else {
                Line {
                    from: point1,
                    to: point2,
                }
            }
        })
        .collect::<Vec<Line>>()
}
