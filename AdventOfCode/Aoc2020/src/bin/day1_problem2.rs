use std::collections::{HashSet, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "data/day1_input";
    let input = File::open(path).unwrap();
    let br = BufReader::new(input);

    let mut nums = HashSet::<u32>::new();
    let mut pairs_by_sum = HashMap::<u32, (u32, u32)>::new();

    let mut triple = None;
    let numbers = br.lines().map(|ln| ln.unwrap().parse::<u32>().unwrap());
    for n in numbers {
        if n > 2020 {
            continue;
        }

        // Look for a triple
        let target = 2020 - n;
        if let Some((n1, n2)) = pairs_by_sum.get(&target) {
            triple = Some((*n1, *n2, n));
            break;
        }

        // Form pairs
        for &n2 in &nums {
            let new_pair_sum = n + n2;
            if new_pair_sum > 2020 {
                continue;
            }
            if pairs_by_sum.contains_key(&new_pair_sum) {
                continue;
            }
            if n < n2 {
                pairs_by_sum.insert(new_pair_sum, (n, n2));
            } else {
                pairs_by_sum.insert(new_pair_sum, (n2, n));
            }
        }

        // Add the number to the set of single numbers
        nums.insert(n);
    }
    let triple = triple.expect("No triple adding up to 2020 was found");
    println!("Product of {} and {} and {} = {}", triple.0, triple.1, triple.2,
             triple.0 * triple.1 * triple.2);
}
