fn main() {
    // the sum of cubes is the square of the sum,
    // so square of sum - sum of squares = sum of cubes - sum of squares = sum of (cube - square)
    let diff: i64 = (1..=100).map(|i| i * i * (i - 1)).sum();
    println!("Square of sum - Sum of squares = {}", diff);
}
