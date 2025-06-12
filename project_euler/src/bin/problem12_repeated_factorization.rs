use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;
const NUM_DIVISORS: u64 = 500;

// NUM_DIVISORS    RELEASE MODE     SOLUTION
//          500        1.610237ms      T(12375) =        76576500
//         2000      116.158434ms     T(313599) =     49172323200
//         5000    1.715159128s      T(2203200) =   2427046221600
//        10000   25.063840952s     T(14753024) = 108825865948800

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let mut triangular_num: u64 = 0;
        let mut divs_i = 1;
        for i in 1_u64.. {
            triangular_num += i;
            let divs_i_plus_1 = if i % 2 == 0 {
                count_divisors(i + 1)
            } else {
                // if i+1 is even, count divisors of (i+1)/2
                count_divisors(i.div_ceil(2))
            };
            // i and i+1 have no common factors except 1
            // (since common factors divide the difference, and the difference is 1).
            // So after first halving whichever of i and i + 1 is even, we can multiply
            // the number of divisors of each to get the number of divisors of their product
            if divs_i * divs_i_plus_1 > NUM_DIVISORS {
                if rep == 0 {
                    println!("T({}) = {}", i, triangular_num);
                    println!("Divisor counts: {} * {} = {}", divs_i, divs_i_plus_1,
                             divs_i * divs_i_plus_1);
                }
                break;
            }
            divs_i = divs_i_plus_1;
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn count_divisors(mut n: u64) -> u64 {
    let mut div_count = 1;
    if n != 1 {
        // Try all numbers as potential prime factors (but only primes will factorize the number)
        for prime in 2.. {
            // If n is a large prime number, then this could take a very long time to find.
            // Check if prime > square root of n. If so, n is the only remaining divisor.
            if prime * prime > n
            {
                // Two divisors: 1 and n
                div_count *= 2;
                break;
            }

            if n % prime == 0 {
                let mut prime_exponent = 1;
                n /= prime;
                while n % prime == 0 {
                    n /= prime;
                    prime_exponent += 1;
                }
                // prime^prime_count has divisors: 1, prime, prime^2, ..., prime^prime_count
                let prime_power_divs = prime_exponent + 1;
                div_count *= prime_power_divs;
                if n == 1 {
                    break;
                }
            }
        }
    }
    div_count
}
