use std::cmp::Ordering;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 1000;

// Each turn consists of 3 dice throws followed by adding the position (1 to 10) to the score.
// Every turn the score goes up by at least 1, so it will take at most 21 turns to finish.
type WorldTallyByTurn = [u128; 22];

// Each player's turn consists of 3 dice throws, summing up to 9 at most.
// This tracks the number of ways (i.e. possible worlds) of reaching each possible sum of 3 throws.
type WorldTallyBySumOf3Throws = [u8; 10];

fn main() {
    let mut start_time = Instant::now();

    let player1_start_pos = 7;
    let player2_start_pos = 2;

    for rep in 0..NUM_REPETITIONS {
        // Calculate and cache the number of ways of reaching each possible
        // sum of 3 dice throws (the array index is the sum of the 3 throws)...
        let mut world_tallies_by_dice_sum = WorldTallyBySumOf3Throws::default();
        tally_worlds_by_sum_of_3_dice_throws(&mut world_tallies_by_dice_sum);

        // Tally the number of different ways of either finishing (reaching 21)
        // or not finishing in N turns for each player. The array indices are the number of turns.
        let mut player1_worlds_finishing_on_turn = WorldTallyByTurn::default();
        let mut player2_worlds_finishing_on_turn = WorldTallyByTurn::default();

        let mut player1_worlds_not_finishing_by_turn = WorldTallyByTurn::default();
        let mut player2_worlds_not_finishing_by_turn = WorldTallyByTurn::default();

        tally_worlds_by_turn_for_player(
            player1_start_pos,
            0,
            1, // 1 way to start
            0,
            &world_tallies_by_dice_sum,
            &mut player1_worlds_finishing_on_turn,
            &mut player1_worlds_not_finishing_by_turn,
        );
        tally_worlds_by_turn_for_player(
            player2_start_pos,
            0,
            1, // 1 way to start
            0,
            &world_tallies_by_dice_sum,
            &mut player2_worlds_finishing_on_turn,
            &mut player2_worlds_not_finishing_by_turn,
        );

        let mut player1_wins: u128 = 0;
        let mut player2_wins: u128 = 0;

        for turn in 3..=21 {
            player1_wins += player1_worlds_finishing_on_turn[turn]
                * player2_worlds_not_finishing_by_turn[turn - 1];
            player2_wins +=
                player2_worlds_finishing_on_turn[turn] * player1_worlds_not_finishing_by_turn[turn];
        }

        let (winning_player, winning_worlds) = match player1_wins.cmp(&player2_wins) {
            Ordering::Greater => ("1", player1_wins),
            Ordering::Less => ("2", player2_wins),
            Ordering::Equal => ("tied", player1_wins),
        };

        if rep == 0 {
            println!("Day 21, part 2:");
            println!("---------------");
            println!("Player 1 winning worlds: {}", player1_wins);
            println!("Player 2 winning worlds: {}", player2_wins);
            println!();
            println!("Player winning most often: {}", winning_player);
            println!("Winning worlds for player: {}", winning_worlds);
            println!();

            // Restart timer, to time remaining repetitions with no I/O such as printlns...
            start_time = Instant::now();
        }
    }

    let avg_duration = start_time.elapsed() / NUM_REPETITIONS;
    println!(
        "Avg duration after {} repetitions: {:?}",
        NUM_REPETITIONS, avg_duration
    );
}

fn tally_worlds_by_sum_of_3_dice_throws(
    world_tallies_by_sum_of_3_throws: &mut WorldTallyBySumOf3Throws,
) {
    for t1 in 1_usize..=3 {
        for t2 in 1_usize..=3 {
            for t3 in 1_usize..=3 {
                world_tallies_by_sum_of_3_throws[t1 + t2 + t3] += 1;
            }
        }
    }
}

fn tally_worlds_by_turn_for_player(
    position: u8,
    score: u8,
    worlds: u128,
    turn_count: u8,
    tallies_by_dice_sum: &[u8; 10],
    worlds_finishing_on_turn: &mut WorldTallyByTurn,
    worlds_not_finishing_by_turn: &mut WorldTallyByTurn,
) {
    tallies_by_dice_sum
        .iter()
        .enumerate()
        .filter(|(_, &ways_to_reach_turn_sum)| ways_to_reach_turn_sum != 0)
        .for_each(|(dice_sum, &ways_to_reach_dice_sum)| {
            let new_position = (position + dice_sum as u8 - 1) % 10 + 1;
            let new_score = score + new_position;
            let new_worlds = worlds * ways_to_reach_dice_sum as u128;
            if new_score >= 21 {
                worlds_finishing_on_turn[turn_count as usize + 1] += new_worlds;
            } else {
                worlds_not_finishing_by_turn[turn_count as usize + 1] += new_worlds;

                // recursively simulate more throws starting from the new state
                tally_worlds_by_turn_for_player(
                    new_position,
                    new_score,
                    new_worlds,
                    turn_count + 1,
                    tallies_by_dice_sum,
                    worlds_finishing_on_turn,
                    worlds_not_finishing_by_turn,
                );
            }
        });
}
