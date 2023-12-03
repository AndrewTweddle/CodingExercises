use aoc2023::read_and_solve_and_time_more_runs;
use std::str::FromStr;

fn main() {
    read_and_solve_and_time_more_runs(
        "data/day2_input.txt",
        "Day 2 part 1",
        get_sum_of_ids_of_possible_games,
        10_000,
    );
}

const MAX_RED_COUNT: u8 = 12;
const MAX_GREEN_COUNT: u8 = 13;
const MAX_BLUE_COUNT: u8 = 14;

fn get_sum_of_ids_of_possible_games(contents: &str) -> u32 {
    contents
        .lines()
        .filter_map(|ln| {
            let (prefix, draws_str) = ln.split_once(':').expect("No colon found in line");
            let game_id = u32::from_str(&prefix[5..]).expect("No game id found");
            draws_str
                .split(';')
                .all(|draw| {
                    draw.split(',').all(|cube_set_str| {
                        let (cube_count_str, cube_colour) = cube_set_str
                            .trim()
                            .split_once(' ')
                            .expect("Pattern doesn't match count and colour separated by a space");
                        let cube_count =
                            u8::from_str(cube_count_str).expect("Could not parse count");
                        match (cube_count, cube_colour) {
                            (red_count, "red") => red_count <= MAX_RED_COUNT,
                            (green_count, "green") => green_count <= MAX_GREEN_COUNT,
                            (blue_count, "blue") => blue_count <= MAX_BLUE_COUNT,
                            _ => panic!("Unmatched cube colour: {cube_colour}"),
                        }
                    })
                })
                .then_some(game_id)
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::get_sum_of_ids_of_possible_games;

    #[test]
    fn test_part1_example() {
        let contents = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let total = get_sum_of_ids_of_possible_games(contents);
        assert_eq!(total, 8);
    }
}
