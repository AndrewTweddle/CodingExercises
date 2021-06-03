# Roman Numerals Kata

## Overview

This kata can be found on [the CodingDojo web site](https://codingdojo.org/kata/RomanNumerals/).

The goal for the kata has two parts:
* Part 1 is to convert any integer between 1 and 3000 to its representation as a Roman numeral.
* Part 2 is to convert any Roman numeral in this range to the corresponding integer.

The actual problem statement is a bit more vague about the range, 
saying "There is no need to be able to convert numbers larger than about 3000."
As the kata progressed, I decided to make this an exact limit of 3000.

_(I was tempted to make it 3999, which would have been easier to implement, 
but decided that 3000 was more awkward and hence a bit more like a real-world specification.)_

I did this kata in May 2021.

## Approaches

### Algorithm 1: TDD approach

See [the source code](../roman_numerals/src/main.rs).

## Background

The first part of this kata was done using a test-driven development (TDD) approach.

This was in preparation for a pair programming session with a colleague who had recently presented the case for TDD.

The pair programming sessions were conducted over 2 lunchtimes,
as we collaboratively solved [problem 17](https://projecteuler.net/problem=17) 
on the [Project Euler](https://projecteuler.net) web site,
using TDD and ["ping-pong"](https://martinfowler.com/articles/on-pair-programming.html#PingPong).

#### Personal Goals

1. Evaluate TDD.
2. Become more familiar with Rust strings.
3. Bonus: See how well property-based testing complements traditional unit testing. 

#### The process

* Follow a red-green-refactor cycle to solve this problem.
* Commit after each step in the cycle (started after first cycle).

_[Note: This was to keep a history of the process for didactic purposes.
But, in the red cycle, I had to deliberately check in code that either wouldn't compile, or wouldn't pass the unit tests.
Of course one shouldn't do that when working on a project with other people.]_

Here's [the commit history](https://github.com/AndrewTweddle/CodingExercises/commits/master/Katas/CodingDojo/roman_numerals).

* In phase 1, don't follow a pre-conceived plan. See whether a solution emerges naturally from the process.

_(Note: I chose to do it this way as an experiment. I don't think TDD precludes spending time thinking about the problem up front.)_

#### The phase 1 experience

This went smoothly and I had a fully unit tested solution later that night.

#### The phase 2 experience

I returned to phase 2 a week later. 

I felt that regular expressions would be useful for extracting the thousands, hundreds, tens and units digits.
And that this would also make validation of the format easier.

I wasn't sure whether I could still follow the TDD process with a design already in mind.

I resisted the temptation to dive straight in. I followed the TDD process, even though I already had an end in mind.
It worked well again.

However, I didn't do a lot of unit testing of validations, because I trusted the regular expression...

```rust
fn test_converting_from_IIII() { assert!(convert_from_roman("IIII").is_err()); }
```

```rust
fn test_converting_from_VIIII_fails() { assert!(convert_from_roman("VIIII").is_err()); }
```

```rust
fn test_converting_from_empty_string() { assert!(convert_from_roman("").is_err()); }
```

```rust
fn test_converting_from_MMMI_fails() { assert!(convert_from_roman("MMMI").is_err()); }
```

```rust
fn test_is_roman_numeral_on_IXCM() { assert!(!is_roman("IXCM")); }
```

#### Assessment

Pros:
1. I had feared that TDD would be cumbersome and would make it hard to maintain a good flow. In practice, this didn't seem to be an issue.
2. A decent algorithm emerged in phase 1, despite little up-front design work.
3. In phase 2, I had an approach in mind. It was easier than I expected to work towards this while still following a TDD approach.

Cons:
1. I suspect it took longer to reach a solution due to the extra code written.

Some unaddressed concerns:
1. There are usually multiple ways of tackling a problem. Would more up front design or experimentation pay off?
2. TDD is an incremental approach. Is there a risk of getting stuck in a local optimum (in the search space of all possible programs)?
3. Unit tests act as an extra dependency on the code they test. Could this become a barrier to change later on? (In particular, when the change affects either the interface or the solution approach.)

#### Bonus work: property-based testing with quickcheck

##### Overview

Property-based testing is an approach that came out of the functional programming community.
It involves specifying the properties that your solution should possess and the input types (or "domain") for each property.
The testing framework then generates many sets of test data (typically randomly) and checks that all properties are satisfied.
To make troubleshooting more convenient, when a failing test case is found, the property-based testing framework will often try to find a smaller subset of the data that still fails the property checks. 

See this more detailed explanation on [jessitron's blog](https://jessitron.com/2013/04/25/property-based-testing-what-is-it/).

One of the first property-based testing libraries was [Haskell's quickcheck](https://en.wikipedia.org/wiki/QuickCheck).

There are many ports of quickcheck, including the following Rust quickcheck crate: 
* [The documentation](https://docs.rs/quickcheck/), and 
* [the github repository](https://github.com/BurntSushi/quickcheck)

Property-based testing covers many more scenarios. And it tests an abstract specification, rather than a concrete example.

However, there are also disadvantages to property-based testing:

* Difficulty: It can be harder to define the property specifications since they are more generic and abstract than normal unit tests.
* Non-determinism of tests: due to random generation of test data, a check that passes may fail in future, despite no changes to the code.
* Performance: Due to examining many sets of test data, the check can sometimes take longer than expected.

##### Adding Rust quickcheck property specifications

The property I wanted to check was that the convert_from_roman() and convert_to_roman() functions were inverses of one another.
For this to be true, I needed to limit the domain (i.e. input set) of each function to be the range (i.e. output set) of the other function.

This took a little while to get working efficiently, since:
* It was my first experience with the quickcheck library
* There are an extremely large number of input strings that could be generated for passing to convert_from_roman(). This wouldn't have been very useful. I added RomanChar and RomanString structs to help with limiting the strings to ones that had only the characters found in Roman numerals, plus an extra invalid character for testing purposes.   

Once the tests were working, I marked them with an "ignore" attribute, so that they will not be part of the test suite.
This was due to the non-determinism of the tests. They can still be run manually, one at a time.

### Algorithm 2: Intuitive over Incremental Approach

See [the source code](Rust/roman_numerals_v2/src/main.rs).

#### Goals

1. Solve the same problem using a more intuitive approach, to compare the experience and outcome with the TDD approach.

#### The algorithm

The main data structures I used to drive the algorithm were as follows:

```rust
const PATTERNS: [Pattern; 13] = [
    Pattern { pattern: "M",  value: 1000, max_repetitions: 3, steps_to_skip: 0 },
    Pattern { pattern: "CM", value:  900, max_repetitions: 1, steps_to_skip: 3 },
    Pattern { pattern: "D",  value:  500, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "CD", value:  400, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "C",  value:  100, max_repetitions: 3, steps_to_skip: 0 },
    Pattern { pattern: "XC", value:   90, max_repetitions: 1, steps_to_skip: 3 },
    Pattern { pattern: "L",  value:   50, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "XL", value:   40, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "X",  value:   10, max_repetitions: 3, steps_to_skip: 0 },
    Pattern { pattern: "IX", value:    9, max_repetitions: 1, steps_to_skip: 3 },
    Pattern { pattern: "V",  value:    5, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "IV", value:    4, max_repetitions: 1, steps_to_skip: 1 },
    Pattern { pattern: "I",  value:    1, max_repetitions: 3, steps_to_skip: 0 },
];
```

The patterns are matched in order. 

When converting from a Roman numeral string and a pattern is successfully matched to the next few characters, 
then the ```steps_to_skip``` cause the following few patterns to be ignored 
(otherwise invalid strings like "XCL" would get processed without an error).

The ```PATTERNS``` structure is effectively a compact way of specifying a finite state machine.
 
#### The process

I used the same API as for the previous problem.

I got the algorithm working very quickly (in about 45 minutes, including some time struggling to get the Rust syntax correct and make the compiler happy).

However, I realized that the algorithm would also produce values for some invalid Roman strings, such as "VIV", instead of flagging these as an error.

Instead of fixing this issue right away (by adding the ```steps_to_skip``` logic), 
I decided to copy-and-paste all the test code used for algorithm 1. 
I wanted to see whether the unit tests would pick up this issue or not.

A rewrite from scratch, but with the same public method signatures, is equivalent to doing a complete refactoring 
of the previous Roman Numerals implementation. So this "simulates" a rewrite of the previous algorithm.

The unit tests didn't pick up this issue. But one of the property-based tests did.

Rust has a nice feature, the "dbg!" macro, which one can place around any expression. 
This causes the line #, expression text and expression value to be logged to the console.

This macro can be added to one of the failing property tests...

```rust
        /// Check that the function that converts to a Roman numeral
        /// is the inverse of the function that converts from a Roman numeral
        #[quickcheck]
        #[ignore = "Property-based tests are not deterministic. Runs slowly."]
        fn check_convert_to_roman_is_left_inverse_of_convert_from_roman(
            roman_numeral: RomanString) -> TestResult
        {
            let roman = dbg!(roman_numeral.0);
            if !is_roman(roman.as_str()) {
                return TestResult::discard();
            }
            TestResult::from_bool(
                roman == convert_to_roman(
                    convert_from_roman(roman.as_str()).unwrap()
                ).unwrap()
            )
        }
```

A run might produce text similar to the following...

```text
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests (target/debug/deps/roman_numerals_v2-6407bb9a5cfeb186)
[src/main.rs:398] roman_numeral.0 = "MXCIXVCCCZCIZIXVXLCMDXCIVLIDCCZZZVLCCXIMXXIXXXDIXICMVLDLIIXZCMXMDZCZXDXZ"
[src/main.rs:398] roman_numeral.0 = "DIMZILIXIMVCVLVDDDVLCLIVMXICXCCCXV"
[src/main.rs:398] roman_numeral.0 = "MCCLDIZXLMIIDXDXMXMZCIDVMXDVXLMC"
[src/main.rs:398] roman_numeral.0 = "ZZZLDVZLLLXMLMZVMMXMDMXXLI"
...
[src/main.rs:398] roman_numeral.0 = "XDVLCXDZCMXLZXVXCMDDZLLVLDLMCVDCMV"
[src/main.rs:398] roman_numeral.0 = "CMD"
[src/main.rs:398] roman_numeral.0 = ""
[src/main.rs:398] roman_numeral.0 = "MD"
[src/main.rs:398] roman_numeral.0 = "CD"
[src/main.rs:398] roman_numeral.0 = "CM"
[src/main.rs:398] roman_numeral.0 = "\u{0}MD"
[src/main.rs:398] roman_numeral.0 = "\"MD"
[src/main.rs:398] roman_numeral.0 = "3MD"
...
[src/main.rs:398] roman_numeral.0 = "C\'C"
[src/main.rs:398] roman_numeral.0 = "C:C"
[src/main.rs:398] roman_numeral.0 = "CDC"
[src/main.rs:398] roman_numeral.0 = ""
[src/main.rs:398] roman_numeral.0 = "DC"
[src/main.rs:398] roman_numeral.0 = "CC"
[src/main.rs:398] roman_numeral.0 = "CD"
[src/main.rs:398] roman_numeral.0 = "\u{0}DC"
[src/main.rs:398] roman_numeral.0 = "\"DC"
[src/main.rs:398] roman_numeral.0 = "3DC"
[src/main.rs:398] roman_numeral.0 = ";DC"
[src/main.rs:398] roman_numeral.0 = "?DC"
[src/main.rs:398] roman_numeral.0 = "ADC"
[src/main.rs:398] roman_numeral.0 = "BDC"
[src/main.rs:398] roman_numeral.0 = "C\u{0}C"
[src/main.rs:398] roman_numeral.0 = "C\"C"
[src/main.rs:398] roman_numeral.0 = "C3C"
[src/main.rs:398] roman_numeral.0 = "C<C"
[src/main.rs:398] roman_numeral.0 = "C@C"
[src/main.rs:398] roman_numeral.0 = "CBC"
[src/main.rs:398] roman_numeral.0 = "CCC"
[src/main.rs:398] roman_numeral.0 = "CD\u{0}"
[src/main.rs:398] roman_numeral.0 = "CD\""
[src/main.rs:398] roman_numeral.0 = "CD3"
[src/main.rs:398] roman_numeral.0 = "CD;"
[src/main.rs:398] roman_numeral.0 = "CD?"
[src/main.rs:398] roman_numeral.0 = "CDA"
[src/main.rs:398] roman_numeral.0 = "CDB"

[quickcheck] TEST FAILED. Arguments: (RomanString("CDC"))
thread 'tests::property_based_tests::check_convert_to_roman_is_left_inverse_of_convert_from_roman' panicked at '[quickcheck] TEST FAILED. Arguments: (RomanString("CDC"))',...
```

This shows a small subset of the many strings considered.
But some of these contain suspect data (characters not configured in ```RomanChar```).

It also demonstrates how the quickcheck library searches for the shortest input that still fails.

Although the unit tests didn't pick up the validation issue, two of the tests did detect a bug.
I had forgotten to check that convert_from_roman() didn't product a result over 3000.

#### proptest crate for property-based testing

Since the quickcheck test data was a bit cumbersome to configure, 
and because the shrinking seems to produce invalid characters,
I decided to try the proptest crate to compare it to quickcheck.

After skimming the documentation, I tried adding proptest tests corresponding to the quickcheck tests.

This was much easier to do, and it was also possible to create more targeted test data...

```rust
            /// Check that the function that converts to a Roman numeral
            /// is the inverse of the function that converts from a Roman numeral
            #[test]
            fn test_convert_to_roman_is_left_inverse_of_convert_from_roman(
                roman in "(V?I{0,3}|IV|IX|L?X{0,3}|XL|XC|D?C{0,3}|CD|CM|M{0,3}|Z)+"
                    .prop_filter("ignore invalid Roman numerals", |r| is_roman(r.as_str())))
            {
                let roman_str = dbg!(roman.as_str());
                let reconverted_roman = convert_to_roman(
                    convert_from_roman(roman_str).unwrap()
                ).unwrap();
                assert_eq!(roman, reconverted_roman);
            }
```

With the dbg! macros to inspect the generated (and shrunk) data, we see output similar to the following...

```text
    Finished test [unoptimized + debuginfo] target(s) in 2.73s
     Running unittests (target/debug/deps/roman_numerals_v2-0c386f4935c60e27)
[src/main.rs:473] roman.as_str() = "ZZ"
[src/main.rs:473] roman.as_str() = "IXZIXIXIVXCZXDXCCZLXCMZ"
[src/main.rs:473] roman.as_str() = "XCXLMMMVIIXLXCZZCMLXZMMMIXCMCDLIVMZ"
...
[src/main.rs:473] roman.as_str() = "IXIVXLCZCDMMXCXCIIIMMXLCCMMIIICDCMCMXCZXCIVIVIX"
[src/main.rs:473] roman.as_str() = "MCDIVCDXCXCCMCMD"
[src/main.rs:473] roman.as_str() = "XLCMXLCMXXMMZ"
[src/main.rs:459] roman.as_str() = "CMCD"

Test failed: assertion failed: `(left == right)`
  left: `"CMC"`,
 right: `"M"`; minimal failing input: roman = "CMC"
	successes: 0
	local rejects: 30
		30 times at ignore invalid Roman numerals
	global rejects: 0

thread 'tests::proptest_property_based_tests::test_convert_to_roman_is_left_inverse_of_convert_from_roman' panicked at 'assertion failed: `(left == right)`
  left: `"CMCD"`,
 right: `"MCCC"`', src/main.rs:463:17
...
```

Another nice feature of proptest is that it saves the failing test case (and the random number seed that created it)
into a file ```proptest-regressions/main.txt```

#### Comparison of proptest and quickcheck

Pros of proptest vs quickcheck:
1. Much simpler, more flexible and more powerful control over test data generation e.g. regex strings are used to define the pattern of input strings. 
2. Issue of non-determinism mitigated by saving failing cases to a file, for retrying in the next test run.
3. Shrinking appears to work better (no strings with unexpected characters generated).

Cons of proptest vs quickcheck:
1. In the CLion IDE at least, there is no longer a "run test" arrow next to the test method. 
   So it is not as convenient to run an individual property-based test.
2. Use of various macros means the code no longer looks like pure Rust.
   It feels a little more like a Rust-flavoured DSL.
   So it's harder to conceptualize what it does.

#### Assessment

I thought that the intuitively-derived algorithm had shorter and cleaner code than the algorithm that emerged from the TDD process.

It was much faster to code, however no unit tests were written. 

In terms of code accuracy, there was:
1. a validation scenario deliberately left unimplemented, and
2. an actual bug in the code.

The unit tests copied from algorithm 1 only found one of these.
This was probably because I hadn't added very many unit tests to check the validation logic.
I had had a lot of confidence in the regular expression validation logic from the original solution.
But when copied to a new solution, the unit tests weren't adequate to test a very different implementation strategy.

The property-based checks often detect the other (but not always, since the test data is randomly generated).

A combination of unit testing and property-based testing seems to work better, but neither type of testing was sufficient on its own.

### Approach 3: Simple lookups

See [the source code](Rust/roman_numerals_v3/src/main.rs).

#### The process

I saw [a Ruby solution online](https://codingdojo.org/solution/KataRomanNumeralsAsReadableAsWeCouldMakeIt/) 
that used a separate array for each decimal digit, with the array contents being the Roman versions of each digit.

For comparison with both the Ruby version and my previous Rust versions, I wrote a Rust solution following a similar approach.
It was considerably more verbose than the Ruby solution.

#### The algorithm

The following lookup tables drive the algorithm:

```rust
const ONES     : [&str; 10] = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
const TENS     : [&str; 10] = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
const HUNDREDS : [&str; 10] = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
const THOUSANDS: [&str;  4] = ["", "M", "MM", "MMM"];
```

When converting a number to its Roman Numeral representation, this algorithm is similar to the first one I wrote,
except that it looks up the digits instead of calculating them.

#### The lesson

It's often simpler to use lookup tables instead of calculations.
