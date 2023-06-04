use std::fs;

// clippy wants then_some() to be used instead of then(), but this causes an underflow error
#[allow(clippy::unnecessary_lazy_evaluations)]
fn main() {
    let data_file_path = "data/p042_words.txt";
    let text = fs::read_to_string(data_file_path).unwrap();
    let triangular_count: usize = text
        .split(',')
        .filter(|&line| {
            let word_value: u64 = line
                .to_uppercase()
                .bytes()
                .filter_map(|byte| {
                    byte.is_ascii_uppercase().then(|| (byte - b'A' + 1) as u64)
                })
                .sum();

            is_triangular(word_value)
        })
        .count();
    println!("triangle number count: {triangular_count}");
}

// If t is triangular, then t = n(n+1)/2 for some integer n.
// 8t + 1 = 4n(n+1) + 1 = 4n^2 + 4n + 1 = (2n+1)^2.
fn is_triangular(t: u64) -> bool {
    let candidate_square = 8 * t + 1;
    let s = int_sqrt(candidate_square);
    s * s == candidate_square
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
