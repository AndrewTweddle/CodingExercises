use crate::Evaluation::{RoyalFlush, StraightFlush};
use std::cmp::Ordering;
use std::fs;

fn main() {
    let contents = fs::read_to_string("data/0054_poker.txt").expect("Error loading input data");
    solve_and_print_solution_and_time_more_runs_without_printing(|| solve(&contents), 1000);
}

fn solve(contents: &str) -> usize {
    let wins_for_player_1 = contents
        .lines()
        .filter(|line| {
            let (p1_str, p2_str) = line.split_at(3 * CARDS_IN_HAND_COUNT);
            let player1_hand = Hand::parse(p1_str.trim());
            let player2_hand = Hand::parse(p2_str.trim());
            player1_hand > player2_hand
        })
        .count();
    wins_for_player_1
}

const CARDS_IN_HAND_COUNT: usize = 5;
const SUIT_COUNT: usize = 4;
const RANK_COUNT: usize = 13;
const LOWEST_RANK_IN_ROYAL_FLUSH: u8 = (RANK_COUNT - CARDS_IN_HAND_COUNT) as u8;

#[derive(Eq, PartialEq)]
struct Hand {
    cards: [[bool; RANK_COUNT]; SUIT_COUNT],
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Debug)]
struct TallyWithRank {
    count: usize,
    rank: u8,
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum Evaluation {
    HighCard([u8; 5]),     // In descending value
    OnePair([u8; 4]),      // 1st is the value of the pair, then other cards in descending value
    TwoPair([u8; 3]),      // 1st is high value pair, 2nd is low value pair, 3rd is remaining card
    ThreeOfAKind([u8; 3]), // 1st is value of the triple, then other cards in descending order
    Straight(u8),          // The value of the low card in the straight
    Flush([u8; 5]),        // In descending value
    FullHouse((u8, u8)),   // 1st is the value of the triple, 2nd is the value of the pair
    FourOfAKind((u8, u8)), // 1st is the value of the four, 2nd is the value of the extra card
    StraightFlush(u8),     // The value of the low card in the straight
    RoyalFlush,
}

impl Hand {
    fn parse(s: &str) -> Hand {
        let mut hand = Hand {
            cards: [[false; RANK_COUNT]; SUIT_COUNT],
        };
        s.split(' ').for_each(|card_str| {
            let bytes = card_str.as_bytes();
            let rank_byte = bytes[0];
            let rank = match rank_byte {
                b'2'..=b'9' => (rank_byte - b'2') as usize,
                b'T' => 8,
                b'J' => 9,
                b'Q' => 10,
                b'K' => 11,
                b'A' => 12,
                _ => panic!(
                    "Unrecognized rank '{}' in hand '{}'",
                    card_str.chars().nth(0).unwrap(),
                    card_str
                ),
            };
            let suit_byte = bytes[1];
            let suit = match suit_byte {
                b'S' => 0,
                b'H' => 1,
                b'C' => 2,
                b'D' => 3,
                _ => panic!(
                    "Unrecognized suit '{}' in hand '{}'",
                    card_str.chars().nth(1).unwrap(),
                    card_str
                ),
            };
            hand.cards[suit][rank] = true;
        });
        hand
    }

    fn evaluate(&self) -> Evaluation {
        // Check for multiple of the same rank (in which case there can't be a flush)
        let mut counts_by_rank: Vec<TallyWithRank> = Vec::with_capacity(CARDS_IN_HAND_COUNT);
        for r in 0..RANK_COUNT {
            let mut count: usize = 0;
            for s in 0..SUIT_COUNT {
                if self.cards[s][r] {
                    count += 1;
                }
            }
            if count > 0 {
                counts_by_rank.push(TallyWithRank {
                    count,
                    rank: r as u8,
                });
            }
        }
        counts_by_rank.sort();
        counts_by_rank.reverse();

        match counts_by_rank.len() {
            2 => {
                if counts_by_rank[0].count == 4 {
                    Evaluation::FourOfAKind((counts_by_rank[0].rank, counts_by_rank[1].rank))
                } else {
                    Evaluation::FullHouse((counts_by_rank[0].rank, counts_by_rank[1].rank))
                }
            }
            3 => {
                if counts_by_rank[0].count == 3 {
                    Evaluation::ThreeOfAKind([
                        counts_by_rank[0].rank,
                        counts_by_rank[1].rank,
                        counts_by_rank[2].rank,
                    ])
                } else {
                    Evaluation::TwoPair([
                        counts_by_rank[0].rank,
                        counts_by_rank[1].rank,
                        counts_by_rank[2].rank,
                    ])
                }
            }
            4 => Evaluation::OnePair([
                counts_by_rank[0].rank,
                counts_by_rank[1].rank,
                counts_by_rank[2].rank,
                counts_by_rank[3].rank,
            ]),
            5 => {
                // All 5 cards have different ranks, so check for flushes and sequences
                let descending_ranks: [u8; 5] = [
                    counts_by_rank[0].rank,
                    counts_by_rank[1].rank,
                    counts_by_rank[2].rank,
                    counts_by_rank[3].rank,
                    counts_by_rank[4].rank,
                ];
                let highest_rank = descending_ranks[0];
                let lowest_rank = descending_ranks[CARDS_IN_HAND_COUNT - 1];

                // Check for a sequence
                let is_sequence = highest_rank - lowest_rank + 1 == (CARDS_IN_HAND_COUNT as u8);

                // Check for a flush
                let mut is_flush = false;
                for &cards_in_suit in &self.cards {
                    if cards_in_suit.iter().any(|&has_card| has_card) {
                        if is_flush {
                            is_flush = false;
                            break;
                        } else {
                            is_flush = true;
                        }
                    }
                }

                match (is_flush, is_sequence, lowest_rank) {
                    (true, true, LOWEST_RANK_IN_ROYAL_FLUSH) => RoyalFlush,
                    (true, true, _) => StraightFlush(lowest_rank),
                    (true, false, _) => Evaluation::Flush(descending_ranks),
                    (false, true, _) => Evaluation::Straight(lowest_rank),
                    (false, false, _) => Evaluation::HighCard(descending_ranks),
                }
            }
            _ => panic!(
                "Impossible count of cards by rank: {}",
                counts_by_rank.len()
            ),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        let eval1 = self.evaluate();
        let eval2 = other.evaluate();
        eval1.partial_cmp(&eval2)
    }
}

fn solve_and_print_solution_and_time_more_runs_without_printing<S, T>(solve: S, repetitions: u32)
where
    S: Fn() -> T,
    T: std::fmt::Debug,
{
    use std::time::Instant;

    let mut start_time = Instant::now();
    for i in 0..=repetitions {
        let solution = solve();
        if i == 0 {
            println!("Solution: {solution:?}");
            println!(
                "Solved (including writing to terminal) in {:?}",
                start_time.elapsed()
            );

            // Now restart the timer, so that the timings don't include I/O...
            start_time = Instant::now();
        }
    }

    if repetitions > 0 {
        let avg_duration = start_time.elapsed() / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
    }
}
