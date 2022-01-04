use std::time::Instant;

const NUM_REPETITIONS: u32 = 10;
const MIN_ABUNDANT_NUM: usize = 12;
const MAX_NON_ABUNDANT_SUM: usize = 28123;

fn main() {
    let start = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        let mut is_abund = [false; MAX_NON_ABUNDANT_SUM + 1];
        for i in MIN_ABUNDANT_NUM..=MAX_NON_ABUNDANT_SUM {
            let i_sum_divs = (1..i)
                .into_iter()
                .filter(|&div| i % div == 0)
                .sum::<usize>();
            if i_sum_divs > i {
                is_abund[i] = true;
            }
        }

        let non_abund_sum = (1..=MAX_NON_ABUNDANT_SUM)
            .into_iter()
            .filter(|&i| !is_abund_sum(i, &is_abund))
            .sum::<usize>();

        if rep == 0 {
            println!("Sum of non-abundant sum: {}", non_abund_sum);
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS)
}

fn is_abund_sum(n: usize, is_abund: &[bool; MAX_NON_ABUNDANT_SUM + 1]) -> bool {
    if n < 2 * MIN_ABUNDANT_NUM {
        return false;
    }
    for i in MIN_ABUNDANT_NUM..=(n - MIN_ABUNDANT_NUM) {
        if is_abund[i] {
            let j = n - i;
            if i == j {
                return true;
            } else if is_abund[j] {
                return true;
            }
        }
    }
    false
}
