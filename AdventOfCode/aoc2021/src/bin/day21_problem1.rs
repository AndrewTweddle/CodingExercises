struct Player {
    position: i32,
    score: i32,
}

fn main() {
    let mut players = [
        Player {
            position: 7,
            score: 0,
        },
        Player {
            position: 2,
            score: 0,
        },
    ];
    let mut roll_index = 0;
    let answer = loop {
        let curr_player_index = ((roll_index % 6) / 3) as usize;
        let curr_player = &mut players[curr_player_index];
        let dice_value = roll_index % 100 + 1;
        curr_player.position = (curr_player.position + dice_value - 1) % 10 + 1;

        // Update the score after the third throw
        if roll_index % 3 == 2 {
            curr_player.score += curr_player.position;
        }
        roll_index += 1;
        if curr_player.score >= 1000 {
            let other_player = &players[1 - curr_player_index];
            break other_player.score * roll_index;
        }
    };
    println!("Day 21, part 1 answer: {}", answer);
}
