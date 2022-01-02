use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

const NUM_REPETITIONS: u32 = 100;
const NUM_DIVISORS: u64 = 500;
const NUM_THREADS: u64 = 5;
const PRIME_CUTOFF: u64 = 36;
const PRIMES: [u64; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
// NUM_DIVISORS    RELEASE MODE     SOLUTION
//          500        7.901141ms      T(12375) =        76576500
//         2000      277.964796ms     T(313599) =     49172323200
//         5000    1.349901607s      T(2203200) =   2427046221600
//        10000   14.083371034s     T(14753024) = 108825865948800

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let channels: Vec<(Receiver<u64>, JoinHandle<Result<(), ()>>)> = (0..NUM_THREADS)
            .map(|seq| {
                let start_num: u64 = 2 + seq;
                let (sender, receiver) = mpsc::sync_channel(1000);
                let handle: JoinHandle<Result<(), ()>> = thread::spawn(move || {
                    for i in (start_num..).step_by(NUM_THREADS as usize) {
                        let n = if i % 2 == 0 { i / 2 } else { i };
                        let divs = count_divisors_using_prime_decomposition(n);
                        if sender.send(divs).is_err() {
                            break;
                        }
                    }
                    Ok(())
                });
                (receiver, handle)
            })
            .collect();

        let mut triangular_num: u64 = 0;
        let mut divs_i = 1;

        for i in 1_u64.. {
            triangular_num += i;
            let channel_index = (i - 1) % NUM_THREADS;
            let (receiver, _) = &channels[channel_index as usize];
            let divs_i_plus_1 = receiver.recv().unwrap();

            // i and i+1 have no common factors except 1
            // (since common factors divide the difference, and the difference is 1).
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

fn count_divisors_using_prime_decomposition(mut n: u64) -> u64 {
    let mut div_count = 1;
    for prime in PRIMES {
        let mut prime_exponent = 0;
        while n % prime == 0 {
            n /= prime;
            prime_exponent += 1;
        }
        // prime^prime_count has divisors: 1, prime, prime^2, ..., prime^prime_count
        let prime_power_divs = prime_exponent + 1;
        div_count *= prime_power_divs;

        // Surprisingly, the following line doubles run-time...
        // if n == 1 { break; }
    }
    if n != 1 {
        let rem_divs = count_divs_excl_small_primes(n);
        div_count *= rem_divs;
    }
    div_count
}

fn count_divs_excl_small_primes(n: u64) -> u64 {
    let floor_sqrt_n = int_sqrt(n);

    // All numbers up to PRIME_CUTOFF are decomposed into powers of the selected primes,
    // so none of them will be divisors of n, since n contains none of these primes as factors.
    // The exception is 1...
    let div_pair_count = ((PRIME_CUTOFF + 1)..=floor_sqrt_n)
        .filter(|i| i % 2 != 0 && i % 3 != 0 && n % i == 0)
        .count() as u64
        + 1;
    if floor_sqrt_n * floor_sqrt_n == n {
        // Don't double count the floor_sqrt_n divisor...
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
