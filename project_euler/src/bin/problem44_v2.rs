use std::time::Instant;
use num::integer::Roots;

const NUM_REPETITIONS: u32 = 0;

fn main() {
    let mut start_time = Instant::now();

    for rep in 0..=NUM_REPETITIONS {
        let answer: u64 = solve().unwrap();

        if rep == 0 {
            println!("Answer: {answer}");
            println!("Duration including printing answer to stdout: {:?}", start_time.elapsed());
            start_time = Instant::now();
        }
    }

    if NUM_REPETITIONS != 0 {
        let avg_duration = start_time.elapsed() / NUM_REPETITIONS;
        println!("Avg duration over {NUM_REPETITIONS} repetitions: {avg_duration:?}");
    }
}

fn solve() -> Option<u64> {
    // We want to find the smallest difference of p_j and p_k,
    // where p_j = P[j], p_k = P[k], for some positive integers j and k, with P[j] < P[k].
    // Iterate over possible values for the difference, so that we can stop at the first solution,
    // since that will also be the solution with the minimum difference.
    for i in 1.. {
        // The difference must also be a pentagonal number...
        let diff = pentagonal_number(i);

        // Iterate over all feasible pentagonal numbers p_j = P[j].
        // We can't iterate forever. So how large can p_j get (given a particular value of diff)?
        //
        // p_k > p_j, so p_k >= P[j+1], since the pentagonal numbers increase monotonically.
        // Since p_k = p_j + diff, we know that P[j] + diff >= P[j+1].
        // Hence: diff >= P[j+1] - P[j]
        //
        // Let's work out the formula for P[j+1] - P[j].
        // Here's a creative way, using triangular numbers...
        //
        // P[n] = n(3n-1)/2, and T[n] = n*(n-1)/2 is the n-th triangular number.
        // Hence: 3 P[n] = T[3n-1]
        // And: 3 (P[n+1] - P[n])
        //    = 3 P[n+1] - 3 P[n]
        //    = T[3(n+1)-1] - T[3n-1]
        //    = T[3n+2] - T[3n-1]
        //    = [1 + 2 + ... + (3n-1) + 3n + (3n+1) + (3n+2)] - [1 + 2 + ... + (3n-1)]
        //    = 3n + (3n+1) + (3n+2)
        //    = 9n + 3
        //    = 3(3n+1)
        // So: P[n+1] - P[n] = 3n + 1
        //
        // Thus: diff >= P[j+1] - P[j] = 3j + 1
        // So: j <= (diff - 1) / 3...
        let max_j = (diff - 1) / 3;
        for j in 1..=max_j {
            let p_j = pentagonal_number(j);

            // since: diff = P[i],
            //   and: diff + P[j] = P[k]
            //  then: P[i] + P[j] >= P[i+1],
            //    so: P[j] >= P[i+1] - P[i] = 3i + 1
            if p_j < 3 * i + 1 {
                continue;
            }

            let p_k = p_j + diff;
            if is_pentagonal(p_k) && is_pentagonal(p_j + p_k) {
                return Some(diff);
            }
        }
    }
    None
}

// The formula for the n-th pentagonal number is given as P[n] = n(3n-1)/2
#[inline]
fn pentagonal_number(i: u64) -> u64 {
    i * (3 * i - 1) / 2
}

#[inline]
fn is_pentagonal(p: u64) -> bool {
    let m = 24 * p + 1;
    let s = m.sqrt();
    (s * s == m) && (s % 6 == 5)
}

#[cfg(test)]
mod tests {
    use crate::{is_pentagonal};

    #[test]
    fn test_is_pentagonal() {
        let expected: [u64; 10] = [1, 5, 12, 22, 35, 51, 70, 92, 117, 145];
        for p in 1..=145 {
            assert_eq!(expected.contains(&p), is_pentagonal(p));
        }
    }
}