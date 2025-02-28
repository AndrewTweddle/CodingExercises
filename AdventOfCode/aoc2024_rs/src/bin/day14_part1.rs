use aoc2024_rs::read_and_solve_and_time_more_runs;
use std::cmp::Ordering;

const INPUT_FILE_PATH: &str = "data/day14_input.txt";

enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    None,
}

struct Robot {
    p_x: i128,
    p_y: i128,
    v_x: i128,
    v_y: i128,
}

impl Robot {
    fn get_quadrant(&self, step_count: usize, area_width: usize, area_height: usize) -> Quadrant {
        let n = step_count as i128;
        let w = area_width as i128;
        let h = area_height as i128;
        let mut final_x = (self.p_x + self.v_x * n) % w;
        let mut final_y = (self.p_y + self.v_y * n) % h;
        let mid_w = (w - 1) / 2;
        let mid_h = (h - 1) / 2;
        if final_x < 0 {
            final_x += w;
        }
        if final_y < 0 {
            final_y += h;
        }
        match (final_x.cmp(&mid_w), final_y.cmp(&mid_h)) {
            (Ordering::Less, Ordering::Less) => Quadrant::TopLeft,
            (Ordering::Less, Ordering::Greater) => Quadrant::TopRight,
            (Ordering::Greater, Ordering::Less) => Quadrant::BottomLeft,
            (Ordering::Greater, Ordering::Greater) => Quadrant::BottomRight,
            _ => Quadrant::None,
        }
    }
}

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 14 part 1", solve, 1000);
}

fn solve(contents: &str) -> i128 {
    solve_for_area(contents, 100, 101, 103)
}

fn solve_for_area(
    contents: &str,
    step_count: usize,
    area_width: usize,
    area_height: usize,
) -> i128 {
    let robots = parse(contents);
    let quadrant_counts = robots.iter().fold((0, 0, 0, 0), |counts, robot| {
        match robot.get_quadrant(step_count, area_width, area_height) {
            Quadrant::TopLeft => (counts.0 + 1, counts.1, counts.2, counts.3),
            Quadrant::TopRight => (counts.0, counts.1 + 1, counts.2, counts.3),
            Quadrant::BottomLeft => (counts.0, counts.1, counts.2 + 1, counts.3),
            Quadrant::BottomRight => (counts.0, counts.1, counts.2, counts.3 + 1),
            Quadrant::None => counts,
        }
    });
    quadrant_counts.0 * quadrant_counts.1 * quadrant_counts.2 * quadrant_counts.3
}

fn parse(contents: &str) -> Vec<Robot> {
    contents.lines().map(parse_robot).collect()
}

fn parse_robot(line: &str) -> Robot {
    let (left, right) = line.split_once(' ').unwrap();
    let (p_x, p_y) = parse_pair(left);
    let (v_x, v_y) = parse_pair(right);
    Robot { p_x, p_y, v_x, v_y }
}

fn parse_pair(coordinates: &str) -> (i128, i128) {
    let (left, right) = coordinates
        .split_once('=')
        .unwrap()
        .1
        .split_once(',')
        .unwrap();
    (
        left.parse::<i128>().unwrap(),
        right.parse::<i128>().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use crate::solve_for_area;

    #[test]
    fn test_example() {
        let contents = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

        assert_eq!(solve_for_area(contents, 100, 11, 7), 12);
    }
}
