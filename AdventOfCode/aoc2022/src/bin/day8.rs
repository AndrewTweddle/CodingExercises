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

fn calculate_scenic_score(trees: &Vec<Vec<Tree>>, row_index: usize, col_index: usize) -> usize {
    let row = &trees[row_index];
    let height = row[col_index].height;
    let row_count = trees.len();
    let col_count = row.len();
    let left_tree_count: usize = (row[0..col_index].iter().rev().take_while(|tree|
        tree.height < height
    ).count() + 1).min(col_index);
    let right_tree_count: usize = (row[(col_index + 1)..col_count].iter().take_while(|tree|
        tree.height < height
    ).count() + 1).min(col_count - col_index - 1);
    let top_tree_count: usize = ((0..row_index).rev().take_while(|&r|
        trees[r][col_index].height < height
    ).count() + 1).min(row_index);
    let bottom_tree_count: usize = (((row_index + 1)..row_count).take_while(|&r|
        trees[r][col_index].height < height
    ).count() + 1).min(row_count - row_index - 1);
    left_tree_count * right_tree_count * top_tree_count * bottom_tree_count
}

fn get_max_scenic_core(trees: &Vec<Vec<Tree>>) -> usize {
    let row_count = trees.len();
    let col_count = trees.first().unwrap().len();
    let mut max_scenic_score: usize = 0;
    for r in 0..row_count {
        for c in 0..col_count {
            let scenic_score = calculate_scenic_score(trees, r, c);
            if scenic_score > max_scenic_score {
                max_scenic_score = scenic_score;
            }
        }
    }
    max_scenic_score
}

fn main() {
    let contents = std::fs::read_to_string("data/day8_input.txt").unwrap();
    let mut trees = parse_trees(contents.as_str());

    calculate_visibility(&mut trees);
    let day8_part1_answer: usize = count_visible_trees(&trees);
    println!("Day 8 part 1 answer: {}", day8_part1_answer);

    let day8_part2_answer = get_max_scenic_core(&trees);
    println!("Day 8 part 2 answer: {}", day8_part2_answer);
}

#[cfg(test)]
mod tests {
    use crate::{calculate_scenic_score, calculate_visibility, count_visible_trees, get_max_scenic_core, parse_trees};

    const TEST_CONTENTS: &str = "30373\n\
                                 25512\n\
                                 65332\n\
                                 33549\n\
                                 35390";

    #[test]
    fn test_part1_example() {
        let mut trees = parse_trees(TEST_CONTENTS);
        calculate_visibility(&mut trees);
        let visible_count = count_visible_trees(&trees);
        assert_eq!(visible_count, 21);
    }

    #[test]
    fn test_part2_first_example() {
        let trees = parse_trees(TEST_CONTENTS);
        let scenic_score = calculate_scenic_score(&trees, 1, 2);
        assert_eq!(scenic_score, 4);
    }

    #[test]
    fn test_part2_second_example() {
        let trees = parse_trees(TEST_CONTENTS);
        let scenic_score = calculate_scenic_score(&trees, 3, 2);
        assert_eq!(scenic_score, 8);
    }

    #[test]
    fn test_part2_max_scenic_score_example() {
        let trees = parse_trees(TEST_CONTENTS);
        let max_scenic_score = get_max_scenic_core(&trees);
        assert_eq!(max_scenic_score, 8);
    }
}