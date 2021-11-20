use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Line {
    pos_one: usize,
    pos_two: usize,
    letter: char,
    password: String,
}

fn main() {
    let path = "data/day2_input";
    let input = File::open(path).unwrap();
    let br = BufReader::new(input);

    let re =
        Regex::new(r"^(?P<pos_one>\d+)-(?P<pos_two>\d+)\s*(?P<letter>\w):\s*(?P<password>\w*)$")
            .unwrap();

    let inputs = br.lines().map(|ln| {
        let line = ln.unwrap();
        let caps = re.captures(&line).unwrap();
        let pos_one = caps
            .name("pos_one")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap()
            - 1; // Change it to be zero-based
        let pos_two = caps
            .name("pos_two")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap()
            - 1; // Change it to be zero-based
        let letter = caps
            .name("letter")
            .unwrap()
            .as_str()
            .chars()
            .next()
            .unwrap();
        let password = caps.name("password").unwrap().as_str().to_string();
        Line {
            pos_one,
            pos_two,
            letter,
            password,
        }
    });
    let valid_password_count = inputs
        .filter(|input| {
            let password = &input.password;
            let has_one = password.len() > input.pos_one
                && password.chars().nth(input.pos_one) == Some(input.letter);
            let has_two = password.len() > input.pos_two
                && password.chars().nth(input.pos_two) == Some(input.letter);
            has_one != has_two
        })
        .count();

    println!("# of valid passwords: {}", valid_password_count);
}
