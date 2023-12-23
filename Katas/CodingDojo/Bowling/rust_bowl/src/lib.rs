mod pattern_scorer;
mod c_api;

use thiserror::Error;

pub type Score = u16;
pub type Frame = u8;
pub type PinCount = u8;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum BowlingScorerError {
    #[error("Unrecognized symbol: {0}")]
    UnrecognizedSymbol(char),
    #[error("Invalid pattern: Next throw cannot be a spare")]
    FirstOf2ThrowsCannotBeASpare,
    #[error("Invalid pattern: the throw after a strike cannot be a spare")]
    ThrowAfterAStrikeCannotBeASpare,
    #[error("Invalid pattern: the first ball of frame {0} is a spare")]
    FirstThrowOfAFrameCannotBeASpare(Frame),
    #[error("Invalid pattern: the second ball of frame {0} is a strike")]
    SecondThrowInFrameCannotBeAStrike(Frame),
    #[error("Invalid frame {0}")]
    InvalidFrame(Frame),
    #[error("Invalid pattern: no more throws in frame {0}")]
    NoMoreThrowsInFrame(Frame),
    #[error("Invalid pattern: not enough throws in frame {0}")]
    NotEnoughThrowsInFrame(Frame),
    #[error("Invalid pattern: no more throws after spare in frame {0}")]
    NoMoreThrowsAfterSpareInFrame(Frame),
    #[error("Invalid pattern: not enough throws after strike in frame {0}")]
    NotEnoughThrowsAfterStrikeInFrame(Frame),
    #[error("Invalid pattern: two throws in a frame {0} add up to {1}")]
    ThrowsInFrameAddUpToTenOrMore(Frame, Score),
    #[error("Invalid pattern: a spare in frame 10 cannot be followed by another spare")]
    SpareCannotFollowASpareInFrame10,
    #[error("Invalid pattern at frame {frame}: {all_remaining_throws:?}")]
    InvalidPatternInFrame {
        frame: Frame,
        all_remaining_throws: Vec<Throw>,
    },
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Throw {
    Pins(PinCount),
    Spare,
    Strike,
}

impl From<char> for Throw {
    fn from(ch: char) -> Self {
        char_to_throw(ch).unwrap()
    }
}

impl From<&Throw> for Score {
    fn from(throw: &Throw) -> Self {
        match throw {
            Throw::Pins(pins) => *pins as Score,
            _ => 10,
        }
    }
}

impl Throw {
    fn score(&self) -> Score {
        Score::from(self)
    }
}

fn char_to_throw(ch: char) -> Result<Throw, BowlingScorerError> {
    match ch {
        '0'..='9' => Ok(Throw::Pins((ch as PinCount) - b'0')),
        '-' => Ok(Throw::Pins(0)),
        '/' => Ok(Throw::Spare),
        'X' => Ok(Throw::Strike),
        _ => Err(BowlingScorerError::UnrecognizedSymbol(ch)),
    }
}

pub fn convert_symbols_to_throws(symbols: &str) -> Result<Vec<Throw>, BowlingScorerError> {
    let result = symbols.chars().map(char_to_throw).collect();
    result
}

pub fn score_next_2_throws(throw1: &Throw, throw2: &Throw) -> Result<Score, BowlingScorerError> {
    match (throw1, throw2) {
        (Throw::Spare, _) => Err(BowlingScorerError::FirstOf2ThrowsCannotBeASpare),
        (Throw::Strike, Throw::Spare) => Err(BowlingScorerError::ThrowAfterAStrikeCannotBeASpare),
        (_, Throw::Spare) => Ok(10),
        (throw1, throw2) => Ok(throw1.score() + throw2.score()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_spare_char_score() {
        let throw: Throw = '/'.into();
        let score = Score::from(&throw);
        assert_eq!(score, 10);
    }

    #[test]
    fn check_strike_char_score() {
        let throw: Throw = 'X'.into();
        let score = Score::from(&throw);
        assert_eq!(score, 10);
    }

    #[test]
    fn check_gutter_ball_char_score() {
        let throw: Throw = '-'.into();
        let score = Score::from(&throw);
        assert_eq!(score, 0);
    }

    #[test]
    fn check_numeric_char_score() {
        let throw: Throw = '4'.into();
        let score = Score::from(&throw);
        assert_eq!(score, 4);
    }

    #[test]
    fn check_sum_of_2_throws_making_a_spare() {
        let score = score_next_2_throws(&Throw::Pins(4), &Throw::Spare);
        assert_eq!(score, Ok(10));
    }

    #[test]
    fn check_sum_of_2_throws_not_clearing_all_pins() {
        let score = score_next_2_throws(&Throw::Pins(4), &Throw::Pins(2));
        assert_eq!(score, Ok(6));
    }

    #[test]
    fn check_sum_of_2_throws_where_first_is_a_spare() {
        let score = score_next_2_throws(&Throw::Spare, &Throw::Pins(4));
        assert_eq!(score, Err(BowlingScorerError::FirstOf2ThrowsCannotBeASpare));
    }

    #[test]
    fn check_sum_of_2_throws_where_a_spare_precedes_a_strike() {
        let score = score_next_2_throws(&Throw::Spare, &Throw::Strike);
        assert_eq!(score, Err(BowlingScorerError::FirstOf2ThrowsCannotBeASpare));
    }

    #[test]
    fn check_sum_of_2_throws_where_a_spare_follows_a_strike() {
        let score = score_next_2_throws(&Throw::Strike, &Throw::Spare);
        assert_eq!(
            score,
            Err(BowlingScorerError::ThrowAfterAStrikeCannotBeASpare)
        );
    }

    #[test]
    pub fn test_converting_all_valid_symbols() {
        let symbols = "-0123456789/X";
        let result = convert_symbols_to_throws(symbols);

        let expected_throws = vec![
            Throw::Pins(0),
            Throw::Pins(0),
            Throw::Pins(1),
            Throw::Pins(2),
            Throw::Pins(3),
            Throw::Pins(4),
            Throw::Pins(5),
            Throw::Pins(6),
            Throw::Pins(7),
            Throw::Pins(8),
            Throw::Pins(9),
            Throw::Spare,
            Throw::Strike,
        ];
        assert_eq!(result, Ok(expected_throws));
    }

    #[test]
    pub fn test_converting_invalid_symbols() {
        let symbols = "2-X1/*";
        let result = convert_symbols_to_throws(symbols);
        assert_eq!(result, Err(BowlingScorerError::UnrecognizedSymbol('*')));
    }
}
