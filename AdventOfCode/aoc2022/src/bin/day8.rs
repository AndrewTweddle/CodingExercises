#[derive(Debug)]
struct Tree {
    height: i8,
    max_left: i8,
    max_right: i8,
    max_up: i8,
    max_down: i8,
}

impl Tree {
    fn new(height: i8) -> Self {
        Tree {
            height,
            max_left: 0,
            max_right: 0,
            max_up: 0,
            max_down: 0
        }
    }

    fn is_visible(&self) -> bool {
        let ht = self.height;
        ht > self.max_left || ht > self.max_right || ht > self.max_up || ht > self.max_down
    }
}

fn parse_trees(contents: &str) -> Vec<Vec<Tree>> {
    contents.lines().map(|line|
        line.as_bytes().iter().map(|byte| Tree::new((byte - b'0') as i8)).collect::<Vec<Tree>>()
    ).collect::<Vec<Vec<Tree>>>()
}

fn calculate_visibility(trees: &mut Vec<Vec<Tree>>) {
    let row_count = trees.len();
    let col_count = trees.first().unwrap().len();

    for r in 0..row_count {
        let mut max_left: i8 = -1;
        let mut max_right: i8 = -1;
        let row = &mut trees[r];
        for c in 0..col_count {
            let left_tree = &mut row[c];
            left_tree.max_left = max_left;
            max_left = max_left.max(left_tree.height);

            let right_tree = &mut row[col_count - 1 - c];
            right_tree.max_right = max_right;
            max_right = max_right.max(right_tree.height);
        }
    }

    for c in 0..col_count {
        let mut max_up: i8 = -1;
        let mut max_down: i8 = -1;
        for r in 0..row_count {
            let top_tree = &mut trees[r][c];
            top_tree.max_up = max_up;
            max_up = max_up.max(top_tree.height);

            let bottom_tree = &mut trees[row_count - 1 - r][c];
            bottom_tree.max_down = max_down;
            max_down = max_down.max(bottom_tree.height);
        }
    }
}

fn count_visible_trees(trees: &Vec<Vec<Tree>>) -> usize {
    trees.iter().map(|row| {
        row.iter().filter(|tree| tree.is_visible()).count()
    }).sum()
}

fn main() {
    let contents = std::fs::read_to_string("data/day8_input.txt").unwrap();
    let mut trees = parse_trees(contents.as_str());

    calculate_visibility(&mut trees);
    let visible_count: usize = count_visible_trees(&trees);

    println!("Day 8 part 1 answer: {}", visible_count)
}

#[cfg(test)]
mod tests {
    use crate::{calculate_visibility, count_visible_trees, parse_trees};

    #[test]
    fn test_example() {
        let contents = "30373\n\
                        25512\n\
                        65332\n\
                        33549\n\
                        35390";
        let mut trees = parse_trees(contents);
        calculate_visibility(&mut trees);
        let visible_count = count_visible_trees(&trees);
        assert_eq!(visible_count, 21);
    }
}