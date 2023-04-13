use std::time::Instant;

pub const NUM_REPETITIONS: u32 = 100000;

fn main() {
    let mut start_time = Instant::now();

    for i in 0..=NUM_REPETITIONS {
        let mut prod_of_digits = 1;
        let mut target_digit_index = 1_u32;
        for _ in 0..=6 {
            let mut target_offset_in_range = target_digit_index - 1;
            let mut range_start_number = 1;
            let mut digits_in_range = 9;
            let mut digits_per_number = 1;

            while target_offset_in_range >= digits_in_range {
                target_offset_in_range -= digits_in_range;
                range_start_number *= 10;
                digits_per_number += 1;
                digits_in_range = 9 * range_start_number * digits_per_number;
            }

            let mut number = range_start_number + target_offset_in_range / digits_per_number;
            let exponent_of_digit_in_number =
                digits_per_number - 1 - target_offset_in_range % digits_per_number;
            for _ in 0..exponent_of_digit_in_number {
                number /= 10;
            }
            let digit = number % 10;
            prod_of_digits *= digit;

            target_digit_index *= 10;
        }

        if i == 0 {
            println!("Solution: {prod_of_digits}");
            println!("Duration (including printing): {:?}", start_time.elapsed());

            // Restart the timer
            start_time = Instant::now();
        }
    }

    let duration = start_time.elapsed() / NUM_REPETITIONS;
    println!("Average duration over {NUM_REPETITIONS} iterations: {duration:?}")
}