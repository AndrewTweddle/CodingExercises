use aoc2023::read_and_solve_and_time_more_runs;
use std::str::FromStr;

fn main() {
    read_and_solve_and_time_more_runs(
        "data/day2_input.txt",
        "Day 2 part 2",
        get_sum_of_powers_of_cube_sets,
        10_000,
    );
}

fn get_sum_of_powers_of_cube_sets(contents: &str) -> u64 {
    contents
        .lines()
        .map(|ln| {
            let (_, draws_str) = ln.split_once(':').expect("No colon found in line");
            let mut min_red = 0;
            let mut min_green = 0;
            let mut min_blue = 0;
            let cube_set_iter = draws_str.split(';').flat_map(|draw| {
                draw.split(',').map(|cube_set_str| {
                    let (cube_count_str, cube_colour) = cube_set_str
                        .trim()
                        .split_once(' ')
                        .expect("Pattern doesn't match count and colour separated by a space");
                    let cube_count = u64::from_str(cube_count_str).expect("Could not parse count");
                    (cube_colour, cube_count)
                })
            });
            for (cube_colour, cube_count) in cube_set_iter {
                let min_count = match cube_colour {
                    "red" => &mut min_red,
                    "green" => &mut min_green,
                    "blue" => &mut min_blue,
                    _ => panic!("Unexpected cube colour: {cube_colour}"),
                };
                if cube_count > *min_count {
                    *min_count = cube_count;
                }
            }
            min_red * min_green * min_blue
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::get_sum_of_powers_of_cube_sets;

    #[test]
    fn test_part1_example() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let total = get_sum_of_powers_of_cube_sets(contents);
        assert_eq!(total, 2286);
    }
}
