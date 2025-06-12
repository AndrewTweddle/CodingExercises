fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

// We can search for the first new solution, as the problem statement says there is only one other.
// But to compare performance of different algorithms, it is better to search exhaustively.
// Otherwise an algorithm may seem better simply because it searched in a luckier order.
const EXHAUSTIVE_SEARCH: bool = true;

// Multi-digit prime numbers always end in 1, 3, 7 or 9, otherwise they are even or divisible by 5.
const LOWEST_PRIME_DIGITS: [u8; 4] = [1, 3, 7, 9];

fn solve() -> u64 {
    let mut solution = 0;

    // Let a = [a3, a2, a1, a0] be the first number with its 4 digits,
    // Let b = [b3, b2, b1, b0] be the second number with its 4 digits,
    // Let c = [c3, c2, c1, c0] be the third number with its 4 digits,
    // Let d = [d3, d2, d1, d0] be the difference (d = b-a = c-b) with its 4 digits
    let mut digit_count = [0_u8; 10];
    let mut bits_set = 0_u16;

    // Iterate over possible units digits for the 3 numbers and their difference
    for a0 in LOWEST_PRIME_DIGITS {
        try_inc_digit_count(a0, &mut digit_count, &mut bits_set);
        for d0 in 0..10 {
            let b0 = (a0 + d0) % 10;
            if !LOWEST_PRIME_DIGITS.contains(&b0) {
                continue;
            }
            let c0 = (b0 + d0) % 10;
            if !LOWEST_PRIME_DIGITS.contains(&c0) {
                continue;
            }
            try_inc_digit_count(b0, &mut digit_count, &mut bits_set);
            try_inc_digit_count(c0, &mut digit_count, &mut bits_set);

            // Iterate over possible tens digits for the 3 numbers and their difference
            for a1 in 0..10 {
                try_inc_digit_count(a1, &mut digit_count, &mut bits_set);
                let a = (10 * a1 + a0) as u16;
                for d1 in 0..10 {
                    let d = (d1 * 10 + d0) as u16;
                    let b = a + d;
                    let b1 = ((b / 10) % 10) as u8;
                    if !try_inc_digit_count(b1, &mut digit_count, &mut bits_set) {
                        continue;
                    }

                    let c = b + d;
                    let c1 = ((c / 10) % 10) as u8;
                    if try_inc_digit_count(c1, &mut digit_count, &mut bits_set) {
                        // Calculate possible hundreds digits
                        for a2 in 0..10 {
                            if !try_inc_digit_count(a2, &mut digit_count, &mut bits_set) {
                                continue;
                            }
                            let a = three_digits_to_u16(a0, a1, a2);

                            for d2 in 0..10 {
                                let d = three_digits_to_u16(d0, d1, d2);
                                let b = a + d;
                                let b2 = ((b / 100) % 10) as u8;
                                if !try_inc_digit_count(b2, &mut digit_count, &mut bits_set) {
                                    continue;
                                }

                                let c = b + d;
                                let c2 = ((c / 100) % 10) as u8;
                                if try_inc_digit_count(c2, &mut digit_count, &mut bits_set) {
                                    // Calculate possible thousands digits
                                    for a3 in 1..10 {
                                        if !try_inc_digit_count(a3, &mut digit_count, &mut bits_set)
                                        {
                                            continue;
                                        }
                                        let a = four_digits_to_u16(a0, a1, a2, a3);

                                        for d3 in 0..10 {
                                            let d = four_digits_to_u16(d0, d1, d2, d3);
                                            if d == 0 {
                                                continue;
                                            }

                                            let b = a + d;
                                            if b >= 10_000 {
                                                break;
                                            }

                                            let c = b + d;
                                            if c >= 10_000 {
                                                break;
                                            }

                                            let b3 = (b / 1000) as u8;
                                            if !try_inc_digit_count(
                                                b3,
                                                &mut digit_count,
                                                &mut bits_set,
                                            ) {
                                                continue;
                                            }

                                            let c3 = (c / 1000) as u8;
                                            if try_inc_digit_count(
                                                c3,
                                                &mut digit_count,
                                                &mut bits_set,
                                            ) {
                                                // Now check if they are really permutations
                                                // (since digit counts might differ)...
                                                let code_a = perm_code(a0, a1, a2, a3);
                                                if perm_code(b0, b1, b2, b3) == code_a
                                                    && perm_code(c0, c1, c2, c3) == code_a
                                                    && is_prime(a)
                                                    && is_prime(b)
                                                    && is_prime(c)
                                                    && (a != 1487 || b != 4817)
                                                {
                                                    // Solution found!
                                                    solution = (a as u64) * 100_000_000
                                                        + (b as u64) * 10_000
                                                        + (c as u64);
                                                    if !EXHAUSTIVE_SEARCH {
                                                        // use the first solution found,
                                                        // as the problem statement says it's unique
                                                        return solution;
                                                    }
                                                }
                                                dec_digit_count(
                                                    c3,
                                                    &mut digit_count,
                                                    &mut bits_set,
                                                );
                                            }
                                            dec_digit_count(b3, &mut digit_count, &mut bits_set);
                                        }
                                        dec_digit_count(a3, &mut digit_count, &mut bits_set);
                                    }
                                    dec_digit_count(c2, &mut digit_count, &mut bits_set);
                                }
                                dec_digit_count(b2, &mut digit_count, &mut bits_set);
                            }
                            dec_digit_count(a2, &mut digit_count, &mut bits_set);
                        }
                        dec_digit_count(c1, &mut digit_count, &mut bits_set);
                    }
                    dec_digit_count(b1, &mut digit_count, &mut bits_set);
                }
                dec_digit_count(a1, &mut digit_count, &mut bits_set);
            }
            dec_digit_count(c0, &mut digit_count, &mut bits_set);
            dec_digit_count(b0, &mut digit_count, &mut bits_set);
        }
        dec_digit_count(a0, &mut digit_count, &mut bits_set);
    }

    if EXHAUSTIVE_SEARCH {
        return solution;
    }
    panic!("No solution found")
}

#[inline]
fn try_inc_digit_count(digit: u8, digit_count: &mut [u8], bits_set: &mut u16) -> bool {
    let new_bits_set = *bits_set | (1 << digit);

    // Since a, b and c are 4 digit numbers, and permutations of one another,
    // there can be at most 4 unique digits in total between a, b and c
    if new_bits_set.count_ones() > 4 {
        return false;
    }
    digit_count[digit as usize] += 1;
    *bits_set = new_bits_set;
    true
}

#[inline]
fn dec_digit_count(digit: u8, digit_count: &mut [u8], bits_set: &mut u16) {
    digit_count[digit as usize] -= 1;
    if digit_count[digit as usize] == 0 {
        *bits_set ^= 1 << digit;
    }
}

#[inline]
fn three_digits_to_u16(units: u8, tens: u8, hundreds: u8) -> u16 {
    100 * hundreds as u16 + 10 * tens as u16 + units as u16
}

#[inline]
fn four_digits_to_u16(units: u8, tens: u8, hundreds: u8, thousands: u8) -> u16 {
    1000 * thousands as u16 + 100 * hundreds as u16 + 10 * tens as u16 + units as u16
}

#[inline]
fn is_prime(n: u16) -> bool {
    // n is not divisible by 2 or 5, since its last digit is 1, 3, 7 or 9
    if n % 3 == 0 {
        return false;
    }

    // So n's prime factors must be of the form 6k +/- 1.
    for k in 1.. {
        let factor = 6 * k - 1;
        if n % factor == 0 {
            return false;
        }

        let factor = 6 * k + 1;
        if n % factor == 0 {
            return false;
        }

        if factor * factor >= n {
            return true;
        }
    }
    true
}

#[inline]
fn perm_code(digit0: u8, digit1: u8, digit2: u8, digit3: u8) -> u32 {
    (1 << (3 * digit0 as u32))
        + (1 << (3 * digit1 as u32))
        + (1 << (3 * digit2 as u32))
        + (1 << (3 * digit3 as u32))
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
