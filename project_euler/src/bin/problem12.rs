use std::time::Instant;

fn main() {
    let start = Instant::now();

    let triangular_number = get_first_triangular_number_with_over_n_divisors(500);
    println!(
        "First triangular number with over 500 divisors: {}",
        triangular_number
    );

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

fn get_first_triangular_number_with_over_n_divisors(min_divisors_less_1: u64) -> u64 {
    let mut i = 0;
    let mut divisor_count = 0;
    let mut n = 0;
    while divisor_count <= min_divisors_less_1 {
        i += 1;
        n = i * (i + 1) / 2;
        divisor_count = count_divisors(n);
    }
    n
}

fn count_divisors(n: u64) -> u64 {
    let mut divisors = 0;
    for divisor in (1..).take_while(|i| i * i <= n) {
        if n % divisor == 0 {
            let larger = n / divisor;
            divisors += if larger == divisor { 1 } else { 2 };
        }
    }
    divisors
}

#[test]
fn test_count_divisors() {
    assert_eq!(count_divisors(28), 6);
}

#[test]
fn test_first_triangular_number() {
    assert_eq!(get_first_triangular_number_with_over_n_divisors(5), 28);
}
