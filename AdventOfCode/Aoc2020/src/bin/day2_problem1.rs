use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Line {
    min_count: usize,
    max_count: usize,
    letter: char,
    password: String,
}

fn main() {
    let path = "data/day2_input";
    let input = File::open(path).unwrap();
    let br = BufReader::new(input);

    let re = Regex::new(
        r"^(?P<min_count>\d+)-(?P<max_count>\d+)\s*(?P<letter>\w):\s*(?P<password>\w*)$",
    )
    .unwrap();

    let inputs = br.lines().map(|ln| {
        let line = ln.unwrap();
        let caps = re.captures(&line).unwrap();
        let min_count = caps
            .name("min_count")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let max_count = caps
            .name("max_count")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let letter = caps
            .name("letter")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let password = caps.name("password").unwrap().as_str().to_string();
        Line {
            min_count,
            max_count,
            letter,
            password,
        }
    });
    let valid_password_count = inputs
        .filter(|input| {
            let letter_count = input
                .password
                .chars()
                .filter(|&ch| ch == input.letter)
                .count();
            letter_count >= input.min_count && letter_count <= input.max_count
        })
        .count();

    println!("# of valid passwords: {}", valid_password_count);
}
