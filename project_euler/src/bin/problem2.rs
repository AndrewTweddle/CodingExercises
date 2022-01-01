fn main() {
    let cutoff = 4_000_000;
    let mut prev = 1;
    let mut curr = 2;
    let mut total = 0;

    while curr < cutoff {
        if curr % 2 == 0 {
            total += curr;
        }
        let new_prev = curr;
        curr += prev;
        prev = new_prev;
    }
    println!("Total = {}", total);
}
