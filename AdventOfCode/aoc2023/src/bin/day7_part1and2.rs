use aoc2023::read_and_solve_and_time_more_runs;

struct Hand {
    hand_value: u32,
    bid: u32,
}

const RANK_COUNT: usize = 13;
const PRIMARY_MULTIPLIER: u32 = 13 * 13 * 13 * 13 * 13;
const INPUT_FILE_PATH: &str = "data/day7_input.txt";

fn main() {
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 7 part 1", solve_part_1, 10_000);
    read_and_solve_and_time_more_runs(INPUT_FILE_PATH, "Day 7 part 2", solve_part_2, 10_000);
}

fn solve_part_1(contents: &str) -> u32 {
    solve(contents, false, part1_symbol_to_rank_index)
}

fn solve_part_2(contents: &str) -> u32 {
    solve(contents, true, part2_symbol_to_rank_index)
}

fn solve<S>(contents: &str, treat_j_as_joker: bool, symbol_to_rank_index: S) -> u32
where
    S: Fn(u8) -> u8,
{
    let mut hands: Vec<Hand> = contents
        .lines()
        .map(|ln| {
            let (hand, bid_str) = ln.split_once(' ').expect("No separator found in line.");
            let bid: u32 = bid_str
                .parse()
                .expect("The bid could not be parsed as a number.");

            // Count the cards of each rank.
            let mut card_counts = [0_u8; RANK_COUNT];

            // The secondary value of the hand is the lexicographic value of the symbols in order.
            // We can treat the hand as a base 13 number with 5 digits, to calculate this value.
            let mut hand_value: u32 = 0;
            let mut joker_count: u8 = 0;

            for rank_index in hand.bytes().map(&symbol_to_rank_index) {
                hand_value *= RANK_COUNT as u32;
                if treat_j_as_joker && rank_index == 0 {
                    joker_count += 1;
                } else {
                    hand_value += rank_index as u32;
                    card_counts[rank_index as usize] += 1;
                }
            }

            // Calculate the highest and second highest counts of cards of the same rank
            let mut most_of_a_rank = 0;
            let mut second_most_of_a_rank = 0;

            for count in card_counts {
                if count > most_of_a_rank {
                    second_most_of_a_rank = most_of_a_rank;
                    most_of_a_rank = count;
                } else if count > second_most_of_a_rank {
                    second_most_of_a_rank = count;
                }
            }
            most_of_a_rank += joker_count;

            // The second most of a type can range in value from 0 to 2. So use base 3.
            // Convert each hand to a numeric value which sorts all hands correctly...
            hand_value += PRIMARY_MULTIPLIER * (3 * most_of_a_rank + second_most_of_a_rank) as u32;

            Hand { hand_value, bid }
        })
        .collect();
    hands.sort_by_key(|hand| hand.hand_value);
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i as u32 + 1))
        .sum()
}

fn part1_symbol_to_rank_index(symbol: u8) -> u8 {
    match symbol {
        b'2'..=b'9' => symbol - b'2',
        b'T' => 8,
        b'J' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => panic!("Unrecognized card type '{}'.", symbol as char),
    }
}

fn part2_symbol_to_rank_index(symbol: u8) -> u8 {
    match symbol {
        b'J' => 0,
        b'2'..=b'9' => symbol - b'1',
        b'T' => 9,
        b'Q' => 10,
        b'K' => 11,
        b'A' => 12,
        _ => panic!("Unrecognized card type '{}'.", symbol as char),
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};

    const EXAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part1_example() {
        let total = solve_part_1(EXAMPLE);
        assert_eq!(total, 6440);
    }

    #[test]
    fn test_part2_example() {
        let total = solve_part_2(EXAMPLE);
        assert_eq!(total, 5905);
    }
}
