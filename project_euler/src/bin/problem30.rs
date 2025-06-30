use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let start_time = Instant::now();

    for rep in 0..NUM_REPETITIONS {
        // 7 * 9^5 = 413_343, which is only a 6 digit number, not 7. So at most 6 digits are needed.
        let mut sum_of_nums = 0;
        for candidate in 2..1_000_000_u32 {
            let mut sum_of_pows = 0;
            let mut quotient = candidate;
            while quotient > 0 {
                let digit = quotient % 10;
                quotient /= 10;
                sum_of_pows += digit.pow(5);
            }
            if sum_of_pows == candidate {
                if rep == 0 {
                    println!("Found: {}", candidate);
                }
                sum_of_nums += sum_of_pows;
            }
        }

        if rep == 0 {
            println!("Answer = {}", sum_of_nums);
        }
    }

    let duration = start_time.elapsed();
    println!("Duration: {:?}", duration / NUM_REPETITIONS);
}
