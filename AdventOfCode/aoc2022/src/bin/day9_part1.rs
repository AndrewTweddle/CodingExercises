use std::collections::HashSet;

type Pos = (isize, isize);

fn main() {
    let contents = std::fs::read_to_string("data/day9_input.txt").unwrap();
    let mut tail_cells_visited = HashSet::<Pos>::new();
    let mut head: Pos = (0, 0);
    let mut tail: Pos = (0, 0);
    tail_cells_visited.insert(tail);
    for line in contents.lines() {
        let (dir_str, steps_str) = line.split_at(2);
        let dir = dir_str.chars().next().unwrap();
        let steps = steps_str.parse::<u8>().unwrap();
        for _ in 0..steps {
            match dir {
                'L' => {
                    if head.0 < tail.0 {
                        tail = head;
                        tail_cells_visited.insert(tail);
                    }
                    head.0 -= 1;
                }
                'R' => {
                    if head.0 > tail.0 {
                        tail = head;
                        tail_cells_visited.insert(tail);
                    }
                    head.0 += 1;
                }
                'U' => {
                    if head.1 > tail.1 {
                        tail = head;
                        tail_cells_visited.insert(tail);
                    }
                    head.1 += 1;
                }
                'D' => {
                    if head.1 < tail.1 {
                        tail = head;
                        tail_cells_visited.insert(tail);
                    }
                    head.1 -= 1;
                }
                unknown_char => panic!("Unrecognized direction: {unknown_char}"),
            }
        }
    }
    println!("AOC 2022: day 9 part 1: {}", tail_cells_visited.len());
}
