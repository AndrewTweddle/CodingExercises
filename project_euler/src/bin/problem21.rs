use std::time::Instant;

const N: usize = 10_000;

fn main() {
    let now = Instant::now();
    let use_lookup_table = true; // This is faster by a factor of about 600!
    let num_iterations = 10;
    for rep in 0..num_iterations {
        let sum_of_amicable_numbers = if use_lookup_table {
            get_sum_of_amicable_numbers_under_using_lookup(N)
        } else {
            get_sum_of_amicable_numbers_under_directly(N)
        };
        if rep == 0 {
            println!(
                "The sum of amicable numbers under {} is {}",
                N, sum_of_amicable_numbers
            );
        }
    }

    let duration = now.elapsed();
    println!("Total duration: {:?}", duration);
    println!("Avg duration: {:?}", duration / num_iterations);
}

fn get_sum_of_amicable_numbers_under_using_lookup(max_num: usize) -> usize {
    let sums_of_proper_divisors = get_sums_of_proper_divisors(max_num - 1);
    let max_sum: usize = *sums_of_proper_divisors.iter().max().unwrap();
    let sums_of_proper_divisors = get_sums_of_proper_divisors(max_sum);
    let mut sum_of_amicable_numbers = 0;
    for i in 2..max_num {
        let sum_1 = sums_of_proper_divisors[i];
        // Ignore perfect numbers...
        if sum_1 == i {
            continue;
        }
        let sum_2 = sums_of_proper_divisors[sum_1];
        if i == sum_2 {
            sum_of_amicable_numbers += i;
        }
    }
    sum_of_amicable_numbers
}

fn get_sums_of_proper_divisors(max_num: usize) -> Vec<usize> {
    let mut sum_of_divisors = vec![0_usize; max_num + 1];
    for divisor in 1..=max_num {
        let mut factor = 2 * divisor;
        while factor <= max_num {
            sum_of_divisors[factor] += divisor;
            factor += divisor;
        }
    }
    sum_of_divisors
}

fn get_sum_of_amicable_numbers_under_directly(max_num: usize) -> usize {
    let mut sum_of_amicable_numbers = 0;
    for i in 2..max_num {
        let sum_1 = get_sum_of_divisors_of(i);
        // Ignore perfect numbers...
        if sum_1 == i {
            continue;
        }
        let sum_2 = get_sum_of_divisors_of(sum_1);
        if i == sum_2 {
            sum_of_amicable_numbers += i;
        }
    }
    sum_of_amicable_numbers
}

fn get_sum_of_divisors_of(number: usize) -> usize {
    let mut sum_of_divisors = 1;
    for candidate_divisor in 2..(number - 1) {
        if number % candidate_divisor == 0 {
            sum_of_divisors += candidate_divisor;
        }
    }
    sum_of_divisors
}

#[cfg(test)]
mod tests {
    use super::{get_sum_of_divisors_of, get_sums_of_proper_divisors};

    mod test_sums_of_all_proper_divisors {
        use super::get_sums_of_proper_divisors;

        #[test]
        fn test_sum_of_divisors_of_220() {
            let sums_of_all_proper_divisors = get_sums_of_proper_divisors(285);
            let sum_of_divisors = sums_of_all_proper_divisors[220];
            assert_eq!(284, sum_of_divisors);
        }

        #[test]
        fn test_sum_of_divisors_of_284() {
            let sums_of_all_proper_divisors = get_sums_of_proper_divisors(285);
            let sum_of_divisors = sums_of_all_proper_divisors[284];
            assert_eq!(220, sum_of_divisors);
        }
    }

    mod test_sums_of_divisors_of {
        use super::get_sum_of_divisors_of;

        #[test]
        fn test_sum_of_divisors_of_220() {
            let sum_of_divisors = get_sum_of_divisors_of(220);
            assert_eq!(284, sum_of_divisors);
        }

        #[test]
        fn test_sum_of_divisors_of_284() {
            let sum_of_divisors = get_sum_of_divisors_of(284);
            assert_eq!(220, sum_of_divisors);
        }
    }
}
