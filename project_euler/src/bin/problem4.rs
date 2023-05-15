fn main() {
    let mut max_palindrome = 0;
    for i in (100..999).rev() {
        'smaller: for j in (100..=i).rev() {
            let prod = i * j;
            let digits = get_rev_digits(prod);
            let num_digits = digits.len();
            for k in 0..(num_digits / 2) {
                if digits[k] != digits[num_digits - k - 1] {
                    continue 'smaller;
                }
            }
            max_palindrome = max_palindrome.max(prod);
        }
    }
    println!("Max palindrome = {}", max_palindrome);
}

// Assume n > 0...
fn get_rev_digits(n: i64) -> Vec<i64> {
    let mut rev_digits = Vec::<i64>::new();
    let mut quot = n;
    while quot > 0 {
        let rem = quot % 10;
        rev_digits.push(rem);
        quot /= 10;
    }
    rev_digits
}
