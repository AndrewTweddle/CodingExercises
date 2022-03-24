use std::collections::HashSet;
use std::time::Instant;

type Permutation = [usize; 9];

fn main() {
    let start_time = Instant::now();

    let sum_of_products = get_sum_of_products();
    println!("Sum of products: {sum_of_products}");

    let duration = start_time.elapsed();
    println!("Duration: {duration:?}");
}

fn get_sum_of_products() -> usize {
    let mut permutation: Permutation = [0; 9];
    let mut products = HashSet::<usize>::new();
    populate_products(&mut permutation, &mut products, 0, 9);
    products.iter().sum()
}

fn populate_products(
    permutation: &mut Permutation,
    products: &mut HashSet<usize>,
    bitmask: u16,
    rem_digits: usize,
) {
    let mut digit_bitmask = 1;
    let mut digit = 1;

    // Try each of the remaining digits in the next position of the permutation
    for _ in 0..rem_digits {
        // Find the next unused digit
        while bitmask & digit_bitmask != 0 {
            digit_bitmask <<= 1;
            digit += 1;
        }
        permutation[9 - rem_digits] = digit;

        if rem_digits == 1 {
            // The permutation of 1..9 is complete.
            // So try all possible ways of forming a valid equation.
            check_all_equations_for_permutation(permutation, products);
        } else {
            // Recursively build the next level of the permutation.
            populate_products(
                permutation,
                products,
                bitmask | digit_bitmask,
                rem_digits - 1,
            );
        }

        // Move to the next digit
        digit_bitmask <<= 1;
        digit += 1;
    }
}

fn check_all_equations_for_permutation(
    permutation: &mut Permutation,
    products: &mut HashSet<usize>,
) {
    let first_digit_in_multiplicand = permutation[0];
    for digits_in_multiplicand in 1..=4_usize {
        // Only check numbers where first digit in multiplicand < first digit in multiplier.
        // This is to skip checking commutatively equivalent products.
        let first_digit_in_multiplier = permutation[digits_in_multiplicand];
        if first_digit_in_multiplicand > first_digit_in_multiplier {
            continue;
        }

        let mut multiplicand = 0;
        for i in 0..digits_in_multiplicand {
            multiplicand = 10 * multiplicand + permutation[i];
        }
        for digits_in_multiplier in 1..=4_usize {
            // If there are 6 or more digits in the multiplier or multiplicand,
            // then the product will have too many digits as well...
            if digits_in_multiplier + digits_in_multiplicand > 5 {
                break;
            }

            let mut multiplier = 0;
            for i in 0..digits_in_multiplier {
                multiplier = 10 * multiplier + permutation[digits_in_multiplicand + i];
            }

            let mut product = 0;
            for i in (digits_in_multiplier + digits_in_multiplicand)..9 {
                product = 10 * product + permutation[i];
            }

            if multiplier * multiplicand == product {
                products.insert(product);
            }
        }
    }
}
