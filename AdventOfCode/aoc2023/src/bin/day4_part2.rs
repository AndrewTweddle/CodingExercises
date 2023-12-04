use aoc2023::read_and_solve_and_time_more_runs;
use std::str::FromStr;

fn main() {
    read_and_solve_and_time_more_runs(
        "data/day4_input.txt",
        "Day 4 part 2",
        get_total_scratchcards,
        10_000,
    );
}

fn get_total_scratchcards(contents: &str) -> u64 {
    let mut copy_counts: Vec<u64> = Vec::new();
    let mut card_index: usize = 0; // NB: zero-based, unlike the card number, which we ignore
    contents.lines().for_each(|ln| {
        if card_index == copy_counts.len() {
            copy_counts.push(1);
        } else {
            copy_counts[card_index] += 1;
        }
        let copies_of_this_card = copy_counts[card_index];
        card_index += 1;

        let (cards_str, numbers_str) = ln.split_once('|').expect("No separator found in line");
        let (_, winning_nums_str) = cards_str.split_once(':').expect("No colon found in card");
        let winning_numbers: Vec<u64> = winning_nums_str
            .split_whitespace()
            .map(|winning_num_str| {
                u64::from_str(winning_num_str).expect("Could not parse a winning number")
            })
            .collect();

        let matching_nums = numbers_str
            .split_whitespace()
            .map(|num_str| u64::from_str(num_str).expect("Could not parse one of your numbers"))
            .filter(|num| winning_numbers.contains(num))
            .count();

        for offset in 0..matching_nums {
            let copy_card_index = card_index + offset;
            if copy_card_index == copy_counts.len() {
                copy_counts.push(copies_of_this_card);
            } else {
                copy_counts[copy_card_index] += copies_of_this_card;
            }
        }
    });
    copy_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::get_total_scratchcards;

    #[test]
    fn test_part1_example() {
        let contents = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let total = get_total_scratchcards(contents);
        assert_eq!(total, 30);
    }
}
