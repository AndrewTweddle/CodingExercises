fn main() {
    let contents = std::fs::read_to_string("data/day3_input.txt").unwrap();
    let lines: Vec<&[u8]> = contents.lines().map(|line| line.as_bytes()).collect();
    let sum_of_priorities: u32 = lines
        .chunks(3)
        .map(|elves| {
            let &badge = elves[0]
                .iter()
                .find(|&byte1| elves[1].contains(byte1) && elves[2].contains(byte1))
                .unwrap();
            (if badge >= b'a' {
                1 + badge - b'a'
            } else {
                27 + badge - b'A'
            }) as u32
        })
        .sum();
    println!("2022 day 3 part 3 answer: {}", sum_of_priorities);
}
