use aoc2024_rs::read_and_solve_and_time_more_runs;

const INPUT_FILE_PATH: &str = "data/day13_input.txt";

struct Machine {
    a_x: i128,
    a_y: i128,
    b_x: i128,
    b_y: i128,
    prize_x: i128,
    prize_y: i128,
}

impl Machine {
    fn solve(&self) -> Option<i128> {
        let det = self.a_x * self.b_y - self.a_y * self.b_x;
        if det == 0 {
            // There are either infinitely many or no solutions.
            // Assuming there are infinitely many solutions.
            // Since more tokens are spent on button A,
            // choose the solution that only uses button B
            let n = if self.b_x == 0 {
                0
            } else {
                self.prize_x / self.b_x
            };
            if self.b_x * n == self.prize_x && self.b_y * n == self.prize_y {
                Some(n)
            } else {
                None
            }
        } else {
            // Invert the 2x2 [A B] matrix and multiply by p (column vector of prize coordinates) 
            let a_count = (self.b_y * self.prize_x - self.b_x * self.prize_y) / det;
            let b_count = (self.a_x * self.prize_y - self.a_y * self.prize_x) / det;
            if a_count < 0 || b_count < 0 {
                None
            } else if (a_count * self.a_x + b_count * self.b_x == self.prize_x)
                && (a_count * self.a_y + b_count * self.b_y == self.prize_y)
            {
                Some(3 * a_count + b_count)
            } else {
                None
            }
        }
    }
}

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 13 part 1", solve, 1000);
}

fn solve(contents: &str) -> i128 {
    let machines = parse(contents);
    machines.iter().filter_map(|machine| machine.solve()).sum()
}

fn parse(contents: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    let mut line_iter = contents.lines();
    while let Some(machine) = parse_machine(&mut line_iter) {
        machines.push(machine);
    }
    machines
}

fn parse_machine<'a>(line_iter: &mut impl Iterator<Item = &'a str>) -> Option<Machine> {
    line_iter.next().map(|line1| {
        let (a_x, a_y) = parse_button_line(line1);
        let (b_x, b_y) = parse_button_line(line_iter.next().unwrap());
        let (prize_x, prize_y) = parse_prize_line(line_iter.next().unwrap());
        line_iter.next();
        Machine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        }
    })
}

fn parse_button_line(line: &str) -> (i128, i128) {
    parse_line(line, '+')
}

fn parse_prize_line(line: &str) -> (i128, i128) {
    parse_line(line, '=')
}

fn parse_line(line: &str, sep: char) -> (i128, i128) {
    let (left, right) = line.split_once(',').unwrap();
    let x = left.split_once(sep).unwrap().1.parse::<i128>().unwrap();
    let y = right.split_once(sep).unwrap().1.parse::<i128>().unwrap();
    (x, y)
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn test_first_machine() {
        let contents = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400
";
        let soln = solve(contents);
        assert_eq!(soln, 280);
    }

    #[test]
    fn test_example() {
        let contents = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

        assert_eq!(solve(contents), 480);
    }
}
