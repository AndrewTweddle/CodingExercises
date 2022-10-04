use std::fs;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;

fn main() {
    for (problem, num_rounds) in [("Part 1", 2), ("Part 2", 50)] {
        let mut start_time = Instant::now();

        // Read the input file (outside the loop, so that IO does not affect the timings)...
        let contents = fs::read_to_string("data/day20_input.txt").unwrap();

        // Perform multiple repetitions to get an average duration of the algorithm...
        for rep in 0..=NUM_REPETITIONS {
            let (alg_str, input_image_str) = contents.split_once("\n\n").unwrap();
            let alg: Vec<usize> = alg_str
                .bytes()
                .map(|byte| if byte == b'#' { 1 } else { 0 })
                .collect();
            assert_eq!(alg.len(), 512);

            let input_image: Vec<Vec<usize>> = input_image_str
                .lines()
                .map(|line| {
                    line.bytes()
                        .map(|byte| if byte == b'#' { 1 } else { 0 })
                        .collect::<Vec<usize>>()
                })
                .collect();

            let row_count = input_image.len();
            let col_count = input_image[0].len();

            // The area is infinite but uniform outside the currently expanded (rectangular) region.
            // In round 0, the region is the initial image.
            // Each round thereafter, we expand the previous round's region
            // by an extra (calculated) pixel in each direction.
            // To perform this calculation, for each calculated pixel we need to look at
            // the 3x3 block around the corresponding pixel in the previous round.
            // Thus the previous round needs to have set the pixels (to the out-of-region value).
            // This needs to be to a margin of 2 around the previous region.
            // So in round i, the region being calculated has a margin i around the original image.
            // But a further 2 pixels in each direction need to be set to the out-of-region value.

            // The initial out-of-region state is false (dark/unlit).
            // Calculate the out-of-region state in subsequent odd and even numbered rounds.
            let odd_out_of_region_state = alg[0];
            let even_out_of_region_state = alg[511 * odd_out_of_region_state];

            // Since the part 1 problem takes place over 2 rounds,
            // and there must be a finite answer to the number of lit pixels in the final image,
            // we know that the even rounds must be unlit outside of the calculated image.
            assert_eq!(even_out_of_region_state, 0);

            // We will store two images, one for even rounds and one for odd rounds.
            // All images must be able to accommodate the calculated region in the final round.
            // However the penultimate round needs a margin one greater than this
            // to server as input to the final round calculations.
            let final_margin = num_rounds + 1;
            let final_width = col_count + 2 * final_margin;
            let final_height = row_count + 2 * final_margin;

            let mut even_image = vec![vec![even_out_of_region_state; final_width]; final_height];
            let mut odd_image = vec![vec![odd_out_of_region_state; final_width]; final_height];

            // Write the original image to even_image (since round 0 is even)...
            for r in 0..row_count {
                for c in 0..col_count {
                    even_image[r + final_margin][c + final_margin] = input_image[r][c];
                }
            }

            for round in 1..=num_rounds {
                let (src_image, dest_image) = if round % 2 == 0 {
                    (&odd_image, &mut even_image)
                } else {
                    (&even_image, &mut odd_image)
                };

                for r in (final_margin - round)..(final_height - final_margin + round) {
                    for c in (final_margin - round)..(final_height - final_margin + round) {
                        let nine_pixel_code = 256 * src_image[r - 1][c - 1]
                            + 128 * src_image[r - 1][c]
                            + 64 * src_image[r - 1][c + 1]
                            + 32 * src_image[r][c - 1]
                            + 16 * src_image[r][c]
                            + 8 * src_image[r][c + 1]
                            + 4 * src_image[r + 1][c - 1]
                            + 2 * src_image[r + 1][c]
                            + src_image[r + 1][c + 1];
                        dest_image[r][c] = alg[nine_pixel_code];
                    }
                }
            }

            let final_image = if num_rounds % 2 == 0 {
                &even_image
            } else {
                &odd_image
            };
            let num_lit: usize = final_image
                .iter()
                .map(|row| (*row).iter().sum::<usize>())
                .sum();

            if rep == 0 {
                // Print out the solution during the initial repetition only...
                println!("{}", problem);
                println!("------");

                #[cfg(debug_assertions)]
                {
                    println!("{} final image:", problem);
                    for row in final_image {
                        let row_text: String = row
                            .iter()
                            .map(|&cell| if cell == 0 { '.' } else { '#' })
                            .collect();
                        println!("    {}", row_text);
                    }
                    println!();
                }

                println!("Number of lit pixels: {}", num_lit);
                println!("Duration (incl file i/o and printlns): {:?}", start_time.elapsed());

                // Restart the timer to time all other iterations without the overhead of printlns
                start_time = Instant::now();
            }
        }

        let duration = start_time.elapsed();
        println!(
            "Average duration over {} further repetitions: {:?}\n",
            NUM_REPETITIONS,
            duration / NUM_REPETITIONS
        );
    }
}
