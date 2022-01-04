use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let sum_of_diagonals = get_sum(1001);
        if rep == 0 {
            println!("Sum of diagonals: {}", sum_of_diagonals);
        }
    }

    let duration = start.elapsed() / NUM_REPETITIONS;
    println!("Avg duration of {} runs: {:?}", NUM_REPETITIONS, duration);
}

// Assumed: n is odd
fn get_sum(n: u32) -> u32 {
    if n == 1 {
        1
    } else {
        let prev_max = (n - 2) * (n - 2);
        let prev_sum = get_sum(n - 2);
        let diff_of_sums = 4 * prev_max + 10 * (n - 1);
        prev_sum + diff_of_sums
    }
}

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn test_1x1() {
        assert_eq!(get_sum(1), 1);
    }

    #[test]
    fn test_3x3() {
        assert_eq!(get_sum(3), 25);
    }

    #[test]
    fn test_5x5() {
        assert_eq!(get_sum(5), 101);
    }
}
