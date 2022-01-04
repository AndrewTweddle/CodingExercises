use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let data_file_path = "data/p022_names.txt";
    let text = fs::read_to_string(data_file_path).expect("File contains invalid characters");
    let mut names: Vec<&str> = text
        .strip_prefix("\"")
        .unwrap()
        .strip_suffix("\"")
        .unwrap()
        .split("\",\"")
        .collect();
    names.sort();
    let sum: usize = names
        .iter()
        .enumerate()
        .map(|(index, line)| {
            let order_in_list = index + 1;
            let alpha_value: usize = line
                .trim()
                .chars()
                .map(|ch| ch as usize - 'A' as usize + 1)
                .sum();
            order_in_list * alpha_value
        })
        .sum();
    println!("Sum is {}", sum);

    let duration = start.elapsed();
    println!("Total duration: {:?}", duration);
}
