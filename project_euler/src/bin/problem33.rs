use std::collections::HashSet;
use std::time::Instant;

type Ratio = (usize, usize);

fn main() {
    let start_time = Instant::now();

    let mut digit_cancelling_fractions: HashSet<Ratio> = HashSet::new();

    println!("Digit cancelling fractions found...");
    for numerator in 10..99 {
        for denominator in (numerator + 1)..100 {
            let orig_ratio = simplify((numerator, denominator));
            let (n_tens, n_units) = (numerator / 10, numerator % 10);
            let (d_tens, d_units) = (denominator / 10, denominator % 10);

            if n_units == 0 && d_units == 0 {
                // Skip if trivial
                continue;
            }

            if (n_tens == d_units && ratios_equal((n_units, d_tens), orig_ratio))
                || (n_units == d_tens && ratios_equal((n_tens, d_units), orig_ratio))
                || (n_tens == d_tens && ratios_equal((n_units, d_units), orig_ratio))
                || (n_units == d_units && ratios_equal((n_tens, d_tens), orig_ratio))
            {
                println!("  {} / {}", numerator, denominator);
                digit_cancelling_fractions.insert((numerator, denominator));
            }
        }
    }

    let product_of_ratios = simplify(
        digit_cancelling_fractions
            .into_iter()
            .reduce(|(n1, d1), (n2, d2)| (n1 * n2, d1 * d2))
            .unwrap(),
    );
    println!(
        "Product of ratios (simplified): {} / {}",
        product_of_ratios.0, product_of_ratios.1
    );
    println!("Answer: {}", product_of_ratios.1);

    let duration = start_time.elapsed();
    println!("Duration: {:?}", duration);
}

fn ratios_equal(ratio1: Ratio, ratio2: Ratio) -> bool {
    let (n1, d1) = simplify(ratio1);
    let (n2, d2) = simplify(ratio2);
    n1 == n2 && d1 == d2
}

fn simplify((numerator, denominator): Ratio) -> Ratio {
    let divisor = gcd(numerator, denominator);
    if divisor == 1 {
        (numerator, denominator)
    } else {
        (numerator / divisor, denominator / divisor)
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else if a > b {
        gcd(b, a)
    } else {
        gcd(b % a, a)
    }
}
