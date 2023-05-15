use std::time::Instant;

const DENOMINATIONS: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];

fn main() {
    let start_time = Instant::now();

    // Incrementally calculate and cache the number of ways of reaching each number from 1 to 200,
    // using a particular denomination (or rather index into DENOMINATIONS) as the highest coin.
    // This avoids counting every permutation of each valid set of coins.
    let mut ways_by_max_denom = Vec::<[usize; 8]>::with_capacity(201);
    ways_by_max_denom.push([0; 8]);

    for target in 1..=200_usize {
        ways_by_max_denom.push([0; 8]);
        for (i, &denom) in DENOMINATIONS.iter().enumerate() {
            if denom > target {
                break;
            };
            let mut total_ways_with_denom_as_max = 0;
            if denom == target {
                total_ways_with_denom_as_max = 1;
            } else {
                for j in 0..=i {
                    total_ways_with_denom_as_max += ways_by_max_denom[target - denom][j];
                }
            }
            ways_by_max_denom[target][i] = total_ways_with_denom_as_max;
        }
    }

    let answer: usize = (0..8).map(|i| ways_by_max_denom[200][i]).sum();

    println!("# of ways: {answer}");

    let duration = start_time.elapsed();
    println!("Duration: {duration:?}");
}
