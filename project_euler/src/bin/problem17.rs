use std::time::Instant;

const ONE_TO_NINE: &str = "OneTwoThreeFourFiveSixSevenEightNine";
const TEN_TO_NINETEEN: &str =
    "TenElevenTwelveThirteenFourteenFifteenSixteenSeventeenEighteenNineteen";
const TWENTY_TO_NINETY_IN_TENS: &str = "TwentyThirtyFortyFiftySixtySeventyEightyNinety";
const HUNDRED: &str = "Hundred";
const HUNDRED_AND: &str = "HundredAnd";
const ONE_THOUSAND: &str = "OneThousand";

fn main() {
    let start = Instant::now();

    let num_repetitions = 1000;
    for rep in 0..num_repetitions {
        let one_to_nine_len = ONE_TO_NINE.len();
        let one_to_ninety_nine_len = TEN_TO_NINETEEN.len()
            // tens part of 1..9, 20..29, 30..39, ..., 91..99:
            + 10 * TWENTY_TO_NINETY_IN_TENS.len()
            // units part of 1..9, 20..29, 30..39, ..., 90..99 with zeroes blank:
            + 9 * one_to_nine_len;
        let one_to_one_thousand_len = one_to_nine_len
            // 100, 200, 300, ..., 900:
            + 9 * HUNDRED.len()
            // tens and units of 1..99, 101..199, 201..299, ..., 901..999:
            + 10 * one_to_ninety_nine_len
            + 99 * (
                // hundreds digit of 1..99, 101..199, 201..299, ..., 901..999:
                one_to_nine_len
                // Prepend "hundred and" to each of "one" to "nine":
                 + 9 * HUNDRED_AND.len())
            + ONE_THOUSAND.len();
        if rep == 0 {
            println!(
                "Letters in 1 to 1000 written in words: {}",
                one_to_one_thousand_len
            );
        }
    }
    let duration = start.elapsed();
    println!("Total duration: {:?}", duration);
    println!(
        "Average duration per repetition: {:?}",
        duration / num_repetitions
    );
}
