use std::time::Instant;

const NUM_REPETITIONS: u32 = 10;
const NUM_DIVISORS: u64 = 500;

// For integer square root calculations, determine a pair of rational numbers
// which are close lower and upper bounds for 1.0/sqrt(2).
// Use powers of 2 for the denominators, for quick division using the shift right operator.
// Choose a power of 2 that is <= 32, because, if an i32 produces a solution,
// then a u64 won't overflow when multiplying by a numerator under 2^32 (NB: 1/sqrt(2) < 1).

// In practice, the following bounds work well: 0.6875 = 22/32 < 1/sqrt(2) < 3/4 = 0.75
const LOWER_BOUND_DENOM_BITS: i32 = 5;
const UPPER_BOUND_DENOM_BITS: i32 = 2;
const LOWER_BOUND_DENOM: u64 = 1u64 << LOWER_BOUND_DENOM_BITS;
const UPPER_BOUND_DENOM: u64 = 1u64 << UPPER_BOUND_DENOM_BITS;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let lower_bound_numer: u64 = (0.5f32.sqrt() * LOWER_BOUND_DENOM as f32).floor() as u64;
        let upper_bound_numer: u64 = (0.5f32.sqrt() * UPPER_BOUND_DENOM as f32).floor() as u64 + 1;

        let mut num: u64 = 0;
        for i in 1.. {
            num += i;

            // Find the integer square root of num, based on the formula n = T(i) = i*(i+1)/2.
            //
            // 1. i = sqrt(i*i) < sqrt(i*(i+1)) < srt((i+1)(i+1) = i + 1
            // 2. lower_bound_numerator/2^32 < 1/sqrt(2) < upper_bound_numerator/2^32
            // 3. Multiply these to give...
            //    i*lower_bound_numerator/2^32 < sqrt(i*(i+1)/2) < (i+1)*upper_bound_numerator/2^32
            // 4. Integer division is equivalent to the floor of the floating point lower bound.
            //    Start searching from this lower bound to find the floor of the square root of num.
            // 5. Do the same for the upper bound, but using i+1 not i, and convert a ceiling
            //    upper bound to a floor by first adding the denominator to the numerator
            //    (i.e. adding 1) before dividing by the denominator.
            let mut min_sqrt_num: u64 = (lower_bound_numer * i) >> LOWER_BOUND_DENOM_BITS;
            let mut max_sqrt_num: u64 =
                (upper_bound_numer * (i + 1) + UPPER_BOUND_DENOM) >> UPPER_BOUND_DENOM_BITS;

            // Find the integer square root of num by binary search
            while min_sqrt_num < max_sqrt_num - 1 {
                let mid_sqrt_num = (min_sqrt_num + max_sqrt_num) / 2;
                if mid_sqrt_num * mid_sqrt_num < num {
                    min_sqrt_num = mid_sqrt_num
                } else {
                    max_sqrt_num = mid_sqrt_num;
                }
            }

            // Now min_sqrt_num == floor(sqrt(num))
            if count_divisors(num, min_sqrt_num) >= NUM_DIVISORS {
                if rep == 0 {
                    println!("T({}) = {}", i, num);
                }
                break;
            }
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

#[inline]
fn count_divisors(n: u64, floor_sqrt_n: u64) -> u64 {
    let div_pair_count: u64 = (1..=floor_sqrt_n).filter(|i| n % i == 0).count() as u64;
    if floor_sqrt_n * floor_sqrt_n == n {
        // Don't double count the int_sqrt divisor...
        div_pair_count * 2 - 1
    } else {
        div_pair_count * 2
    }
}
