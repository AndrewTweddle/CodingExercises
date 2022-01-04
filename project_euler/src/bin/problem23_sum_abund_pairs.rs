use std::time::Instant;

const NUM_REPETITIONS: u32 = 10;
const MIN_ABUNDANT_NUM: usize = 12;
const MAX_NON_ABUNDANT_SUM: usize = 28123;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let abunds: Vec<usize> = (MIN_ABUNDANT_NUM..=MAX_NON_ABUNDANT_SUM)
            .into_iter()
            .filter(|&n| sum_of_divs(n) > n)
            .collect();

        let mut is_abund_sum = [false; MAX_NON_ABUNDANT_SUM + 1];
        abunds.iter().enumerate().for_each(|(index, &abund_num)| {
            let higher_abund_nums = &abunds[index..];
            higher_abund_nums.iter().for_each(|&higher_abund_num| {
                let abund_sum = abund_num + higher_abund_num;
                if abund_sum <= MAX_NON_ABUNDANT_SUM {
                    is_abund_sum[abund_sum] = true;
                };
            });
        });

        let sum_non_abund_sums = is_abund_sum
            .iter()
            .enumerate()
            .filter_map(|(non_abund_sum, &is_ab_sum)| {
                if !is_ab_sum {
                    Some(non_abund_sum)
                } else {
                    None
                }
            })
            .sum::<usize>();
        if rep == 0 {
            println!("Sum of non-abundant sums: {}", sum_non_abund_sums);
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn sum_of_divs(n: usize) -> usize {
    // The following trick checks around sqrt(n) instead of (n-2) divisors.
    // This makes the biggest difference in performance.
    (2..n)
        .into_iter()
        .take_while(|i| i * i <= n)
        .map(|div| {
            if n % div == 0 {
                let other_div = n / div;
                if div == other_div {
                    div
                } else {
                    div + other_div
                }
            } else {
                0
            }
        })
        .sum::<usize>()
        + 1
}

#[cfg(test)]
mod tests {
    use super::sum_of_divs;

    #[test]
    fn test_sum_of_divs() {
        assert_eq!(sum_of_divs(12), 1 + 2 + 6 + 3 + 4);
    }

    #[test]
    fn test_sum_of_divs_of_square_num() {
        assert_eq!(sum_of_divs(25), 1 + 5);
    }
}
