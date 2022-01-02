use std::time::Instant;

fn main() {
    let start = Instant::now();

    let max_chain_length = solve_without_cache();
    println!("Max chain length = {}", max_chain_length);

    let duration = start.elapsed();
    println!("Elapsed time: {:?}", duration);
}

fn solve_without_cache() -> u64 {
    let mut max_chain_length: u64 = 1;
    for i in 2..1_000_000 {
        let chain_length = get_chain_length(i);
        if max_chain_length < chain_length {
            max_chain_length = chain_length;
            eprintln!("Max chain length {} for n = {}", chain_length, i);
        }
    }
    max_chain_length
}

fn get_chain_length(i: u32) -> u64 {
    let mut chain_length = 1;
    let mut value: u64 = i as u64;
    while value != 1 {
        value = if value % 2 == 0 {
            value / 2
        } else {
            3 * value + 1
        };
        chain_length += 1
    }
    chain_length
}

#[test]
fn test_13() {
    let chain_length = get_chain_length(13);
    assert_eq!(chain_length, 10);
}

/* Runs out of space...
fn solve_with_cache() -> u64 {
    let mut cache: Vec<Option<u64>> = vec![None; 100_000_000];
    cache[1] = Some(1);
    let mut max_chain_length: u64 = 1;
    for i in 2..1_000_000 {
        let chain_length = get_cache_value(&mut cache, i);
        if chain_length > max_chain_length {
            max_chain_length = chain_length;
        }
    }
    max_chain_length
}

fn get_cache_value(cache: &mut Vec<Option<u64>>, value: usize) -> u64 {
    match cache[value] {
        Some(chain_length) => chain_length,
        None => {
            let transformed_value = if value % 2 == 0 { value / 2 } else { 3 * value + 1 };
            if transformed_value >= cache.len() {
                panic!("Cache not large enough");
            }
            let chain_length = get_cache_value(cache, transformed_value) + 1;
            cache[value] = Some(chain_length);
            chain_length
        }
    }
}
*/
