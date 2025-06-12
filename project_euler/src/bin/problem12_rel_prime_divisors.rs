use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;
const NUM_DIVISORS: u64 = 500;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        // Only start counting factors from ceil(sqrt(500)) = 23,
        // since a number can't have more distinct divisors than itself.
        let mut triangular_num: u64 = 22 * 23 / 2; // The 22nd triangular number
        let mut divs_i = count_divisors(23);

        for i in 23_u64.. {
            triangular_num += i;
            let divs_i_plus_1 = if i % 2 == 0 {
                count_divisors(i + 1)
            } else {
                // if i+1 is even, count divisors of (i+1)/2
                count_divisors(i.div_ceil(2))
            };
            // i and i+1 have no common factors except 1
            // (since common factors divide their difference, and the difference is 1).
            // So after first halving whichever of i and i + 1 is even, we can multiply
            // the number of divisors of each to get the number of divisors of their product
            if divs_i * divs_i_plus_1 > NUM_DIVISORS {
                if rep == 0 {
                    println!("T({}) = {}", i, triangular_num);
                }
                break;
            }
            divs_i = divs_i_plus_1;
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn count_divisors(n: u64) -> u64 {
    let floor_sqrt_n = int_sqrt(n);
    let div_pair_count = (1..=floor_sqrt_n).filter(|i| n % i == 0).count() as u64;
    if floor_sqrt_n * floor_sqrt_n == n {
        // Don't double count the int_sqrt divisor...
        div_pair_count * 2 - 1
    } else {
        div_pair_count * 2
    }
}

// From https://en.wikipedia.org/wiki/Integer_square_root#Using_bitwise_operations
fn int_sqrt(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let small_cand = int_sqrt(n >> 2) << 1;
    let large_cand = small_cand + 1;
    if large_cand * large_cand > n {
        small_cand
    } else {
        large_cand
    }
}
