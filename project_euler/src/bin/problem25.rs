use std::time::Instant;

const NUM_REPETITIONS: u32 = 10;
const MAX_DIGITS: usize = 1000;

fn main() {
    let start = Instant::now();
    for rep in 0..NUM_REPETITIONS {
        let target_digits = MAX_DIGITS;
        let num = inv_fib(target_digits).unwrap();
        if rep == 0 {
            println!("fib({}) has {} digits", num, target_digits);
        }
    }
    let duration = start.elapsed();
    println!("Avg duration: {:?}", duration / NUM_REPETITIONS);
}

fn inv_fib(target_digits: usize) -> Result<i32, String> {
    if target_digits == 1 {
        return Ok(1);
    } else if target_digits > MAX_DIGITS {
        return Err(format!(
            "Fibonacci numbers above {} digits are not supported: {} found.",
            MAX_DIGITS, target_digits
        ));
    }

    let mut num1: [u8; MAX_DIGITS] = [0; MAX_DIGITS];
    let mut num2: [u8; MAX_DIGITS] = [0; MAX_DIGITS];
    num1[0] = 1;
    num2[0] = 1;
    let mut num1_digits: usize = 1;
    let mut num2_digits: usize = 1;

    // Start calculating the 3rd digit onwards, until fib(n) has more than target_digits
    for n in 3.. {
        // a and b will be the smaller and larger of the 2 previous fibonacci numbers.
        // the vector referenced by a will be overwritten with the sum of a and b.
        // But in the next loop, a and b will switch which vector (num1 or num2) they reference.
        let (a, b, a_digits, b_digits) = if n % 2 == 0 {
            (&mut num1, &mut num2, &mut num1_digits, &mut num2_digits)
        } else {
            (&mut num2, &mut num1, &mut num2_digits, &mut num1_digits)
        };

        let mut carry: u8 = 0;
        for i in 0..*b_digits {
            let sum_digits = a[i] + b[i] + carry;
            a[i] = sum_digits % 10;
            carry = sum_digits / 10;
        }
        if carry > 0 {
            *a_digits = *b_digits + 1;
            if *a_digits == target_digits as usize {
                return Ok(n);
            }
            a[*a_digits - 1] = carry;
        } else {
            *a_digits = *b_digits;
        }
    }

    Err("Unreachable code reached".to_string())
}
