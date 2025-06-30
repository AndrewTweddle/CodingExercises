fn main() {
    solve_and_print_solution_and_time_more_runs_without_printing(solve, 1000)
}

type FigurateNumbersCycle = [u16; 6];

fn solve() -> u16 {
    // We will start with the octagonal numbers (since the cycle can start anywhere),
    // and they should be less numerous, so this should give fewer branches in the search tree.

    // Generate a graph of pairs of digits with edges linking to the next 2 digits of each 4 digit
    // figurate number. The edges are "coloured" with the figurate number type.
    let graph_nodes = get_graph_nodes_and_edges();

    // Octagonal numbers are not included in the graph, but they form the first level of the search:
    let min_sum: Option<u16> = (1_u16..)
        .map(|n| figurate(8, n))
        .skip_while(|&fig_num| fig_num < 1010)
        .take_while(|&fig_num| fig_num < 10_000)
        .filter_map(|fig_num| {
            let expected_final_pair = (fig_num / 100) as u8;
            if expected_final_pair < 10 {
                None
            } else {
                find_min_sum_of_cyclic_figurate_numbers(
                    fig_num as u128,
                    0,
                    0,
                    &graph_nodes,
                    expected_final_pair,
                )
            }
        })
        .min();

    // There should be exactly one solution, since the problem statement implies this...
    min_sum.unwrap()
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
    T: Copy + Default + PartialEq,
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

// Each pair of digits will be a node in a graph. Store the edges for each such node.
// Each edge will link a figurate number's higher 2 digits with its lower 2 digits.
// Additionally, each edge will have a "colour" - the index of the figurate number type
// (0 for triangular, 1 for pentagonal, up to 5 for heptagonal).
// The octagonal numbers are not included in the graph, but they will seed the search.
// Note that each figurate number type will have at most 2 edges per pair of digits,
// since Triangular(48) = 1176 and Triangular(50) = 1275, 34^2 - 32^2 = 1156 - 1024 > 100.
// So 5 figurate number types x at most 2 edges per node => 10 edges per node are enough.
type Graph = [InlineVec<Edge, 10>; 100];

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, Hash)]
struct Edge {
    figurate_number_type: u8,
    lowest_2_digits: u8,
}

impl Edge {
    fn new(figurate_number_type: u8, lowest_2_digits: u8) -> Self {
        Self {
            figurate_number_type,
            lowest_2_digits,
        }
    }
}

fn get_graph_nodes_and_edges() -> Graph {
    let mut first2_to_last2: Graph = [InlineVec::new(); 100];
    for fig in 0..5 {
        for n in 1_u16.. {
            let fig_num = figurate(fig + 3, n);
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

            first2_to_last2[hundreds].push(Edge::new(fig as u8, tens_and_ones));
        }
    }
    first2_to_last2
}

fn figurate(fig: u16, n: u16) -> u16 {
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

// Instead of storing the various pairs of digits, use 128 bits to store all digits.
// 6 numbers x 4 decimal digits = 24 decimal digits, or 10 ^ 24.
// And 10 ^ 3 < 2 ^ 10. So 10 ^ 24 = (10 ^ 3) ^ 8 < (2 ^ 10) ^ 8 = 2 ^ 80 < 2 ^ 128.
fn find_min_sum_of_cyclic_figurate_numbers(
    encoded_digits: u128,
    level: u8,
    fig_types_used: u8,
    graph_nodes: &Graph,
    expected_final_pair: u8,
) -> Option<u16> {
    if level == 5 {
        convert_encoded_digits_to_cycle_sum_if_valid(encoded_digits, expected_final_pair)
    } else {
        graph_nodes[(encoded_digits % 100) as usize]
            .iter()
            .filter_map(|edge| {
                let fig_num_type_mask = 1 << edge.figurate_number_type;
                if fig_types_used & fig_num_type_mask != 0 {
                    None
                } else {
                    let new_encoded_digits = 100 * encoded_digits + edge.lowest_2_digits as u128;
                    find_min_sum_of_cyclic_figurate_numbers(
                        new_encoded_digits,
                        level + 1,
                        fig_types_used | fig_num_type_mask,
                        graph_nodes,
                        expected_final_pair,
                    )
                }
            })
            .min()
    }
}

fn convert_encoded_digits_to_cycle_sum_if_valid(
    digits: u128,
    expected_final_pair: u8,
) -> Option<u16> {
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
        let cycle_sum: u16 = numbers.iter().map(|&n| n).sum();
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
