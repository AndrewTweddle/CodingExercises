mod pattern_scorer;

pub type Score = u16;
pub type Frame = u8;
pub type PinCount = u8;

#[derive(Debug)]
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

fn char_to_throw(ch: char) -> Result<Throw, String> {
    match ch {
        '0'..='9' => Ok(Throw::Pins((ch as PinCount) - b'0')),
        '.' => Ok(Throw::Pins(0)),
        '/' => Ok(Throw::Spare),
        'X' => Ok(Throw::Strike),
        _ => Err(format!("Unrecognized symbol: {}", ch)),
    }
}

pub fn convert_symbols_to_throws(symbols: &str) -> Result<Vec<Throw>, String> {
    let result: Result<Vec<Throw>, String> = symbols.chars().map(char_to_throw).collect();
    result
}

pub fn sum_of_next_2_throws(throw1: &Throw, throw2: &Throw) -> Result<Score, String> {
    match (throw1, throw2) {
        (Throw::Spare, _) => Err("Invalid pattern: the next throw cannot be a spare".into()),
        (Throw::Strike, Throw::Spare) => {
            Err("Invalid pattern: the throw after a strike cannot be a spare".into())
        }
        (_, Throw::Spare) => Ok(10),
        (throw1, throw2) => Ok(throw1.score() + throw2.score()),
    }
}

pub trait BowlingScorer {
    fn score(throws: &str) -> Result<Score, String>;
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
        let throw: Throw = '.'.into();
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
        let score = sum_of_next_2_throws(&Throw::Pins(4), &Throw::Spare);
        assert_eq!(score, Ok(10));
    }

    #[test]
    fn check_sum_of_2_throws_not_clearing_all_pins() {
        let score = sum_of_next_2_throws(&Throw::Pins(4), &Throw::Pins(2));
        assert_eq!(score, Ok(6));
    }

    #[test]
    fn check_sum_of_2_throws_where_first_is_a_spare() {
        let score = sum_of_next_2_throws(&Throw::Spare, &Throw::Pins(4));
        assert!(score.is_err());
    }

    #[test]
    fn check_sum_of_2_throws_where_a_spare_follows_a_strike() {
        let score = sum_of_next_2_throws(&Throw::Spare, &Throw::Strike);
        assert!(score.is_err());
    }
}
