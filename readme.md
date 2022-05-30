# Overview

This project contains my solutions to various coding exercises.

These include exercises from:
  * Katas from [CodingDojo.org](https://codingdojo.org/)
  * [Project Euler](https://projecteuler.net)
  * [Advent of Code](https://adventofcode.com/)
  * [CSES Problem Set](https://cses.fi/problemset/list/) 
  * [cryptopals](https://cryptopals.com/) crypto challenges
  * Exercises from the book "Cracking the Coding Interview" (a.k.a. CTCI)

I use these to: 
  * practise coding skills,
  * learn new programming languages,
  * compare languages or paradigms,
  * prepare for technical interviews, and
  * have fun solving coding puzzles!

Other similar repositories:
  * [AndrewTweddle/GoogleCodeJam](https://github.com/AndrewTweddle/GoogleCodeJam)
    * submissions to the Google CodeJam algorithmic coding competition
  * [AndrewTweddle/CodingChallenges](https://github.com/AndrewTweddle/CodingChallenges):
    * including entries to the [CodeForces](https://codeforces.com/) algorithmic coding competitions
  * [AndrewTweddle/fpinscala](https://github.com/AndrewTweddle/fpinscala):
    * forked exercises from the book ["Functional Programming in Scala"](http://www.manning.com/bjarnason/)

# Various exercises

## Katas from the CodingDojo.org web site

| Exercise                                                     | Description                                    | Date       | Solution                                                                                                             | Notes                                                                                                                    |
|--------------------------------------------------------------|------------------------------------------------|------------|----------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| [Bowling scorer](https://codingdojo.org/kata/Bowling/)       | Ten pin bowling scoring algorithm              | 2015-09-25 | [C# (v1)](Katas/CodingDojo/Bowling/DotNet/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer.cs)              | [readme](Katas/CodingDojo/Bowling/DotNet/src/main/csharp/AndrewTweddle.Katas.Bowling)                                    |
|                                                              |                                                | 2015-09-25 | [C# (v2 - stateless)](Katas/CodingDojo/Bowling/DotNet/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer2.cs) |                                                                                                                          |
|                                                              |                                                | 2015-09-27 | [C# (v3 - immutable)](Katas/CodingDojo/Bowling/DotNet/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer3.cs) |                                                                                                                          |
|                                                              |                                                | 2015-09-27 | [F#](Katas/CodingDojo/Bowling/DotNet/src/main/fsharp/AndrewTweddle.Katas.Bowling.FSharp/FunctionalBowlingScorer.fs)  |                                                                                                                          |
|                                                              |                                                | 2015-09-25 | [unit tests](Katas/CodingDojo/Bowling/DotNet/src/test/csharp/AndrewTweddle.Katas.Test.Bowling)                       |                                                                                                                          |
| [Roman Numerals](https://codingdojo.org/kata/RomanNumerals/) | Convert to and from Roman numerals up to 3000. | 2021-06-01 | [README file](Katas/CodingDojo/RomanNumerals/README.md)                                                              | This discusses the various experiments below.                                                                            |
|                                                              |                                                | 2021-05-11 | [Rust (TDD)](Katas/CodingDojo/roman_numerals/src/main.rs)                                                            | Experiment with TDD and with quickcheck property-based testing.                                                          |
|                                                              |                                                | 2021-05-24 | [Rust v2](Katas/CodingDojo/RomanNumerals/Rust/roman_numerals_v2/src/main.rs)                                         | Designed intuitively, not iteratively to compare with the TDD approach. Experiment with proptest property-based testing. |
|                                                              |                                                | 2021-06-02 | [Rust v3](Katas/CodingDojo/RomanNumerals/Rust/roman_numerals_v3/src/main.rs)                                         | A simpler approach (unfortunately not mine - I saw others doing this, and rewrote it in Rust).                           |

## Project Euler problems

Project Euler requests that solutions not be shared online for problems beyond #100.

| #   | Description                                                               | Date       | Solution                                                             | Notes                                                                                                                                                                                                                        |
|-----|---------------------------------------------------------------------------|------------|----------------------------------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| 1   | [Multiples of 3 and 5](https://projecteuler.net/problem=1)                | 2021-04-11 | [Rust](project_euler/src/bin/problem1.rs)                            |                                                                                                                                                                                                                              |
| 2   | [Even Fibonacci Numbers](https://projecteuler.net/problem=2)              | 2021-04-12 | [Rust](project_euler/src/bin/problem2.rs)                            |                                                                                                                                                                                                                              |
| 3   | [Largest Prime Factor](https://projecteuler.net/problem=3)                | 2021-04-12 | [Rust](project_euler/src/bin/problem3.rs)                            | Experimented with, and timed, various methods of calculating primes.                                                                                                                                                         |
| 4   | [Largest Palindrome Product](https://projecteuler.net/problem=4)          | 2021-04-12 | [Rust](project_euler/src/bin/problem4.rs)                            |                                                                                                                                                                                                                              |
| 5   | [Smallest Multiple](https://projecteuler.net/problem=5)                   | 2021-04-12 | [Rust](project_euler/src/bin/problem5.rs)                            | Calculate lcm of 2 to 20 using gcd and reduce.                                                                                                                                                                               |
|     |                                                                           | 2021-04-12 | [Rust](project_euler/src/bin/problem5_v2.rs)                         | Calculate lcm of 2 to 20 using primes.                                                                                                                                                                                       |
| 6   | [Sum square difference](https://projecteuler.net/problem=6)               | 2021-04-12 | [Rust](project_euler/src/bin/problem6.rs)                            | Very short and creative solution: sum of squares - square of sums = sum of squares - sum of cubes.                                                                                                                           |
| 7   | [10001st prime](https://projecteuler.net/problem=7)                       | 2021-04-13 | [Rust](project_euler/src/bin/problem7.rs)                            |                                                                                                                                                                                                                              |
| 8   | [Largest product in a series](https://projecteuler.net/problem=8)         | 2021-04-13 | [Rust](project_euler/src/bin/problem8.rs)                            |                                                                                                                                                                                                                              |
| 9   | [Special Pythagorean triplet](https://projecteuler.net/problem=9)         | 2021-04-14 | [Rust](project_euler/src/bin/problem9.rs)                            |                                                                                                                                                                                                                              |
| 10  | [Summation of primes](https://projecteuler.net/problem=10)                | 2021-04-15 | [Rust](project_euler/src/bin/problem10.rs)                           | Includes performance comparison of getting primes using 3 variants on the Sieve of Eratosthenes (see [Wikipedia](https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes#Algorithm_and_variants)).                               |
| 11  | [Largest product in a grid](https://projecteuler.net/problem=11)          | 2021-04-16 | [Rust](project_euler/src/bin/problem11.rs)                           |                                                                                                                                                                                                                              |
| 12  | [Highly divisible triangular number](https://projecteuler.net/problem=12) | 2021-04-17 | [Rust](project_euler/src/bin/problem12.rs)                           | Duration: 182.175505ms. Original attempt. Succinct but slow, because `i*i <= n` is performed repeatedly in the innermost loop.                                                                                               |
|     |                                                                           | 2021-09-06 | [Rust](project_euler/src/bin/problem12_sqrt_bounds_binary_search.rs) | Duration: 133.347543ms. Do binary search for integer square root based on lower and upper integer bounds. Still slow, even though int sqrt is outside the inner loop.                                                        |
|     |                                                                           | 2021-09-06 | [Rust](project_euler/src/bin/problem12_rel_prime_divisors.rs)        | Duration: 2.400409ms. I had a brainwave! T(n) = n*(n+1)/2. n and n+1 are relatively prime. Hence # of divisors of T(n) is the product of # of divisors of n/2 and n+1 (if n is even) or n and (n+1)/2 if odd.                |
|     |                                                                           | 2021-09-06 | [Rust](project_euler/src/bin/problem12_prime_factors.rs)             | Duration: 1.540086ms. As above, but also pre-generate a few primes and further decompose each divisor by these primes before counting the # of divisors. 30 makes a good cut-off, so factorize up to 29.                     |
|     |                                                                           | 2021-09-06 | [Rust](project_euler/src/bin/problem12_const_prime_factors.rs)       | Duration: 852.896µs. As above, but hard-coding the primes up to 31, instead of calculating them on the fly.                                                                                                                  |
|     |                                                                           | 2021-09-06 | [Rust](project_euler/src/bin/problem12_primes_6q_plus_r.rs)          | Duration: 733.94µs. As above, but only count divisors of the remainder (after reducing by primes up to 31) of the form 6q+1 or 6q+5.                                                                                         |
|     |                                                                           | 2021-09-06 | [Rust](project_euler/src/bin/problem12_primes_30q_plus_r.rs)         | Duration: 847.894µs. As above, but only count divisors of the remainder (after reducing by small primes) of the form 30q+r for suitable r. This is slower for n=500, but scales well for much higher values.                 |
|     |                                                                           | 2021-09-06 | [Rust](project_euler/src/bin/problem12_const_prime_factors_mpsc.rs)  | Duration: 5.59817ms. Using hard-coded primes, with mpsc (5 threads works well on my PC). Perhaps unsurprisingly, this is much slower since we are not I/O bound. It catches up for very large n (> 10,000).                  |
|     |                                                                           | 2021-09-06 | [Rust](project_euler/src/bin/problem12_repeated_factorization.rs)    | Duration:  1.734901ms. Instead of pre-computing or hard-coding primes, completely factorize into primes on the fly. Although slower, this solution is shorter and more elegant.                                              |
| 13  | [Large sum](https://projecteuler.net/problem=13)                          | 2021-04-17 | [Rust](project_euler/src/bin/problem13.rs)                           |                                                                                                                                                                                                                              |
| 14  | [Longest Collatz sequence](https://projecteuler.net/problem=14)           | 2021-04-19 | [Rust](project_euler/src/bin/problem14.rs)                           |                                                                                                                                                                                                                              |
| 15  | [Lattice paths](https://projecteuler.net/problem=15)                      | 2021-04-30 | [Rust](project_euler/src/bin/problem15.rs)                           | Duration: 13.432µs. Calculate combinatorial without overflow by aggressively cancelling common factors from numerators & denominators.                                                                                       |
|     |                                                                           | 2021-05-02 | [Rust](project_euler/src/bin/problem15_diagonal_sweep.rs)            | Duration: 18.734µs. Calculate paths from top left to bottom right by doing a "diagonal sweep", calculating distances to each point on a diagonal based on the previous diagonal.                                             |
|     |                                                                           | 2021-05-02 | [Rust](project_euler/src/bin/problem15_fast_combinatorial.rs)        | Duration: 11.358µs. I converted someone else's C++ solution to Rust. Calculate a combinatorial by generating and iterating through all relevant primes and calculating its contribution to each factorial.                   |
| 16  | [Power digit sum](https://projecteuler.net/problem=16)                    | 2021-05-02 | [Rust](project_euler/src/bin/problem16.rs)                           | Duration: 13.081µs. Problem: Sum the decimal digits in 2^1000. Solution: Use base 10^19 < 2^64. Store digits as u128. So squaring a digit and adding a carry will not overflow. 2^1000 = ((((2^125)^2)^2)^2.                 |
|     |                                                                           | 2021-05-02 | [Rust](project_euler/src/bin/problem16_decimal_digits.rs)            | Duration: 951.85µs. By contrast, calculating one decimal digit at a time is very slow, showing how neat the previous solution is! However, it is much simpler and shorter to do one decimal digit at a time.                 |
|     |                                                                           | 2021-05-03 | [Rust](project_euler/src/bin/problem16_bigint.rs)                    | Duration: 5.263µs. Using the num-bigint crate.                                                                                                                                                                               |
|     |                                                                           | 2021-05-04 | [Rust](project_euler/src/bin/problem16_bigint_fast.rs)               | Duration: 4.87µs. Using the num-bigint crate and calculating digits by dividing by 10 and taking remainders, not by converting to a string and iterating over chars.                                                         |
|     |                                                                           | 2021-05-10 | [Rust](project_euler/src/bin/problem16_rayon.rs)                     | Duration: 45.801µs. Using the original approach, but summing the digits in parallel using the rayon crate (as a way of learning rayon). This was MUCH slower.                                                                |
|     |                                                                           | 2021-05-10 | [Rust](project_euler/src/bin/problem16_original_fast.rs)             | Duration: 10.767µs. Using the original approach again, but calculating the digits by dividing by 10 and taking remainders.                                                                                                   |
| 17  | [Number Letter Counts](https://projecteuler.net/problem=17)               | 2021-05-20 | [Rust](project_euler/src/bin/problem17_tdd_collab.rs)                | Duration: 373.045µs. TDD solution pair programmed with a colleague (for him to show me how he does TDD, and for me to show him Rust). proptest crate also used for property-based testing of the algorithm.                  |
|     |                                                                           | 2021-05-20 | [Rust](project_euler/src/bin/problem17.rs)                           | Duration: 9ns. Coded on my own using a more intuitive approach. This was simple, succinct and lightning fast, but also brittle and tricky to get right.                                                                      |
| 18  | [Maximum path sum I](https://projecteuler.net/problem=18)                 | 2021-05-31 | [Rust](project_euler/src/bin/problem18.rs)                           | Duration: 1.228µs.                                                                                                                                                                                                           |
|     |                                                                           | 2021-05-31 | [Rust](project_euler/src/bin/problem18_combinators.rs)               | Duration: 1.915µs. Same algorithm, but using combinators instead of manual loops.                                                                                                                                            |
| 19  | [Counting Sundays](https://projecteuler.net/problem=19)                   | 2021-06-01 | [Rust](project_euler/src/bin/problem19.rs)                           |                                                                                                                                                                                                                              |
| 20  | [Factorial digit sum](https://projecteuler.net/problem=20)                | 2021-06-06 | [Rust](project_euler/src/bin/problem20.rs)                           | Duration: 15.009µs. Problem: Sum of digits in 100! Solution: Calculate factorial using a vector representing digits in base 10^36, since 100*(10^36-1)+max_carry_digit will not overflow a u128.                             |
|     |                                                                           | 2021-06-06 | [Rust](project_euler/src/bin/problem20_prime_factors.rs)             | Duration: 18.875µs. Determine factorial from prime factors, one prime at a time. Since n/p numbers in n! have p as a factor, n/(p^2) have another factor p, etc. Use base 10^19 to avoid overflowing a u128.                 |
|     |                                                                           | 2022-01-03 | [Rust](project_euler/src/bin/problem20_bigint.rs)                    | Duration: 3.221µs. Using the num-bigint crate.                                                                                                                                                                               |
| 21  | [Amicable numbers](https://projecteuler.net/problem=21)                   | 2021-07-04 | [Rust](project_euler/src/bin/problem21.rs)                           | Duration: 419.454µs. Two similar solutions are provided. The one using a lookup table is about 6000 times faster.                                                                                                            |
|     |                                                                           | 2021-07-04 | [Rust](project_euler/src/bin/problem21_using_primes.rs)              | Duration: 74.144837ms. Uses primes. Suppose n has prime factorization p1^e1 * p2^e2 * ... * pk^ek. then the sum of divisors of n is (1+p1+p1^2+...+p1^e1) * (1+p2+p2^2+...+p2^e2) * ... * (1+pk+pk^2+...+pk^ek).             |
| 22  | [Names scores](https://projecteuler.net/problem=22)                       | 2021-07-17 | [Rust](project_euler/src/bin/problem22.rs)                           |                                                                                                                                                                                                                              |
| 23  | [Non-abundant sums](https://projecteuler.net/problem=23)                  | 2021-08-23 | [Rust](project_euler/src/bin/problem23.rs)                           | Duration: 1.176304453s. Simple but slow approach.                                                                                                                                                                            |
|     |                                                                           | 2021-08-23 | [Rust](project_euler/src/bin/problem23_sum_abund_pairs.rs)           | Duration: 26.661017ms. More verbose, but MUCH faster!                                                                                                                                                                        |
| 24  | [Lexicographic permutations](https://projecteuler.net/problem=24)         | 2021-08-23 | [Rust](project_euler/src/bin/problem24.rs)                           | Duration: 385ns.                                                                                                                                                                                                             |
|     |                                                                           | 2021-08-23 | [Rust](project_euler/src/bin/problem24_mixed_radix.rs)               | Duration: 343ns. Flash of inspiration here! The millionth lexicographic sort of the 10 decimal digits is simply 999,999 expressed in the [factorial number system](https://en.wikipedia.org/wiki/Factorial_number_system)!   |
| 25  | [1000-digit Fibonacci number](https://projecteuler.net/problem=25)        | 2021-09-25 | [Rust](project_euler/src/bin/problem25.rs)                           |                                                                                                                                                                                                                              |
| 26  | [Reciprocal cycles](https://projecteuler.net/problem=26)                  | 2021-10-19 | [Rust](project_euler/src/bin/problem26.rs)                           | Duration: 1.376608ms.                                                                                                                                                                                                        |
|     |                                                                           | 2021-10-20 | [Rust](project_euler/src/bin/problem26_fast.rs)                      | Duration: 966.352µs.                                                                                                                                                                                                         |
| 27  | [Quadratic primes](https://projecteuler.net/problem=27)                   | 2021-11-14 | [Rust](project_euler/src/bin/problem27.rs)                           |                                                                                                                                                                                                                              |
| 28  | [Number spiral diagonals](https://projecteuler.net/problem=28)            | 2021-11-17 | [Rust](project_euler/src/bin/problem28.rs)                           |                                                                                                                                                                                                                              |
| 29  | [Distinct powers](https://projecteuler.net/problem=29)                    | 2021-11-17 | [Rust](project_euler/src/bin/problem29.rs)                           | Duration: 2.703525ms. Problem: count distinct a^b for 2 ≤ a ≤ 100 and 2 ≤ b ≤ 100. Solution: Generate prime representation of a's, multiply exponents by each b. Add to a set to eliminate duplicates.                       |
|     |                                                                           | 2021-11-17 | [Rust](project_euler/src/bin/problem29_bruteforce.rs)                | Duration: 2.472783ms. Alternate solution: Represent a's as a Vec<u128> of digits in base 2^120 (so that a u128 digit doesn't overflow if multiplied by 100 or any smaller value of a). Add to a set to eliminate duplicates. |
|     |                                                                           | 2022-01-03 | [Rust](project_euler/src/bin/problem29_bigint.rs)                    | Duration: 2.622977ms. Using the num-bigint crate.                                                                                                                                                                            |
| 30  | [Digit fifth powers](https://projecteuler.net/problem=30)                 | 2022-01-06 | [Rust](project_euler/src/bin/problem30.rs)                           | Duration: 8.935239ms.                                                                                                                                                                                                        |
| 31  | [Coin sums](https://projecteuler.net/problem=31)                          | 2022-03-22 | [Rust](project_euler/src/bin/problem31.rs)                           | Duration: 215.894µs.                                                                                                                                                                                                         |
| 32  | [Pandigital products](https://projecteuler.net/problem=32)                | 2022-03-23 | [Rust](project_euler/src/bin/problem32.rs)                           | Duration: 28.314084ms.                                                                                                                                                                                                       |
| 33  | [Digit cancelling fractions](https://projecteuler.net/problem=33)         | 2022-03-24 | [Rust](project_euler/src/bin/problem33.rs)                           | Duration: 285.596µs.                                                                                                                                                                                                         |
| 34  | [Digit factorials](https://projecteuler.net/problem=34)                   | 2022-05-28 | [Rust](project_euler/src/bin/problem34.rs)                           | Duration: 9.102047ms.                                                                                                                                                                                                        |

## Advent of Code

### Learning by comparison

Many participants share their solutions on the reddit [AOC mega-thread](https://www.reddit.com/r/adventofcode/wiki/solution_megathreads).

This can be very useful for:
* Comparing solution approaches.
* Learning clever tricks and handy features of the language and libraries.
* Comparing readability of different coding styles and languages.
* Comparing performance of solutions.

### 2020

|Day                                           | Part |Date Solved | My solution                                           |
|----------------------------------------------|------|------------|-------------------------------------------------------|
| [Day 1](https://adventofcode.com/2020/day/1) | 1    | 2021-11-18 | [Rust](AdventOfCode/Aoc2020/src/bin/day1_problem1.rs) | 
|                                              | 2    | 2021-11-18 | [Rust](AdventOfCode/Aoc2020/src/bin/day1_problem2.rs) |
| [Day 2](https://adventofcode.com/2020/day/2) | 1    | 2021-11-20 | [Rust](AdventOfCode/Aoc2020/src/bin/day2_problem1.rs) |
|                                              | 2    | 2021-11-20 | [Rust](AdventOfCode/Aoc2020/src/bin/day2_problem2.rs) |
| [Day 3](https://adventofcode.com/2020/day/3) | 1    | 2021-11-27 | [Rust](AdventOfCode/Aoc2020/src/bin/day3_problem1.rs) |
|                                              | 2    | 2021-11-27 | [Rust](AdventOfCode/Aoc2020/src/bin/day3_problem2.rs) |

### 2021

| Day                                        | Part  | Date Solved | My solution                                                     | Notes                                                                                                       |
|--------------------------------------------|-------|-------------|-----------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------|
| [1](https://adventofcode.com/2021/day/1)   | 1     | 2021-12-01  | [Rust](AdventOfCode/aoc2021/src/bin/day1_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-01  | [Rust](AdventOfCode/aoc2021/src/bin/day1_problem2.rs)           |                                                                                                             |
| [2](https://adventofcode.com/2021/day/2)   | 1     | 2021-12-02  | [Rust](AdventOfCode/aoc2021/src/bin/day2_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-02  | [Rust](AdventOfCode/aoc2021/src/bin/day2_problem2.rs)           |                                                                                                             |
| [3](https://adventofcode.com/2021/day/3)   | 1     | 2021-12-04  | [Rust](AdventOfCode/aoc2021/src/bin/day3_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-04  | [Rust](AdventOfCode/aoc2021/src/bin/day3_problem2.rs)           |                                                                                                             |
| [4](https://adventofcode.com/2021/day/4)   | 1     | 2021-12-04  | [Rust](AdventOfCode/aoc2021/src/bin/day4_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-04  | [Rust](AdventOfCode/aoc2021/src/bin/day4_problem2.rs)           |                                                                                                             |
| [5](https://adventofcode.com/2021/day/5)   | 1     | 2021-12-05  | [Rust](AdventOfCode/aoc2021/src/bin/day5_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-05  | [Rust](AdventOfCode/aoc2021/src/bin/day5_problem2.rs)           |                                                                                                             |
| [6](https://adventofcode.com/2021/day/6)   | 1     | 2021-12-06  | [Rust](AdventOfCode/aoc2021/src/bin/day6_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-06  | [Rust](AdventOfCode/aoc2021/src/bin/day6_problem2.rs)           |                                                                                                             |
| [7](https://adventofcode.com/2021/day/7)   | 1     | 2021-12-07  | [Rust](AdventOfCode/aoc2021/src/bin/day7_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-07  | [Rust](AdventOfCode/aoc2021/src/bin/day7_problem2.rs)           |                                                                                                             |
| [8](https://adventofcode.com/2021/day/8)   | 1     | 2021-12-08  | [Rust](AdventOfCode/aoc2021/src/bin/day8_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-08  | [Rust](AdventOfCode/aoc2021/src/bin/day8_problem2.rs)           |                                                                                                             |
| [9](https://adventofcode.com/2021/day/9)   | 1     | 2021-12-09  | [Rust](AdventOfCode/aoc2021/src/bin/day9_problem1.rs)           |                                                                                                             |
|                                            | 2     | 2021-12-09  | [Rust](AdventOfCode/aoc2021/src/bin/day9_problem2.rs)           |                                                                                                             |
| [10](https://adventofcode.com/2021/day/10) | 1     | 2021-12-10  | [Rust](AdventOfCode/aoc2021/src/bin/day10_problem1.rs)          |                                                                                                             |
|                                            | 2     | 2021-12-10  | [Rust](AdventOfCode/aoc2021/src/bin/day10_problem2.rs)          |                                                                                                             |
| [11](https://adventofcode.com/2021/day/11) | 1 & 2 | 2021-12-11  | [Rust](AdventOfCode/aoc2021/src/bin/day11_problem1and2.rs)      |                                                                                                             |
| [12](https://adventofcode.com/2021/day/12) | 1     | 2021-12-12  | [Rust](AdventOfCode/aoc2021/src/bin/day12_problem1.rs)          |                                                                                                             |
|                                            | 2     | 2021-12-12  | [Rust](AdventOfCode/aoc2021/src/bin/day12_problem2.rs)          |                                                                                                             |
| [13](https://adventofcode.com/2021/day/13) | 1     | 2021-12-13  | [Rust](AdventOfCode/aoc2021/src/bin/day13_problem1.rs)          |                                                                                                             |
|                                            | 2     | 2021-12-13  | [Rust](AdventOfCode/aoc2021/src/bin/day13_problem2.rs)          |                                                                                                             |
| [14](https://adventofcode.com/2021/day/14) | 1     | 2021-12-14  | [Rust](AdventOfCode/aoc2021/src/bin/day14_problem1.rs)          |                                                                                                             |
|                                            | 2     | 2021-12-14  | [Rust](AdventOfCode/aoc2021/src/bin/day14_problem2_attempt1.rs) | Attempt 1: Would take 20.5 hours to run.                                                                    |
|                                            | 2     | 2021-12-15  | [Rust](AdventOfCode/aoc2021/src/bin/day14_problem2_attempt2.rs) | Attempt 2: 205.371371ms using nalgebra 0.30 crate with f64 matrix elements.                                 |
|                                            | 2     | 2021-12-18  | [Rust](AdventOfCode/aoc2021/src/bin/day14_problem2_attempt3.rs) | Attempt 3: 3.37s. Handwritten linear algebra. Messy. My Rust skills aren't good enough for this yet!        |
|                                            | 2     | 2021-12-18  | [Rust](AdventOfCode/aoc2021/src/bin/day14_problem2_attempt4.rs) | Attempt 4: 82µs!                                                                                            |
|                                            | 2     | 2021-12-18  | [Rust](AdventOfCode/aoc2021/src/bin/day14_problem2_attempt5.rs) | Attempt 5: 446µs. Like attempt 4, but using BTreeMap instead of arrays, due to sparsity, but it was slower. |
|                                            | 2     | 2022-01-02  | [Rust](AdventOfCode/aoc2021/src/bin/day14_problem2_attempt6.rs) | Attempt 6: 1.25090696s using nalgebra 0.30 crate with usize matrix elements. Much slower, unfortunately.    |
| [15](https://adventofcode.com/2021/day/15) | 1     | 2021-12-20  | [Rust](AdventOfCode/aoc2021/src/bin/day15_problem1.rs)          |                                                                                                             |
|                                            | 2     | 2021-12-20  | [Rust](AdventOfCode/aoc2021/src/bin/day15_problem2.rs)          |                                                                                                             |
| [16](https://adventofcode.com/2021/day/16) | 1     | 2021-12-23  | [Rust](AdventOfCode/aoc2021/src/bin/day16_problem1.rs)          | Feels quite elegant (albeit verbose, and without enough checking for invalid parse formats).                |
|                                            | 2     | 2021-12-23  | [Rust](AdventOfCode/aoc2021/src/bin/day16_problem2.rs)          | Part 2 easily accommodated. 544 LOC exactly evenly split between 272 lines of code and 272 of unit tests.   |
| [17](https://adventofcode.com/2021/day/17) | 1     | 2022-01-15  | [Rust](AdventOfCode/aoc2021/src/bin/day17_problem1.rs)          | Use triangular numbers for bounds of velocities. Then use brute force simulation (avoiding tricky math).    |
|                                            | 2     | 2022-01-15  | [Rust](AdventOfCode/aoc2021/src/bin/day17_problem2.rs)          |                                                                                                             |
| [18](https://adventofcode.com/2021/day/18) | 1 & 2 | 2022-03-06  | [Rust](AdventOfCode/aoc2021/src/bin/day18_problem1and2.rs)      |                                                                                                             |

## CSES Problem Set

### Introductory Problems

| #   | Description                                             | Date       | Solution                                     | Notes |
|-----|---------------------------------------------------------|------------|----------------------------------------------|-------|
| 1   | [Weird Algorithm](https://cses.fi/problemset/task/1068) | 2022-05-28 | [Rust](cses_rust/src/bin/weird_algorithm.rs) |       |
| 2   | [Missing Number](https://cses.fi/problemset/task/1083/) | 2022-05-28 | [Rust](cses_rust/src/bin/missing_number.rs)  |       |
| 3   | [Repetitions](https://cses.fi/problemset/task/1069)     | 2022-05-29 | [Rust](cses_rust/src/bin/repetitions.rs)     |       |

## cryptopals crypto challenges

### Solutions for [Set 1](https://cryptopals.com/sets/1)

| Challenge | Description                                                               | Date Solved | Solution                                                  | Notes                                                                                                                    |
|-----------|---------------------------------------------------------------------------|-------------|-----------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| #1        | [Convert hex to base64](https://cryptopals.com/sets/1/challenges/1)       | 2021-11-20  | [Rust](cryptopals/set1/src/bin/challenge1.rs)             |                                                                                                                          |
| #2        | [Fixed XOR](https://cryptopals.com/sets/1/challenges/2)                   | 2021-12-01  | [Rust](cryptopals/set1/src/bin/challenge2.rs)             |                                                                                                                          |
| #3        | [Single-byte XOR cipher](https://cryptopals.com/sets/1/challenges/3)      | 2021-12-03  | [Rust](cryptopals/set1/src/bin/challenge3.rs)             | Order histogram of chars and find min variance of positions from frequency histogram.                                    |
|           |                                                                           | 2022-01-15  | [Rust](cryptopals/set1/src/bin/challenge3_alpha_count.rs) | Count lowercase ASCII letters twice, and uppercase once and use the key with the max score.                              |
| #4        | [Detect single-character XOR](https://cryptopals.com/sets/1/challenges/4) | 2022-01-15  | [Rust](cryptopals/set1/src/bin/challenge4.rs)             | Naive ASCII score fails. More complex ascii score works, taking 15 ms. Letter frequency histogram approach takes 131 ms. |
| #5        | [Implement repeating-key XOR](https://cryptopals.com/sets/1/challenges/5) | 2022-05-29  | [Rust](cryptopals/set1/src/bin/challenge5.rs)             | The repeating key XOR encryption function was added to lib.rs.                                                           |

## Miscellaneous

| Exercise                                                                  | Description                                                     | Date       | Solution                                                                                                           |
|---------------------------------------------------------------------------|-----------------------------------------------------------------|------------|--------------------------------------------------------------------------------------------------------------------|
| [wordrect](misc/WordRectangles) - [readme](misc/WordRectangles/readme.md) | Largest rectangle of letters with words in every row and column | 2020-01-19 | [Scala prototype (v1)](misc/WordRectangles/Scala_v1/src/main/scala/com/andrewtweddle/wordrects/WordRectMain.scala) |
|                                                                           |                                                                 | 2020-01-25 | [C++ prototype (v1)](misc/WordRectangles/Cpp_v1/main.cpp)                                                          | 

## CTCI

The CTCI sub-folder contains my solutions to exercises from the 5th edition of [Cracking the Coding Interview](http://www.amazon.com/Cracking-Coding-Interview-Programming-Questions/dp/098478280X) by Gayle Laakmann McDowell.

| Exercise | Description                                               | My solutions                                                                                                                                       |
|----------|-----------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------|
| 1.1      | Check for duplicate letters in a string                   | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise1/DuplicateLetterChecker.cs)                                                  |
| 1.3      | Check if two strings are permutations                     | [C# (readable)](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise3/SimplePermutationChecker.cs)                                     |
|          |                                                           | [C# (fast)](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise3/QuickPermutationChecker.cs)                                          |
| 2.1      | Remove duplicate letters from a linked list               | [C# (fast)](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter2/Exercise1/QuickDuplicateLetterRemover.cs)                                      |
|          |                                                           | [C# (without temporary buffer)](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter2/Exercise1/DuplicateLetterRemoverWithoutTemporaryBuffer.cs) |
| 3.1      | Implement multiple stacks in a single array               | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter3/Exercise1/StackArray.cs)                                                              |
| 3.1      | Repeat as a challenge, using array space more effectively | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter3/Exercise1Challenge/Stacker.cs)                                                        |
| 4.1      | Determine if a binary tree is balanced                    | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter4/Exercise1/BinaryTreeBalanceChecker.cs)                                                |
| 5.1      | Overwrite bits in one int from another                    | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter5/Exercise1/BitInserter.cs)                                                             |
| 9.1      | Number of different ways of hopping up stairs             | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter9/Exercise1/StepsSolver.cs)                                                             |
| 1.4      | Replacing spaces with "%20" in a string                   | [Scala](CTCI/Scala/src/main/scala/ctci/chapter1/Exercise4.scala)                                                                                   |
| 2.2      | kth to last node in a linked list                         | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter2/Exercise2/Node.cs)                                                                    |

## Functional Programming In Scala

Exercises from the book
["Functional Programming in Scala"](http://www.manning.com/bjarnason/)
are in [a separate GitHub project](https://github.com/AndrewTweddle/fpinscala)
which was forked from [the original FPInScala repo](https://github.com/fpinscala/fpinscala)
for the book.

# Learnings and notes

## General approach

_Note: This is the approach I was experimenting with in 2015. It can be quite onerous and slow (compared to a more intuitive approach)._

### Follow a miniaturised SDLC

* Analysis
  * Summarize the problem statement (preferably highlighting critical clauses)
  * Confirm the scope
  * Question the requirements
  * Look for simplifying assumptions
  * List any issues that complicate things (and consider unit testing these earlier)
* Design
  * Brainstorm a number of solutions
    * Count number of steps and estimate the big-O scalability of each
    * Decide which solution to implement
      * Check the stakeholder's preferences when there is a trade-off (e.g. speed versus maintainability)
      * Otherwise favour clarity over cleverness
    * Look for utility methods which could be useful across multiple solutions
  * Do UML designs if necessary
  * Do pseudocode if useful
    * Be alert for useful utility methods to make the main algorithm more readable
* Code
  * If there are multiple algorithms to implement, create a base class and a derived class per algorithm
* Unit test
  * If there are multiple algorithms to test, create a base class and a derived unit test class per implementation
* Code review
  * Look at the code critically
  * Refactor, add TODO's or make a note of areas that could be improved
* Compare
  * If a commonly used Kata (e.g. the bowling game), see how others have solved it and compare to my solution

### Use a notebook to write all code on paper first

* Ideally I wanted to use my whiteboard for all steps in this mini SDLC, but it was already in use
* Instead I used a notebook to do analysis, design, coding and unit testing
* Once done I would capture and compile the code and unit tests electronically
  * An exception was exercise 4.1, which I coded directly.
  * The quality of the code in this exercise is noticeably worse, so pre-planning on paper is clearly worthwhile
  * I was initially skeptical of doing whiteboard coding in interviews, as this is not common practice in South Africa
  * However I am already seeing benefits from doing this:
    * Improved designs through adding greater structure and focus to the creative process
    * An improved ability to "play out" detailed coding scenarios in my head
* I then fixed coding errors or refactored the code and made notes on these errors and refactorings
* After each exercise I add any language-specific notes and reminders to the sections below

  
## C\# 

| Purpose   | Solution              | Notes                      |
| ---       | ---                   | ---                        |
| Sets      | HashSet&lt;T&gt;      | Useful methods: Add and Contains. |
| Hash maps | Dictionary&lt;K, V&gt;   | Useful methods: ContainsKey, Keys, Values, enumerator over KeyValuePair&lt;K, V&gt; |
| Test multiple implementations | MSTest: [TestMethod] on base class methods, [TestClass] on derived class | The derived unit test class chooses which implementation to use. The base class defines the common tests. |
| Assert.AreEqual | Parameters: expected then actual |  |
| Int to binary string | Convert.ToString(value, 2 ) | Useful for making unit test more readable. See [exercise 5.1](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI.Tests/Chapter5/Exercise1/WhenInsertingBits.cs) |
| Binary string to int | Convert.ToInt32(str, 2 )    | The second parameter is the base. See [exercise 5.1](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI.Tests/Chapter5/Exercise1/WhenInsertingBits.cs) |
| Validate arguments    | ArgumentException(message, paramName ) | The parameter name is the second parameter to the constructor. |
| Validate argument range | ArgumentOutOfRangeException(paramName, message) | But this time the parameter name is the first parameter to the constructor! |
| Overflow checking | checked { ... } | Arithmetic overflow is silent unless the statement/s (NOT expressions) are in a checked block |
| Initialize dictionary | ... = { {key; value}, ... } | |

## Other coding tips

1. Whenever adding to an int, consider whether the expression should be in a checked block "{}" to catch overflows.
2. "for (int i = 0; i <= n; i++)" - consider whether n could be int.MaxValue, which would cause overflow. If so, consider summing in reverse.

## General approach

1. When faced with a "monolithic method", consider applying separation of concerns, even if this causes some duplication or multiple passes through a (small) iteration.
2. In the above case, consider refactoring out smaller methods first, as this might reduce duplication.
