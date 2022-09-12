use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let mut start_time = Instant::now();

    for rep in 0..=NUM_REPETITIONS {
        let mut best_p = 0;
        let mut best_soln_count = 0;

        for p in 1..=1000 {
            let mut soln_count = 0;

            for b in 1..=(p / 2) {
                for a in 1..b {
                    let c = p - a - b;
                    if a * a + b * b == c * c {
                        soln_count += 1;
                    }
                }
            }

            if soln_count > best_soln_count {
                best_soln_count = soln_count;
                best_p = p;

                if rep == 0 {
                    println!("p = {} has {} solutions", p, soln_count);
                }
            }
        }

        if rep == 0 {
            println!("Best p: {}", best_p);
            println!("Solutions: {}", best_soln_count);
            println!("Duration: {:?}", start_time.elapsed());

            // Restart the timer, so that further repetitions don't count the print statements
            start_time = Instant::now();
        }
    }

    let duration = start_time.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}
