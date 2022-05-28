use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    // The most digits possible in the result is 7, since
    // a. 7 * 9! = 7 * 362880 = 2540160
    // b. 8 * 9! = 8 * 362880 = 2903040
    //
    // Thus even the largest possible 8 digit input has a 7 digit output

    let mut sum_of_nums: u64 = 0;
    calculate_sums_recursively(7, 0, 0, &mut sum_of_nums);
    sum_of_nums -= 3; // 1! and 2! should not be counted
    println!(
        "The sum of numbers which are equal to the sum of the factorials of their digits is {}",
        sum_of_nums
    );

    let duration = start_time.elapsed();
    println!("Duration: {:?}", duration);
}

fn calculate_sums_recursively(
    digits_left: usize,
    partial_num: u32,
    factorial_sum: u32,
    sum_of_nums: &mut u64,
) {
    // Try the next lower digit
    for digit in 0_u32..10 {
        let new_partial_num = partial_num + digit;
        let new_factorial_sum = if new_partial_num == 0 {
            // Don't count leading zeroes as 0! = 1
            0
        } else {
            factorial_sum + (1..=digit).product::<u32>()
        };
        if digits_left == 1 {
            if new_partial_num == new_factorial_sum {
                *sum_of_nums += new_partial_num as u64
            }
        } else {
            calculate_sums_recursively(
                digits_left - 1,
                10 * new_partial_num,
                new_factorial_sum,
                sum_of_nums,
            );
        }
    }
}
