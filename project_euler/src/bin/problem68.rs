use std::cmp::PartialEq;
use std::iter;

fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1_000_000)
}

// Order the positions within the layout using the following indices:
// Indices 0 to 4 are for the outer ring, starting from the top left, in clockwise order.
// Indices 5 to 9 are for the inner ring, starting from the top left, in clockwise order.
// 
// Note: We will impose the additional constraint that the number in position 0 must be 
// the smallest of the 5 outer numbers, as this simplifies the algorithm.
type Layout = [Option<u8>; 10];

// It's more convenient to work with lines than positions. Define a lookup from lines to positions
// within those lines. Index both in the order that their numbers appear in the 16-digit solution.
type LineIndices = [usize; 3];
const LINE_INDICES: [LineIndices; 5] = [[0, 5, 6], [1, 6, 7], [2, 7, 8], [3, 8, 9], [4, 9, 5]];

// Define the search strategy as a sequence of instructions, one per layer of the search tree.
// This gives flexibility to refine the search strategy experimentally until it is good enough.
const INSTRUCTIONS: [Instruction; 16] = [
    // We want the highest digit in the 16-digit number to be as large as possible. This tries to
    // find the best solution sooner so we can aggressively filter out branches that cannot beat it.
    Instruction::fill_outermost_position_of_line(0, false),
    Instruction::fill_middle_position_of_line(0, false),
    // Choosing the magic number sooner seems wise, as we can use it to calculate the third
    // number in each line, instead of iterating over all remaining numbers.
    Instruction::TryMagicNumbersInDescendingOrder,
    Instruction::fill_innermost_position_of_line(0, true),
    Instruction::fill_outermost_position_of_line(1, false),
    Instruction::AppendDigitsForPreviousPositionToSolution { position: 6 },
    Instruction::fill_innermost_position_of_line(1, true),
    Instruction::fill_outermost_position_of_line(2, false),
    Instruction::AppendDigitsForPreviousPositionToSolution { position: 7 },
    Instruction::fill_innermost_position_of_line(2, true),
    Instruction::fill_outermost_position_of_line(3, false),
    Instruction::AppendDigitsForPreviousPositionToSolution { position: 8 },
    Instruction::fill_innermost_position_of_line(3, true),
    Instruction::fill_outermost_position_of_line(4, true),
    Instruction::AppendDigitsForPreviousPositionToSolution { position: 9 },
    Instruction::AppendDigitsForPreviousPositionToSolution { position: 5 },
];

enum Instruction {
    /// Try larger magic numbers first. This is to encourage the search to try larger digits
    /// in the higher digit positions of the first solution found, since that solution
    /// will be used to prune out later branches that cannot match the best solution.
    TryMagicNumbersInDescendingOrder,
    FillPosition {
        /// The index of the position in the layout that is being filled.
        position: usize,

        /// The sequence of numbers that can be used to fill the position.
        numbers_to_try: NumbersToTry,
    },
    /// The numbers in the inner ring will appear in multiple positions in the 16-digit solution,
    /// so this instruction updates the current solution for previously placed numbers.
    /// Also, check whether this makes it impossible for the current solution to ever become
    /// better than the best solution. If it can't, prune this branch of the search immediately.
    AppendDigitsForPreviousPositionToSolution { position: usize },
}

impl Instruction {
    const fn fill_outermost_position_of_line(line: usize, completes_line: bool) -> Self {
        Self::fill_position_on_line(line, 0, completes_line)
    }

    const fn fill_middle_position_of_line(line: usize, completes_line: bool) -> Self {
        Self::fill_position_on_line(line, 1, completes_line)
    }

    const fn fill_innermost_position_of_line(line: usize, completes_line: bool) -> Self {
        Self::fill_position_on_line(line, 2, completes_line)
    }

    const fn fill_position_on_line(
        line: usize,
        index_in_line: usize,
        completes_line: bool,
    ) -> Self {
        Self::FillPosition {
            position: LINE_INDICES[line][index_in_line],
            numbers_to_try: if completes_line {
                NumbersToTry::NumberThatMakesLineSumAddUpToMagicNumber { line }
            } else {
                NumbersToTry::HighestExceptTenDownToLowestThenTenIfInOutermostPosition
            },
        }
    }
}

/// 1. We want to choose the most promising numbers first, to be able to quickly prune search
///    branches that cannot match the best solution. So we start with the highest *digit*, 9,
///    go down in descending order, and end with 10, because it has two of the lowest digits.
/// 2. But we only want to include 10 as a candidate when it is in the outermost ring of positions.
/// 3. We also want to ensure that the very first number is smaller than the other 4 outer numbers.
/// 4. Also, once two of the 3 digits in a line have been chosen, the 3rd digit can be calculated
///    so that the line will sum to the magic number. Then we just need to see if it is available.
///    This way we won't need to iterate over all remaining numbers in that 3rd position.
/// 
/// These strategies are partially encoded in the NumbersToTry enum.
enum NumbersToTry {
    HighestExceptTenDownToLowestThenTenIfInOutermostPosition,
    NumberThatMakesLineSumAddUpToMagicNumber {
        /// The line that the position belongs to. This is used to calculate the line sum,
        /// which is used to calculate the number that makes the line sum add up to the magic number.
        line: usize,
    },
}

fn solve() -> u64 {
    let mut best_solution: Option<u64> = None;
    let mut curr_layout: Layout = [None; 10];

    // We want to filter to only the first D digits in the best solution, when comparing to the
    // current solution, where the current solution has D digits. So keep track of the power of 10
    // to divide the best solution by, to make it comparable with the current solution...
    let initial_pow_of_10: u64 = 10_u64.pow(16);

    apply_instructions(
        &mut curr_layout,
        0, // empty curr_solution
        &mut best_solution,
        0, // dummy magic_number, since not yet chosen
        0, // no numbers have been used yet
        initial_pow_of_10,
        &INSTRUCTIONS,
    );

    if let Some(solution) = best_solution {
        solution
    } else {
        panic!("No solution found!");
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Outcome {
    /// This outcome needs to be treated specially when it propagates up the search tree,
    /// allowing short-circuiting of numbers even smaller than the one that was too small.
    TooLowToReachMagicNumber,

    /// All other outcomes are treated the same, whether a solution is found or not
    Other,
}

fn apply_instructions(
    curr_layout: &mut Layout,
    mut curr_solution: u64,
    best_solution: &mut Option<u64>,
    magic_number: u8,
    used_numbers: u16,
    mut pow_of_10: u64,
    instructions: &[Instruction],
) -> Outcome {
    if instructions.is_empty() {
        // Update best_solution if curr_solution is the new best solution
        if let Some(best_value) = best_solution {
            if curr_solution > *best_value {
                *best_solution = Some(curr_solution);
            }
        } else {
            *best_solution = Some(curr_solution);
        }
        return Outcome::Other;
    }

    let instruction = &instructions[0];
    match instruction {
        Instruction::TryMagicNumbersInDescendingOrder => {
            try_possible_magic_numbers_in_descending_order(
                curr_layout,
                curr_solution,
                best_solution,
                used_numbers,
                pow_of_10,
                &instructions,
            )
        }

        Instruction::FillPosition {
            position,
            numbers_to_try,
        } => match numbers_to_try {
            NumbersToTry::NumberThatMakesLineSumAddUpToMagicNumber { line } => {
                attempt_number_that_makes_line_sum_add_up_to_magic_number(
                    *position,
                    *line,
                    curr_layout,
                    curr_solution,
                    best_solution,
                    magic_number,
                    used_numbers,
                    pow_of_10,
                    &instructions[1..],
                )
            }

            NumbersToTry::HighestExceptTenDownToLowestThenTenIfInOutermostPosition => {
                try_multiple_numbers_in_position(
                    *position,
                    curr_layout,
                    curr_solution,
                    best_solution,
                    magic_number,
                    used_numbers,
                    pow_of_10,
                    &instructions,
                )
            }
        },

        Instruction::AppendDigitsForPreviousPositionToSolution { position } => {
            let number = curr_layout[*position]
                .expect("The layout should already contain a number in position {position}");
            append_digits_of_number_to_solution(number, &mut curr_solution, &mut pow_of_10);
            if incapable_of_being_higher_than_best_solution(curr_solution, best_solution, pow_of_10)
            {
                return Outcome::Other;
            }
            apply_instructions(
                curr_layout,
                curr_solution,
                best_solution,
                magic_number,
                used_numbers,
                pow_of_10,
                &instructions[1..],
            )
        }
    }
}

fn try_possible_magic_numbers_in_descending_order(
    curr_layout: &mut Layout,
    curr_solution: u64,
    best_solution: &mut Option<u64>,
    used_numbers: u16,
    pow_of_10: u64,
    instructions: &&[Instruction],
) -> Outcome {
    let n = curr_layout[0].unwrap(); // The first number chosen (i.e., in position 0 of the layout) 
    let min_outer_sum = 4 * n + 16;  // n, n+1, n+2, n+3, 10
    let max_outer_sum = n + 34; /* n + 7 + 8 + 9 + 10 */
    
    // The numbers from 1 to 10 add up to 55, so we can determine the inner sums too.
    // Numbers in the inner ring contribute to 2 lines, but outer numbers contribute to 1 line.
    let min_sum_of_lines = max_outer_sum + 2 * (55 - max_outer_sum);
    let max_sum_of_lines = min_outer_sum + 2 * (55 - min_outer_sum);

    // The sum of the lines is 5 times the magic number:
    let min_magic_number = min_sum_of_lines.div_ceil(5);  // Round up
    let max_magic_number = max_sum_of_lines / 5;
    
    for magic_number in (min_magic_number..=max_magic_number).rev() {
        apply_instructions(
            curr_layout,
            curr_solution,
            best_solution,
            magic_number,
            used_numbers,
            pow_of_10,
            &instructions[1..],
        );
    }
    Outcome::Other
}

fn attempt_number_that_makes_line_sum_add_up_to_magic_number(
    position: usize,
    line: usize,
    curr_layout: &mut Layout,
    curr_solution: u64,
    best_solution: &mut Option<u64>,
    magic_number: u8,
    used_numbers: u16,
    pow_of_10: u64,
    instructions: &[Instruction],
) -> Outcome {
    let line_sum = get_line_sum(line, curr_layout);
    if line_sum >= magic_number {
        return Outcome::Other;
    }
    let number = magic_number - line_sum;
    if number > 10 {
        return Outcome::TooLowToReachMagicNumber;
    }

    // 10 is not allowed in inner positions; otherwise the number will have 17 digits,
    // not 16. Positions 0 to 4 are outer positions, but 5 to 9 are inner positions.
    if position >= 5 && number == 10 {
        return Outcome::Other;
    }

    attempt_number_in_position(
        number,
        position,
        curr_layout,
        curr_solution,
        best_solution,
        magic_number,
        used_numbers,
        pow_of_10,
        instructions, // NB: The calling code has already filtered out the current instruction
    )
}

fn get_line_sum(line: usize, curr_layout: &Layout) -> u8 {
    LINE_INDICES[line]
        .iter()
        .filter_map(|&i| curr_layout[i])
        .sum::<u8>()
}

fn try_multiple_numbers_in_position(
    position: usize,
    curr_layout: &mut Layout,
    curr_solution: u64,
    best_solution: &mut Option<u64>,
    magic_number: u8,
    used_numbers: u16,
    pow_of_10: u64,
    instructions: &&[Instruction],
) -> Outcome {
    if position <= 4 {
        // In the outermost ring:
        let range_except_ten = if position == 0 {
            // The number in position 0 must be the smallest of the 5 outer numbers.
            // So it can't be any greater than 6.
            1..=6
        } else {
            // We will try 10 last, but we need to only consider numbers greater than the very first
            let min_greater_than_first = curr_layout[0].unwrap() + 1;
            min_greater_than_first..=9
        };

        // If the starting number in a line is too small, it will be impossible to reach the magic
        // number with the last number in the line. So, we shouldn't try numbers smaller than it.
        // The following variable tracks this minimum number to try.
        let mut min_number_to_try = 0;

        // This is a position in the outer ring, so 10 can be used:
        for number in range_except_ten.rev().chain(iter::once(10)) {
            if number < min_number_to_try {
                continue;
            }
            let outcome = attempt_number_in_position(
                number,
                position,
                curr_layout,
                curr_solution,
                best_solution,
                magic_number,
                used_numbers,
                pow_of_10,
                &instructions[1..],
            );
            if outcome == Outcome::TooLowToReachMagicNumber {
                min_number_to_try = number + 1;
            }
        }
    } else {
        for number in (1..=9).rev() {
            let outcome = attempt_number_in_position(
                number,
                position,
                curr_layout,
                curr_solution,
                best_solution,
                magic_number,
                used_numbers,
                pow_of_10,
                &instructions[1..],
            );
            if outcome == Outcome::TooLowToReachMagicNumber {
                break;
            }
        }
    }

    // Don't propagate the outcome up the search tree, since this is the only
    // logic that needs special treatment depending on the outcome lower down the tree.
    // So, we don't want it propagating to a parent search node of this same type!
    Outcome::Other
}

fn attempt_number_in_position(
    number: u8,
    position: usize,
    curr_layout: &mut Layout,
    mut curr_solution: u64,
    best_solution: &mut Option<u64>,
    magic_number: u8,
    mut used_numbers: u16,
    mut pow_of_10: u64,
    instructions: &[Instruction],
) -> Outcome {
    if is_number_already_used(number, used_numbers) {
        return Outcome::Other;
    }
    append_digits_of_number_to_solution(number, &mut curr_solution, &mut pow_of_10);
    if incapable_of_being_higher_than_best_solution(curr_solution, best_solution, pow_of_10) {
        return Outcome::Other;
    }

    curr_layout[position] = Some(number);
    mark_number_as_used(number, &mut used_numbers);

    // Search for a better solution by applying the remaining instructions
    let outcome = apply_instructions(
        curr_layout,
        curr_solution,
        best_solution,
        magic_number,
        used_numbers,
        pow_of_10,
        instructions, // NB: The calling code has already filtered out the current instruction
    );

    // Undo the effect on curr_layout, since it is shared between branches
    curr_layout[position] = None;
    outcome
}

fn incapable_of_being_higher_than_best_solution(
    curr_solution: u64,
    best_solution: &mut Option<u64>,
    pow_of_10: u64,
) -> bool {
    // Make sure that the new candidate solution has a chance to beat the best solution
    if let Some(best_solution) = best_solution {
        // We only compare the first D digits of the current and best solution,
        // where D is the number of digits in the current solution.
        let best_solution_truncated = *best_solution / pow_of_10;
        if curr_solution < best_solution_truncated {
            // The current solution is worse than the best solution,
            // so we can abandon further search in this branch
            return true;
        }
    }
    false
}

fn append_digits_of_number_to_solution(number: u8, curr_solution: &mut u64, pow_of_10: &mut u64) {
    if number == 10 {
        *pow_of_10 /= 100;
        *curr_solution *= 100;
    } else {
        *pow_of_10 /= 10;
        *curr_solution *= 10;
    }
    *curr_solution += number as u64;
}

fn is_number_already_used(number: u8, used_numbers: u16) -> bool {
    used_numbers & (1 << number) != 0
}

fn mark_number_as_used(number: u8, used_numbers: &mut u16) {
    *used_numbers |= 1 << number;
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
        let total_elapsed = start_time.elapsed();
        let avg_duration = total_elapsed / repetitions;
        println!("Average duration (excl I/O) over {repetitions} further runs: {avg_duration:?}");
        println!("Total elapsed time for {repetitions} runs: {total_elapsed:?}");
    }
}
