fn main() {
    let contents = std::fs::read_to_string("data/day3_input.txt").unwrap();
    let sum_of_priorities: u32 = contents
        .lines()
        .map(|line| {
            let (left, right) = line.as_bytes().split_at(line.len() / 2);
            let &common_byte = left
                .iter()
                .find(|left_byte| right.contains(left_byte))
                .unwrap();
            (if common_byte >= b'a' {
                1 + common_byte - b'a'
            } else {
                27 + common_byte - b'A'
            }) as u32
        })
        .sum();
    println!("2022 day 3 part 1 answer: {}", sum_of_priorities);
}
