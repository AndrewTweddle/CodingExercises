fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000);
}

// All primes apart from 2 and 3 are of the form 6n +/- 1.
// If we want to find 3 primes (all 4 digits long) in an arithmetic sequence,
// then all 3 will be of the form 6n+1, or all 3 will be of the form 6n-1.
// Why? Well suppose i, j and k are the multiples of 6 in this representation of the 3 numbers.
// And suppose that r, s and t are the offsets (1 or -1) for each of them.
// Then: (6j+s) - (6i+r) = (6k+t) - (6j+s)
// Hence: 6k + t = 2(6j+s) - (6i+r) = 6(2j-i) + (2s-r)
// If t = 1, then s = 1, r = 1.
// If t = -1, then s = -1, r = -1.
// Either way, s = t = r
//
// Also note that k = 2j-i
//
// Also consider the range of i, j and k.
// 167x6=1002, 1666x6 = 9996, so 167 <= i, j, k <= 1666
// i < j < k

const MIN_MULT_OF_6: i16 = 167;
const MAX_MULT_OF_6: i16 = 1666;

// We can search for the first new solution, as the problem statement says there is only one other.
// But to compare performance of different algorithms, it is better to search exhaustively.
// Otherwise an algorithm may seem better simply because it searched in a luckier order.
const EXHAUSTIVE_SEARCH: bool = true;

fn solve() -> u64 {
    let mut solution: u64 = 0;
    for offset in [-1, 1_i16] {
        for i in MIN_MULT_OF_6..(MAX_MULT_OF_6 - 1) {
            let n1 = (6 * i + offset) as u16;
            let code1 = perm_code(n1);
            for j in (i + 1)..MAX_MULT_OF_6 {
                let k = 2 * j - i;
                if k > MAX_MULT_OF_6 {
                    break; // since higher values of j will increase k
                }
                let n2 = (6 * j + offset) as u16;
                let code2 = perm_code(n2);
                if code2 != code1 {
                    continue;
                }
                let n3 = (6 * k + offset) as u16;
                let code3 = perm_code(n3);
                if code3 != code1 {
                    continue;
                }

                if is_prime(n1) && is_prime(n2) && is_prime(n3) && (n1 != 1487 || n2 != 4817) {
                    solution = (n1 as u64) * 100_000_000 + (n2 as u64) * 10_000 + (n3 as u64);
                    if !EXHAUSTIVE_SEARCH {
                        // return first solution found, since the problem statement says it's unique
                        return solution;
                    }
                }
            }
        }
    }

    if EXHAUSTIVE_SEARCH {
        return solution;
    }
    panic!("No solution found")
}

#[inline]
fn perm_code(n: u16) -> u32 {
    let mut code: u32 = 0;
    for d in digits(n) {
        // Use 3 bits per decimal digit to count the # of that digit in n.
        // With 10 possible digits this uses 3 * 10 bits to form a unique number per set of digits.
        code += 1 << (3 * d);
    }
    code
}

#[inline]
fn digits(mut n: u16) -> [u8; 4] {
    let units = n % 10;
    n /= 10;
    let tens = n % 10;
    n /= 10;
    let hundreds = n % 10;
    let thousands = n / 10;
    [thousands as u8, hundreds as u8, tens as u8, units as u8]
}

#[inline]
fn is_prime(n: u16) -> bool {
    // n is of the form 6n +/- 1, so is not divisible by 2 or 3.
    // Check for factors of the form 6k +/- 1, since all other primes must fit this form.
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
