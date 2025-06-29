fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000)
}

const FIVE_FACTORIAL: u8 = 2 * 3 * 4 * 5;

type FigurateNumbersCycle = [u16; 6];

fn solve() -> u64 {
    // We will always start with the triangular numbers (since the cycle can start anywhere),
    // and they should be more numerous, so avoid having to look them up in a hash table.

    // Store lookups from the first 2 to the last 2 digits, for each other type of figurate number
    let lookups = get_digit_lookups_by_figurate_number_type();

    // Consider all possible orders of the figurate numbers by figurate number type
    let permutations = get_permutations();

    let sum: Option<u64> = (1_u64..)
        .map(|n| figurate(3, n))
        .skip_while(|&fig_num| fig_num < 1010)
        .find_map(|fig_num| {
            let expected_final_pair = (fig_num / 100) as u8;
            if expected_final_pair < 10 {
                None
            } else {
                permutations.iter().find_map(|permutation| {
                    find_sum_of_cyclic_figurate_numbers_for_permutation(
                        fig_num as u128,
                        permutation,
                        &lookups,
                        expected_final_pair,
                    )
                })
            }
        });

    // There should be exactly one solution, since the problem statement implies this...
    sum.unwrap()
}

#[derive(Debug, Copy, Clone)]
struct InlineVec<T, const N: usize>
where
    T: Copy + Default,
{
    items: [T; N],
    count: usize,
}

impl<T, const N: usize> InlineVec<T, N>
where
    T: Copy + Default,
{
    fn new() -> Self {
        Self {
            items: [T::default(); N],
            count: 0,
        }
    }

    fn push(&mut self, item: T) {
        if self.count == N {
            panic!("InlineVec is full");
        }
        self.items[self.count] = item;
        self.count += 1;
    }

    fn iter(&self) -> InlineVecIter<T, N> {
        InlineVecIter {
            inline_vec: *self,
            index: 0,
        }
    }
}

struct InlineVecIter<T, const N: usize>
where
    T: Copy + Default,
{
    inline_vec: InlineVec<T, N>,
    index: usize,
}

impl<T: Copy + Default, const N: usize> Iterator for InlineVecIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inline_vec.count {
            None
        } else {
            let item = self.inline_vec.items[self.index];
            self.index += 1;
            Some(item)
        }
    }
}

impl<T: Copy + Default, const N: usize> IntoIterator for InlineVec<T, N> {
    type Item = T;
    type IntoIter = InlineVecIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inline_vec: self,
            index: 0,
        }
    }
}

type LookupByDigitPair = [InlineVec<u8, 2>; 100];

fn get_digit_lookups_by_figurate_number_type() -> Vec<LookupByDigitPair> {
    let lookups: Vec<LookupByDigitPair> = (0..5)
        .map(|fig| {
            let mut first2_to_last2: LookupByDigitPair = [InlineVec::new(); 100];
            for n in 1_u64.. {
                let fig_num = figurate(fig + 4, n);
                if fig_num < 1010 {
                    continue;
                }
                if fig_num > 10_000 {
                    break;
                }
                let tens_and_ones = (fig_num % 100) as u8;
                if tens_and_ones < 10 {
                    continue;
                }
                let hundreds = (fig_num / 100) as usize;

                first2_to_last2[hundreds].push(tens_and_ones);
            }
            first2_to_last2
        })
        .collect();
    lookups
}

#[inline(always)]
fn figurate(fig: u64, n: u64) -> u64 {
    match fig {
        3 => n * (n + 1) / 2,
        4 => n * n,
        5 => n * (3 * n - 1) / 2,
        6 => n * (2 * n - 1),
        7 => n * (5 * n - 3) / 2,
        8 => n * (3 * n - 2),
        _ => panic!("Invalid figurate number"),
    }
}

fn get_permutations() -> Vec<[u8; 5]> {
    let mut permutations: Vec<[u8; 5]> = Vec::with_capacity(FIVE_FACTORIAL as usize);
    for permutation in 0..FIVE_FACTORIAL {
        let mut indices: Vec<u8> = vec![0, 1, 2, 3, 4];
        let mut p = permutation;
        for i in 0..4 {
            let base = 5 - i;
            let index = p % base;
            p /= base;
            if index != 0 {
                let permutation_to_remove = indices.remove((i + index) as usize);
                indices.insert(i as usize, permutation_to_remove);
            }
        }
        let perm_indices: [u8; 5] = [indices[0], indices[1], indices[2], indices[3], indices[4]];
        permutations.push(perm_indices);
    }
    permutations
}

// Instead of storing the various pairs of digits, use 128 bits to store all digits.
// 6 numbers x 4 decimal digits = 24 decimal digits, or 10 ^ 24.
// And 10 ^ 3 < 2 ^ 10. So 10 ^ 24 = (10 ^ 3) ^ 8 < (2 ^ 10) ^ 8 = 2 ^ 80 < 2 ^ 128.
fn find_sum_of_cyclic_figurate_numbers_for_permutation(
    encoded_digits: u128,
    permutation: &[u8],
    lookups: &[LookupByDigitPair],
    expected_final_pair: u8,
) -> Option<u64> {
    if permutation.is_empty() {
        convert_encoded_digits_to_cycle_sum_if_valid(encoded_digits, expected_final_pair)
    } else {
        let (&index, rem_permutation) = permutation.split_first().unwrap();

        lookups[index as usize][(encoded_digits % 100) as usize]
            .iter()
            .find_map(|next_digits| {
                let new_encoded_digits = 100 * encoded_digits + next_digits as u128;
                find_sum_of_cyclic_figurate_numbers_for_permutation(
                    new_encoded_digits,
                    rem_permutation,
                    lookups,
                    expected_final_pair,
                )
            })
    }
}

#[inline(always)]
fn convert_encoded_digits_to_cycle_sum_if_valid(
    digits: u128,
    expected_final_pair: u8,
) -> Option<u64> {
    // The last 2 digits must match the expected final pair
    if digits % 100 != expected_final_pair as u128 {
        return None;
    }

    let mut rem_digits = digits;
    let mut numbers: FigurateNumbersCycle = [0; 6];

    // The 6 figurate numbers must be unique...
    let mut dup_found = false;
    'num_loop: for i in 0..6 {
        numbers[i] = (rem_digits % 10_000) as u16;
        rem_digits /= 100;
        for j in 0..i {
            if numbers[j] == numbers[i] {
                dup_found = true;
                break 'num_loop;
            }
        }
    }
    if dup_found {
        None
    } else {
        let cycle_sum: u64 = numbers.iter().map(|&n| n as u64).sum();
        Some(cycle_sum)
    }
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
