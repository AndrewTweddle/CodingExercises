use std::time::Instant;

fn main() {
    let start = Instant::now();

    // There are 40 total moves (right or down). In each path, any 20 of them can be rightward.
    let num_paths = choose(40, 20);
    println!("Number of paths: {}", num_paths);

    let duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

pub fn choose(n: u64, mut c: u64) -> u128 {
    if c > n {
        return 0;
    }
    if c > n - c {
        c = n - c;
    }

    let mut numerators: Vec<u64> = ((n - c + 1)..=n).collect();
    let mut denominators: Vec<u64> = (2..=c).collect();

    // Cancel out common factors as much as possible, to reduce the risk of overflow...
    for num in numerators.iter_mut() {
        for denom in denominators.iter_mut() {
            if *denom == 1 {
                continue;
            }
            let factor = gcd(*num, *denom);
            if factor > 1 {
                *num /= factor;
                *denom /= factor;
                if *num == 1 {
                    break;
                }
            }
        }
    }

    // Calculate the combinatorial (the denominators must be 1, since the answer is an integer)...
    numerators.iter().map(|numer| (*numer) as u128).product()
}

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        return a;
    }
    loop {
        let rem = a % b;
        if rem == 0 {
            break b;
        }
        a = b;
        b = rem;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod gcd_tests {
        use super::gcd;

        #[test]
        fn test_gcd_a_lt_b() {
            assert_eq!(gcd(4, 6), 2);
        }

        #[test]
        fn test_gcd_a_zero() {
            assert_eq!(gcd(0, 6), 6);
        }

        #[test]
        fn test_gcd_b_zero() {
            assert_eq!(gcd(6, 0), 6);
        }
    }

    mod choose_tests {
        use super::choose;

        #[test]
        fn test_choose() {
            assert_eq!(choose(4, 0), 1);
            assert_eq!(choose(4, 1), 4);
            assert_eq!(choose(4, 2), 6);
            assert_eq!(choose(4, 3), 4);
            assert_eq!(choose(4, 4), 1);
        }

        #[test]
        fn test_choose_zero_from_zero() {
            assert_eq!(choose(0, 0), 1);
        }

        #[test]
        #[should_panic]
        fn test_choose_very_large() {
            println!("250 choose 125 = {}", choose(250, 125));
        }
    }
}
