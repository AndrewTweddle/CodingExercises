use super::*;

struct PatternScorer;

impl BowlingScorer for PatternScorer {
    fn score(throws: &str) -> Result<Score, BowlingScorerError> {
        let throws: Vec<Throw> = convert_symbols_to_throws(throws)?;
        score_remaining_frames(&throws, 1, 0)
    }
}

fn score_remaining_frames(
    rem_throws: &[Throw],
    frame: Frame,
    partial_score: Score,
) -> Result<Score, BowlingScorerError> {
    if frame == 0 || frame > 10 {
        return Err(BowlingScorerError::InvalidFrame(frame));
    }
    match (frame, rem_throws) {
        // Validation patterns
        (frm, []) => Err(BowlingScorerError::NoMoreThrowsInFrame(frm)),
        (frm, [last_throw]) if *last_throw != Throw::Strike => {
            Err(BowlingScorerError::NotEnoughThrowsInFrame(frm))
        }
        (frm, [_, Throw::Spare]) => Err(BowlingScorerError::NoMoreThrowsAfterSpareInFrame(frm)),
        (frm, [Throw::Strike]) => Err(BowlingScorerError::NotEnoughThrowsAfterStrikeInFrame(frm)),
        (frm, [Throw::Strike, _]) => {
            Err(BowlingScorerError::NotEnoughThrowsAfterStrikeInFrame(frm))
        }
        (frm, [Throw::Spare, ..]) => Err(BowlingScorerError::FirstThrowOfAFrameCannotBeASpare(frm)),
        (frm, [first_throw, Throw::Strike]) if *first_throw != Throw::Strike => {
            Err(BowlingScorerError::SecondThrowInFrameCannotBeAStrike(frm))
        }
        (frm, [Throw::Pins(pins1), Throw::Pins(pins2), ..]) if *pins1 + *pins2 >= 10 => {
            let sum_of_pins = *pins1 as Score + *pins2 as Score;
            Err(BowlingScorerError::ThrowsInFrameAddUpToTenOrMore(
                frm,
                sum_of_pins,
            ))
        }
        (frm, [Throw::Strike, Throw::Pins(_), Throw::Strike]) => {
            Err(BowlingScorerError::SecondThrowInFrameCannotBeAStrike(frm))
        }
        (10, [Throw::Strike, Throw::Pins(pins1), Throw::Pins(pins2), ..])
            if *pins1 + *pins2 >= 10 =>
        {
            let sum_of_pins = *pins1 as Score + *pins2 as Score;
            Err(BowlingScorerError::ThrowsInFrameAddUpToTenOrMore(
                10,
                sum_of_pins,
            ))
        }
        (10, [_, Throw::Spare, Throw::Spare]) => {
            Err(BowlingScorerError::SpareCannotFollowASpareInFrame10)
        }
        // Calculation patterns
        (10, [Throw::Strike, throw2, throw3]) => {
            Ok(partial_score + 10 + score_next_2_throws(throw2, throw3)?)
        }
        (10, [throw1, Throw::Spare, throw3]) if throw1.score() < 10  => {
            Ok(partial_score + 10 + throw3.score())
        }
        (10, [throw1, throw2]) if throw1.score() < 10 => {
            Ok(partial_score + score_next_2_throws(throw1, throw2)?)
        }
        (_, [Throw::Strike, throw2, throw3, ..]) => score_remaining_frames(
            &rem_throws[1..],
            frame + 1,
            partial_score + 10 + score_next_2_throws(throw2, throw3)?,
        ),
        (_, [throw1, Throw::Spare, throw3, ..]) if throw1.score() < 10 => score_remaining_frames(
            &rem_throws[2..],
            frame + 1,
            partial_score + 10 + throw3.score(),
        ),
        (_, [throw1, throw2, ..]) if throw1.score() < 10 => score_remaining_frames(
            &rem_throws[2..],
            frame + 1,
            partial_score + score_next_2_throws(throw1, throw2)?,
        ),
        // Catch-all for any other invalid patterns
        _ => {
            let all_remaining_throws = rem_throws.to_vec();
            Err(BowlingScorerError::InvalidPatternInFrame {
                frame,
                all_remaining_throws,
            })
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
        let throws = "X--".repeat(5);
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
        let throws = "X5/".to_string() + "--".repeat(8).as_str();
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Ok(30));
    }

    #[test]
    fn test_a_game_with_too_many_frames() {
        let throws = "54".repeat(11);
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Err(BowlingScorerError::InvalidFrame(11)));
    }

    #[test]
    fn test_a_game_with_too_few_frames_and_with_a_strike_in_the_last_frame() {
        let throws = "54X";
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::NotEnoughThrowsAfterStrikeInFrame(2))
        );
    }

    #[test]
    fn test_a_game_with_too_few_frames_and_with_a_spare_in_the_last_frame() {
        let throws = "7254-/";
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::NoMoreThrowsAfterSpareInFrame(3))
        );
    }

    #[test]
    fn test_a_game_with_too_few_frames_and_with_no_spare_or_strike_in_the_last_frame() {
        let throws = "9-81726354";
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Err(BowlingScorerError::NoMoreThrowsInFrame(6)));
    }

    #[test]
    fn test_a_game_with_too_few_frames_and_a_single_non_strike_throw_in_the_last_frame() {
        let throws = "9-8172635";
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Err(BowlingScorerError::NotEnoughThrowsInFrame(5)));
    }

    #[test]
    fn test_a_game_with_a_frame_whose_first_throw_is_a_spare() {
        let throws = "54//".repeat(5);
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::FirstThrowOfAFrameCannotBeASpare(2))
        );
    }

    #[test]
    fn test_a_game_with_throws_in_a_frame_adding_up_to_10_but_not_marked_as_a_spare() {
        let throws = "5491".repeat(5);
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::ThrowsInFrameAddUpToTenOrMore(2, 10))
        );
    }

    #[test]
    fn test_a_game_with_throws_in_a_frame_adding_up_to_more_than_10() {
        let throws = "123456789/-".repeat(5);
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::ThrowsInFrameAddUpToTenOrMore(3, 11))
        );
    }

    #[test]
    fn test_a_game_with_a_strike_then_throw_then_spare_in_frame_10() {
        let throws = "--".repeat(9) + "X1/";
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Ok(20));
    }

    #[test]
    fn test_a_game_with_a_strike_then_throw_then_strike_in_frame_10() {
        let throws = "--".repeat(9) + "X1X";
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::SecondThrowInFrameCannotBeAStrike(10))
        );
    }

    #[test]
    fn test_a_game_with_a_throw_then_spare_then_spare_in_frame_10() {
        let throws = "--".repeat(9) + "1//";
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::SpareCannotFollowASpareInFrame10)
        );
    }

    #[test]
    fn test_a_game_with_a_strike_then_two_throws_adding_to_10_in_frame_10() {
        let throws = "--".repeat(9) + "X19";
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::ThrowsInFrameAddUpToTenOrMore(10, 10)),
        );
    }

    #[test]
    fn test_a_game_with_a_spare_symbol_following_a_strike_in_frame_10() {
        let throws = "--".repeat(9) + "X/1";
        let score = PatternScorer::score(&throws);
        assert_eq!(score, Err(BowlingScorerError::FirstOf2ThrowsCannotBeASpare));
    }

    #[test]
    fn test_a_game_with_no_symbols_after_a_strike_in_frame_10() {
        let throws = "--".repeat(9) + "X";
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::NotEnoughThrowsAfterStrikeInFrame(10))
        );
    }

    #[test]
    fn test_a_game_with_one_too_few_symbols_after_a_strike_in_frame_10() {
        let throws = "--".repeat(9) + "X1";
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::NotEnoughThrowsAfterStrikeInFrame(10))
        );
    }

    #[test]
    fn test_a_game_with_symbols_after_a_spare_in_frame_10() {
        let throws = "--".repeat(9) + "1/";
        let score = PatternScorer::score(&throws);
        assert_eq!(
            score,
            Err(BowlingScorerError::NoMoreThrowsAfterSpareInFrame(10))
        );
    }
}
