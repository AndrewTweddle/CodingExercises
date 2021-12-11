use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let input_file = File::open("data/day10_input").unwrap();
    let mut br = BufReader::new(input_file);
    let mut text = String::new();
    br.read_to_string(&mut text).unwrap();
    let part1_score: u64 = score_text(text.as_str(), false);
    println!("Part 1 score: {}", part1_score);
    let part2_score: u64 = score_text(text.as_str(), true);
    println!("Part 2 score: {}", part2_score);
}

fn score_text(text: &str, complete_lines: bool) -> u64 {
    if complete_lines {
        let mut scores: Vec<u64> = text
            .lines()
            .filter_map(|line| score_line(line, complete_lines))
            .collect();
        scores.sort();
        scores[scores.len() / 2]
    } else {
        text.lines()
            .map(|line| score_line(line, complete_lines).unwrap_or_default())
            .sum()
    }
}

fn score_line(line: &str, complete_lines: bool) -> Option<u64> {
    let mut stack = Vec::<char>::with_capacity(line.len());
    let score_of_first_corrupt_char: Option<u64> = line
        .chars()
        .filter_map(|ch| match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(ch);
                None
            }
            ')' => {
                if let Some('(') = stack.pop() {
                    None
                } else {
                    Some(3)
                }
            }
            ']' => {
                if let Some('[') = stack.pop() {
                    None
                } else {
                    Some(57)
                }
            }
            '}' => {
                if let Some('{') = stack.pop() {
                    None
                } else {
                    Some(1197)
                }
            }
            '>' => {
                if let Some('<') = stack.pop() {
                    None
                } else {
                    Some(25137)
                }
            }
            _ => panic!("unrecognized character"),
        })
        .next();

    if let Some(score) = score_of_first_corrupt_char {
        // in part 2 only "completable" lines are considered, so ignore lines with errors
        (!complete_lines).then(|| score)
    } else if complete_lines {
        complete_line_and_get_score(&mut stack)
    } else {
        None
    }
}

fn complete_line_and_get_score(stack: &mut Vec<char>) -> Option<u64> {
    (!stack.is_empty()).then(||
        stack.iter().rev().fold(0_u64, |score, ch| {
            5 * score
                + match ch {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!("Unexpected character found when completing line"),
                }
        })
    )
}

#[cfg(test)]
mod tests {
    use crate::{score_line, score_text};

    const EXAMPLE_TEXT: &str = "[({(<(())[]>[[{[]{<()<>>\n\
                                [(()[<>])]({[<{<<[]>>(\n\
                                {([(<{}[<>[]}>{[]{[(<()>\n\
                                (((({<>}<{<{<>}{[]{[]{}\n\
                                [[<[([]))<([[{}[[()]]]\n\
                                [{[{({}]{}}([{[{{{}}([]\n\
                                {<[[]]>}<{[{[{[]{()[[[]\n\
                                [<(<(<(<{}))><([]([]()\n\
                                <{([([[(<>()){}]>(<<{{\n\
                                <{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1_example() {
        let score = score_text(EXAMPLE_TEXT, false);
        assert_eq!(score, 26397);
    }

    #[test]
    fn test_part2_completable_line1() {
        let line = "[({(<(())[]>[[{[]{<()<>>";
        let score = score_line(line, true);
        assert_eq!(score, Some(288957));
    }

    #[test]
    fn test_part2_completable_line2() {
        let line = "[(()[<>])]({[<{<<[]>>(";
        let score = score_line(line, true);
        assert_eq!(score, Some(5566));
    }

    #[test]
    fn test_part2_completable_line3() {
        let line = "(((({<>}<{<{<>}{[]{[]{}";
        let score = score_line(line, true);
        assert_eq!(score, Some(1480781));
    }

    #[test]
    fn test_part2_completable_line4() {
        let line = "{<[[]]>}<{[{[{[]{()[[[]";
        let score = score_line(line, true);
        assert_eq!(score, Some(995444));
    }

    #[test]
    fn test_part2_completable_line5() {
        let line = "<{([{{}}[<[[[<>{}]]]>[]]";
        let score = score_line(line, true);
        assert_eq!(score, Some(294));
    }

    #[test]
    fn test_part2_completable_text() {
        let score = score_text(EXAMPLE_TEXT, true);
        assert_eq!(score, 288957);
    }
}
