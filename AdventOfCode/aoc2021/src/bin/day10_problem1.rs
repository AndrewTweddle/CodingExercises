use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let input_file = File::open("data/day10_input").unwrap();
    let mut br = BufReader::new(input_file);
    let mut text = String::new();
    br.read_to_string(&mut text).unwrap();
    let score: u64 = score_text(text.as_str());
    println!("Score: {}", score);
}

fn score_text(text: &str) -> u64 {
    text.lines().map(score_line).sum()
}

fn score_line(line: &str) -> u64 {
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
        score
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::score_text;

    #[test]
    fn test_example() {
        let text = "[({(<(())[]>[[{[]{<()<>>\n\
                    [(()[<>])]({[<{<<[]>>(\n\
                    {([(<{}[<>[]}>{[]{[(<()>\n\
                    (((({<>}<{<{<>}{[]{[]{}\n\
                    [[<[([]))<([[{}[[()]]]\n\
                    [{[{({}]{}}([{[{{{}}([]\n\
                    {<[[]]>}<{[{[{[]{()[[[]\n\
                    [<(<(<(<{}))><([]([]()\n\
                    <{([([[(<>()){}]>(<<{{\n\
                    <{([{{}}[<[[[<>{}]]]>[]]";
        let score = score_text(text);
        assert_eq!(score, 26397);
    }
}
