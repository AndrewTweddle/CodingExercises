use std::time::Instant;

// See https://en.wikipedia.org/wiki/Pythagorean_triple#Generating_a_triple.
// Pythagorean triples (a, b, c) with integer a, b and c have the form:
//  (k(m^2-n^2), 2kmn, k(m^2+n^2))
// where k,m and n are positive integers with m > n, gcd(m, n) == 1, and m and n not both odd.
// and k = gcd(a, b, c).
// The perimeter is then p = a + b + c = 2km^2 + 2kmn = 2km(m+n)

const NUM_REPETITIONS: u32 = 100;

fn main() {
    let mut start_time = Instant::now();

    for rep in 0..=NUM_REPETITIONS {
        let mut best_p = 0;
        let mut best_soln_count = 0;

        let mut triangle_counts_by_perimeter = [0_u32; 1001];

        for m in 2..500 {
            let m_is_odd = m % 2 == 1;
            for n in 1..m {
                if m_is_odd && n % 2 == 1 {
                    // m and n cannot both be odd
                    continue;
                }
                if m * (m + n) > 500 {
                    break;
                }
                if gcd(m, n) > 1 {
                    continue;
                }
                // k is a factor that divides into
                for k in 1.. {
                    // p is the perimeter
                    let p = 2 * k * m * (m + n);
                    if p > 1000 {
                        break;
                    }
                    triangle_counts_by_perimeter[p as usize] += 1;
                }
            }
        }

        for p in 1..=1000 {
            let soln_count = triangle_counts_by_perimeter[p as usize];

            if soln_count > best_soln_count {
                best_soln_count = soln_count;
                best_p = p;

                if rep == 0 {
                    println!("p = {} has {} solutions", p, soln_count);
                }
            }
        }

        if rep == 0 {
            println!("Best p: {}", best_p);
            println!("Solutions: {}", best_soln_count);
            println!("Duration: {:?}", start_time.elapsed());

            // Restart the timer, so that further repetitions don't count the print statements
            start_time = Instant::now();
        }
    }

    let duration = start_time.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn gcd(larger: u32, smaller: u32) -> u32 {
    if smaller == 0 {
        larger
    } else {
        let remainder = larger % smaller;
        gcd(smaller, remainder)
    }
}
