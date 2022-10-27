use super::*;

struct PatternScorer;

impl BowlingScorer for PatternScorer {
    fn score(throws: &str) -> Result<Score, String> {
        let throws: Vec<Throw> = convert_symbols_to_throws(throws)?;
        score_remaining_frames(&throws, 1, 0)
    }
}

fn score_remaining_frames(
    rem_throws: &[Throw],
    frame: Frame,
    partial_score: Score,
) -> Result<Score, String> {
    if frame == 0 || frame > 10 {
        return Err(format!("Invalid frame {}", frame));
    }
    match (frame, rem_throws) {
        (10, [Throw::Strike, throw2, throw3]) => {
            Ok(partial_score + 10 + sum_of_next_2_throws(throw2, throw3)?)
        }
        (10, [throw1, Throw::Spare, throw3]) if throw1.score() < 10 => {
            Ok(partial_score + 10 + throw3.score())
        }
        (10, [throw1, throw2]) if throw1.score() < 10 => {
            Ok(partial_score + sum_of_next_2_throws(throw1, throw2)?)
        }
        (_, [Throw::Strike, throw2, throw3, ..]) => score_remaining_frames(
            &rem_throws[1..],
            frame + 1,
            partial_score + 10 + sum_of_next_2_throws(throw2, throw3)?,
        ),
        (_, [throw1, Throw::Spare, throw3, ..]) if throw1.score() < 10 => score_remaining_frames(
            &rem_throws[2..],
            frame + 1,
            partial_score + 10 + throw3.score(),
        ),
        (_, [throw1, throw2, ..]) if throw1.score() < 10 => score_remaining_frames(
            &rem_throws[2..],
            frame + 1,
            partial_score + sum_of_next_2_throws(throw1, throw2)?,
        ),
        _ => {
            return Err(format!(
                "Invalid pattern at frame {}: {:?}",
                frame, rem_throws
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_strikes_score_300() {
        let throws = "X".repeat(12);
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Ok(300));
    }

    #[test]
    fn test_a_repeating_strike_followed_by_two_gutter_balls_scores_50() {
        let throws = "X..".repeat(5);
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Ok(50));
    }

    #[test]
    fn test_knocking_down_5_pins_on_every_throw_scores_150() {
        let throws = "5/".repeat(10) + "5";
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Ok(150));
    }

    #[test]
    fn test_a_strike_then_spare_then_all_gutter_balls_scores_30() {
        let throws = "X5/".to_string() + "..".repeat(8).as_str();
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Ok(30));
    }

    #[test]
    fn test_a_game_with_too_many_frames() {
        let throws = "54".repeat(11);
        let score = PatternScorer::score(&throws);
        assert!(score.is_err());
    }
}
