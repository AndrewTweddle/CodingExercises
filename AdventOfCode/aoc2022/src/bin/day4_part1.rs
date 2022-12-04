fn main() {
    let contents = std::fs::read_to_string("data/day4_input.txt").unwrap();
    let contained_count = contents
        .lines()
        .filter(|ln| {
            let (l_str, r_str) = ln.split_once(',').unwrap();
            let (lft, rgt) = (parse_elf(l_str), parse_elf(r_str));
            (lft.0 <= rgt.0 && lft.1 >= rgt.1) || (lft.0 >= rgt.0 && lft.1 <= rgt.1)
        })
        .count();
    println!("2022 day  part 1 answer: {}", contained_count);
}

fn parse_elf(elf_str: &str) -> (u32, u32) {
    let (start_str, end_str) = elf_str.split_once('-').unwrap();
    let start = start_str.parse().unwrap();
    let end = end_str.parse().unwrap();
    (start, end)
}
