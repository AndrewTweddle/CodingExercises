fn main() {
    let n = 1000;
    let num3s = (n - 1) / 3;
    let num5s = (n - 1) / 5;
    let num15s = (n - 1) / 15;
    let count = 3 * triangular_number(num3s) + 5 * triangular_number(num5s)
        - 15 * triangular_number(num15s);
    println!("{}", count);
}

fn triangular_number(n: u64) -> u64 {
    n * (n + 1) / 2
}
