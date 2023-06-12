use std::time::Instant;

const NUM_REPETITIONS: u32 = 0;

fn main() {
    let mut start_time = Instant::now();

    for rep in 0..=NUM_REPETITIONS {
        let answer: u64 = solve().unwrap();

        if rep == 0 {
            println!("Answer: {answer}");
            println!(
                "Duration including printing answer to stdout: {:?}",
                start_time.elapsed()
            );
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
                #[cfg(debug_assertions)]
                {
                    let sum = p_j + p_k;
                    let k = invert_pentagonal(p_k).unwrap();
                    let n = invert_pentagonal(sum).unwrap();

                    println!("diff = P[{i}] = {diff}");
                    println!("p_j = P[{j}] = {p_j}");
                    println!("p_k = P[{k}] = {p_k}");
                    println!("sum = P[{n}] = {sum}");
                }

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
    invert_pentagonal(p).is_some()
}

// Let T[n] be the n-th triangular number: T[n] = n(n+1)/2
// If p is pentagonal, then p = n(3n-1)/2 for some integer n.
// Then 3p = 3n(3n-1)/2 = (3n-1).[(3n-1) + 1] = T[3n-1]
#[inline]
fn invert_pentagonal(p: u64) -> Option<u64> {
    invert_triangular(3 * p).and_then(|n| (n % 3 == 2).then_some((n + 1) / 3))
}

// If t is triangular, then t = n(n+1)/2 for some integer n.
// 8t + 1 = 4n(n+1) + 1 = 4n^2 + 4n + 1 = (2n+1)^2.
#[inline]
fn invert_triangular(t: u64) -> Option<u64> {
    let candidate_square = 8 * t + 1;
    let s = int_sqrt(candidate_square);
    (s * s == candidate_square).then_some((s - 1) / 2)
}

// From https://en.wikipedia.org/wiki/Integer_square_root#Using_bitwise_operations
#[inline]
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

#[cfg(test)]
mod tests {
    use crate::{invert_pentagonal, invert_triangular};

    #[test]
    fn test_invert_triangular() {
        let expected: [u64; 10] = [1, 3, 6, 10, 15, 21, 28, 36, 45, 55];
        let calculated: Vec<u64> = (1..=60)
            .filter_map(|t| invert_triangular(t).and(Some(t)))
            .collect();
        assert_eq!(&calculated, &expected);
    }

    #[test]
    fn test_invert_pentagonal() {
        let expected: [u64; 10] = [1, 5, 12, 22, 35, 51, 70, 92, 117, 145];
        let calculated: Vec<u64> = (1..=150)
            .filter_map(|p| invert_pentagonal(p).and(Some(p)))
            .collect();
        assert_eq!(&calculated, &expected);
    }
}
