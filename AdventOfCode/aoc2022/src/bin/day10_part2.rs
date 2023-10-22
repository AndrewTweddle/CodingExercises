use std::fs;

const ADDX_STR: &str = "addx ";
const NOOP_STR: &str = "noop";

fn main() {
    let contents = fs::read_to_string("data/day10_input.txt").unwrap();
    let mut cycle = 0;
    let mut x = 1;
    let mut image = String::with_capacity(246);
    for line in contents.lines() {
        if let Some(value_str) = line.strip_prefix(ADDX_STR) {
            let value = value_str
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("addx not a followed by an integer: {}", line));
            update_image_and_increment_cycle(x, &mut cycle, &mut image);
            update_image_and_increment_cycle(x, &mut cycle, &mut image);
            x += value;
        } else if line == NOOP_STR {
            update_image_and_increment_cycle(x, &mut cycle, &mut image);
        } else {
            panic!("Unexpected instruction {line}");
        }
    }
    println!("{image}");
}

fn update_image_and_increment_cycle(x: i64, cycle: &mut i64, image: &mut String) {
    let pixel_pos = *cycle % 40;
    let pixel = if (pixel_pos - x).abs() <= 1 { '#' } else { '.' };
    image.push(pixel);
    if pixel_pos == 39 {
        image.push('\n');
    }
    *cycle += 1;
}
