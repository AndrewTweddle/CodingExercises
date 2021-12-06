use std::cmp::max;
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
    let lines: Vec<Line> = get_lines();

    let orthogonal_line_iter = lines
        .iter()
        .filter(|&line| line.from.x == line.to.x || line.from.y == line.to.y);
    let part1_overlap_count = get_overlap_count(orthogonal_line_iter);

    let line_iter = lines.iter();
    let part2_overlap_count = get_overlap_count(line_iter);

    println!("Part 1 - overlap count: {}", part1_overlap_count);
    println!("Part 2 - overlap count: {}", part2_overlap_count);
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

fn get_overlap_count<'a, I>(line_iter: I) -> usize
where
    I: Iterator<Item = &'a Line> + Clone,
{
    let max_x = line_iter.clone().map(|line| line.to.x).max().unwrap();
    let max_y = line_iter
        .clone()
        .map(|line| max(line.from.y, line.to.y))
        .max()
        .unwrap() as usize;
    let mut grid = vec![vec![0_usize; max_y + 1]; max_x + 1];
    let mut overlap_count = 0;
    for line in line_iter.clone() {
        let range_x = (line.to.x - line.from.x) as isize;
        let step_x = if line.from.x == line.to.x { 0 } else { 1 };
        let (range_y, step_y): (isize, isize) = if line.to.y >= line.from.y {
            (
                (line.to.y - line.from.y) as isize,
                if line.to.y == line.from.y { 0 } else { 1 },
            )
        } else {
            ((line.from.y - line.to.y) as isize, -1)
        };
        let range = max(range_x, range_y);

        for i in 0..=range {
            let x = line.from.x as isize + i * step_x;
            let y = line.from.y as isize + i * step_y;
            let cell = &mut grid[x as usize][y as usize];
            *cell += 1;
            if *cell == 2 {
                overlap_count += 1;
            }
        }
    }
    overlap_count
}
