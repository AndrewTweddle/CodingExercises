fn main() {
    let smallest_multiple = (2..=20).reduce(lcm).unwrap();
    println!("Smallest multiple of 2 to 20 is {}", smallest_multiple)
}

fn gcd(a: u64, b: u64) -> u64 {
    if a < b {
        gcd(b, a)
    } else {
        let rem = a % b;
        if rem == 0 {
            b
        } else {
            gcd(b, rem)
        }
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
