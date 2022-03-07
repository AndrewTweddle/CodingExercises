use std::fs;
use std::time::Instant;

fn main() {
    let start_time = Instant::now();

    let contents = fs::read_to_string("data/day17_input.txt").unwrap();
    let (part1, part2) = contents.split_once(",").unwrap();
    let (x_start_str, x_end_str) = part1["target area: x=".len()..].split_once("..").unwrap();
    let (y_start_str, y_end_str) = part2[" y=".len()..].trim().split_once("..").unwrap();
    let x_start: i64 = x_start_str.parse().unwrap();
    let x_end: i64 = x_end_str.parse().unwrap();
    let y_start: i64 = y_start_str.parse().unwrap();
    let y_end: i64 = y_end_str.parse().unwrap();

    // Only solve the case of a box to the right and lower down from the initial position S...
    assert!(x_start > 0 && x_end > 0);
    assert!(y_start < 0 && y_end < 0);

    if let Some(max_height) = get_max_height(x_start as u64, x_end as u64, y_start, y_end) {
        println!("Part 1 answer: {}", max_height);
    } else {
        println!("Part 1 answer: None found!");
    }
    let duration = start_time.elapsed();
    println!("Duration: {:?}", duration);
}

fn get_max_height(x_start: u64, x_end: u64, y_start: i64, y_end: i64) -> Option<u64> {
    // Let triangular number function be T(n) = 1 + 2 + ... + n = n * (n + 1) / 2
    //
    // Looking back along the x-axis from the point at which the probe goes vertical,
    // the x velocities form the terms of a triangular number series, ending at the origin, S.
    //
    // Let h be the starting horizontal velocity.
    // Calculate the minimum # of horizontal steps to get into or past the box,
    // working backwards from when the velocity could first be zero and be in or past the box...
    let min_h = inv_tri(x_start - 1) + 1;

    // Let v be the initial vertical velocity.
    // Looking from the top of the trajectory (two adjacent points where vertical velocity is zero),
    // we have triangular numbers going back to the origin S, on the left,
    // and to a point in the box, on the right.
    //
    // After rising for v steps, hovering for 1 step, and falling for v steps,
    // the probe will have y == 0 again. 1 or more steps will be required to get it into the box.
    //
    // So the probe rises to a maximum height of T(v) after v steps. So we want to maximize v.
    // It then falls by T(v+b) vertically to be in the box.
    // So the y-value is T(v) - T(v+b), and this must be between y_start and y_end (inclusive).
    // v will be maximal if b == 1, so try this value of v first,
    // and then incrementally reduce v until a valid solution is found.
    // When b == 1, T(v) - T(v+1) = -(v + 1) <= y_start.
    // This gives an upper bound on the value of v...
    let max_v = -y_start - 1;

    for v_offset in 0..max_v {
        let v = max_v - v_offset;

        // Try different horizontal velocities until finding one which works,
        // stopping if the initial horizontal velocity immediately overshoots the box...
        for h in min_h..=x_end {
            let mut x: u64 = 0;
            let mut y: i64 = 0;
            let mut x_vel = h;
            let mut y_vel = v;
            for _time_step in 0.. {
                x += x_vel;
                y += y_vel;
                if x >= x_start && x <= x_end && y <= y_end && y >= y_start {
                    // A solution has been found
                    return Some(tri(v as u64));
                }
                if x > x_end || y < y_start {
                    break;
                }
                if x_vel > 0 {
                    x_vel -= 1;
                }
                y_vel -= 1;
            }
        }
    }
    None
}

// Compute triangular number:
fn tri(n: u64) -> u64 {
    n * (n + 1) / 2
}

// Invert the target triangular number, returning the value n
// such that T[n] <= target_tri_num < T[n+1]
fn inv_tri(target_tri_num: u64) -> u64 {
    // 8*T(n)+1 = 8*n*(n+1)/2+1 = 4*n^2+4*n+1 = (2n+1)^2
    let square = 8 * target_tri_num + 1;
    let sqrt = int_sqrt(square);
    (sqrt - 1) / 2
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

#[cfg(test)]
mod tests {
    use super::{inv_tri, tri};

    #[test]
    fn test_tri() {
        assert_eq!(tri(10), 55);
    }

    #[test]
    fn test_inv_tri_exact() {
        assert_eq!(inv_tri(55), 10);
    }

    #[test]
    fn test_inv_tri_with_max_remainder() {
        assert_eq!(inv_tri(54), 9);
    }

    #[test]
    fn test_inv_tri_with_min_non_zero_remainder() {
        assert_eq!(inv_tri(56), 10);
    }
}
