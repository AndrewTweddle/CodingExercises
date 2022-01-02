fn main() {
    let opt_triple = find_pythagorean_triple_with_sum(1_000);
    if let Some((a, b, c)) = opt_triple {
        println!("Triple is ({}, {}, {}) with product {}", a, b, c, a * b * c);
    } else {
        println!("Triple not found!");
    }
}

fn find_pythagorean_triple_with_sum(n: u64) -> Option<(u64, u64, u64)> {
    for a in 1..(n / 2) {
        for b in a..(n - a - 1) {
            let c = n - a - b;
            if c <= b {
                continue;
            }
            if a * a + b * b == c * c {
                return Some((a, b, c));
            }
        }
    }
    None
}

/*
With n = 10_000, it takes seconds to complete.

with n = 100_000, it takes a long time to complete (tens of minutes),
producing answer that are all 100 times larger...

Triple is (20000, 37500, 42500) with product 31875000000000

So there is room for finding a faster algorithm.
 */
