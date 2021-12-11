# Overview

This project contains my solutions to various coding exercises.

These include exercises from:
  * [Project Euler](https://projecteuler.net)
  * [Advent of Code](https://adventofcode.com/)
  * Katas from [CodingDojo.org](https://codingdojo.org/)
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

| Exercise                                                                                                                                       | Description                                    | My solutions                                                                                                         | Notes                                                                                                                    |
|------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------|----------------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------|
| [Bowling scorer](https://codingdojo.org/kata/Bowling/) - [readme](Katas/CodingDojo/Bowling/DotNet/src/main/csharp/AndrewTweddle.Katas.Bowling) | Ten pin bowling scoring algorithm              | [C# (v1)](Katas/CodingDojo/Bowling/DotNet/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer.cs)              |                                                                                                                          |
|                                                                                                                                                |                                                | [C# (v2 - stateless)](Katas/CodingDojo/Bowling/DotNet/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer2.cs) |                                                                                                                          |
|                                                                                                                                                |                                                | [C# (v3 - immutable)](Katas/CodingDojo/Bowling/DotNet/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer3.cs) |                                                                                                                          |
|                                                                                                                                                |                                                | [F#](Katas/CodingDojo/Bowling/DotNet/src/main/fsharp/AndrewTweddle.Katas.Bowling.FSharp/FunctionalBowlingScorer.fs)  |                                                                                                                          |
|                                                                                                                                                |                                                | [unit tests](Katas/CodingDojo/Bowling/DotNet/src/test/csharp/AndrewTweddle.Katas.Test.Bowling)                       |                                                                                                                          |
| [Roman Numerals](https://codingdojo.org/kata/RomanNumerals/)                                                                                   | Convert to and from Roman numerals up to 3000. | [README file](Katas/CodingDojo/RomanNumerals/README.md)                                                              | This discusses the various experiments below.                                                                            |
|                                                                                                                                                |                                                | [Rust (TDD) - May 2021](Katas/CodingDojo/roman_numerals/src/main.rs)                                                 | Experiment with TDD and with quickcheck property-based testing.                                                          |
|                                                                                                                                                |                                                | [Rust v2 - May 2021](Katas/CodingDojo/RomanNumerals/Rust/roman_numerals_v2/src/main.rs)                              | Designed intuitively, not iteratively to compare with the TDD approach. Experiment with proptest property-based testing. |
|                                                                                                                                                |                                                | [Rust v3 - May 2021](Katas/CodingDojo/RomanNumerals/Rust/roman_numerals_v3/src/main.rs)                              | A simpler approach (unfortunately not mine - I saw others doing this, and rewrote it in Rust).                           |

## Miscellaneous

| Exercise                                                                  |Description                                                     | My solutions                                                                                                       |
|---------------------------------------------------------------------------|-----------------------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|
| [wordrect](misc/WordRectangles) - [readme](misc/WordRectangles/readme.md) |Largest rectangle of letters with words in every row and column | [Scala prototype (v1)](misc/WordRectangles/Scala_v1/src/main/scala/com/andrewtweddle/wordrects/WordRectMain.scala) |
|                                                                           |                                                                 | [C++ prototype (v1)](misc/WordRectangles/Cpp_v1/main.cpp)                                                         | 

## Project Euler problems

Project Euler requests that solutions not be shared online. Instead one can share solutions on the forum for each problem AFTER solving it successfully.

| Problem # | Description                                                      | Forum page with my solution (if shared)             | Notes |
| ---       | ---                                                              | ---                                                 | ---   |
| 1         | [Multiples of 3 and 5](https://projecteuler.net/problem=1)       |                                                     |       |
| 2         | [Even Fibonacci Numbers](https://projecteuler.net/problem=2)    |                                                     |       |
| 3         | [Largest Prime Factor](https://projecteuler.net/problem=3)       |                                                     |       |
| 4         | [Largest Palindrome Product](https://projecteuler.net/problem=4) |                                                     |       |
| 5         | [Smallest Multiple](https://projecteuler.net/problem=5)          |                                                     |       |
| 6         | [Sum square difference](https://projecteuler.net/problem=6)      | [Sum of Cubes is the Square of the Sums](https://projecteuler.net/thread=6;page=6#380323) |   |
| 7-14      | ...                                                              |                                                     |       |
| 15        | [Lattice paths](https://projecteuler.net/problem=15)             | [Various Rust solutions](https://projecteuler.net/thread=15;page=8#381302) |   |
| 16        | [Power digit sum](https://projecteuler.net/problem=16): Sum of decimal digits in 2^1000 | [Various Rust solutions](https://projecteuler.net/thread=16;page=8#381420) | I'm proud of the very different approach I found (which also ran extremely quickly). It earned some kudos from other participants. |
| 17        | [Number Letter Counts](https://projecteuler.net/problem=17)      | [Two Rust solutions](https://projecteuler.net/thread=17;page=8#382439) | TDD solution pair programmed with a colleague, and a faster, shorter one done on my own  |
| 18 - 29   | ...                                                              |                                                     |       |

## Advent of Code

### 2020

|Day                                           |Problem |Date Solved |My solution |
|----------------------------------------------|--------|------------|------------|
| [Day 1](https://adventofcode.com/2020/day/1) |   1    | 2021-11-18 | [Rust](AdventOfCode/Aoc2020/src/bin/day1_problem1.rs)| 
|                                              |   2    | 2021-11-18 | [Rust](AdventOfCode/Aoc2020/src/bin/day1_problem2.rs)|
| [Day 2](https://adventofcode.com/2020/day/2) |   1    | 2021-11-20 | [Rust](AdventOfCode/Aoc2020/src/bin/day2_problem1.rs)|
|                                              |   2    | 2021-11-20 | [Rust](AdventOfCode/Aoc2020/src/bin/day2_problem2.rs)|
| [Day 3](https://adventofcode.com/2020/day/3) |   1    | 2021-11-27 | [Rust](AdventOfCode/Aoc2020/src/bin/day3_problem1.rs)|
|                                              |   2    | 2021-11-27 | [Rust](AdventOfCode/Aoc2020/src/bin/day3_problem2.rs)|

### 2021

| Day                                            |Problem | Date Solved | My solution                                            |
|------------------------------------------------|--------|-------------|--------------------------------------------------------|
| [Day 1](https://adventofcode.com/2021/day/1)   |   1    | 2021-12-01  | [Rust](AdventOfCode/aoc2021/src/bin/day1_problem1.rs)  | 
|                                                |   2    | 2021-12-01  | [Rust](AdventOfCode/aoc2021/src/bin/day1_problem2.rs)  |
| [Day 2](https://adventofcode.com/2021/day/2)   |   1    | 2021-12-02  | [Rust](AdventOfCode/aoc2021/src/bin/day2_problem1.rs)  | 
|                                                |   2    | 2021-12-02  | [Rust](AdventOfCode/aoc2021/src/bin/day2_problem2.rs)  |
| [Day 3](https://adventofcode.com/2021/day/3)   |   1    | 2021-12-04  | [Rust](AdventOfCode/aoc2021/src/bin/day3_problem1.rs)  | 
|                                                |   2    | 2021-12-04  | [Rust](AdventOfCode/aoc2021/src/bin/day3_problem2.rs)  |
| [Day 4](https://adventofcode.com/2021/day/4)   |   1    | 2021-12-04  | [Rust](AdventOfCode/aoc2021/src/bin/day4_problem1.rs)  | 
|                                                |   2    | 2021-12-04  | [Rust](AdventOfCode/aoc2021/src/bin/day4_problem2.rs)  |
| [Day 5](https://adventofcode.com/2021/day/5)   |   1    | 2021-12-05  | [Rust](AdventOfCode/aoc2021/src/bin/day5_problem1.rs)  | 
|                                                |   2    | 2021-12-05  | [Rust](AdventOfCode/aoc2021/src/bin/day5_problem2.rs)  |
| [Day 6](https://adventofcode.com/2021/day/6)   |   1    | 2021-12-06  | [Rust](AdventOfCode/aoc2021/src/bin/day6_problem1.rs)  | 
|                                                |   2    | 2021-12-06  | [Rust](AdventOfCode/aoc2021/src/bin/day6_problem2.rs)  |
| [Day 7](https://adventofcode.com/2021/day/7)   |   1    | 2021-12-07  | [Rust](AdventOfCode/aoc2021/src/bin/day7_problem1.rs)  | 
|                                                |   2    | 2021-12-07  | [Rust](AdventOfCode/aoc2021/src/bin/day7_problem2.rs)  |
| [Day 8](https://adventofcode.com/2021/day/8)   |   1    | 2021-12-08  | [Rust](AdventOfCode/aoc2021/src/bin/day8_problem1.rs)  | 
|                                                |   2    | 2021-12-08  | [Rust](AdventOfCode/aoc2021/src/bin/day8_problem2.rs)  |
| [Day 9](https://adventofcode.com/2021/day/9)   |   1    | 2021-12-09  | [Rust](AdventOfCode/aoc2021/src/bin/day9_problem1.rs)  | 
|                                                |   2    | 2021-12-09  | [Rust](AdventOfCode/aoc2021/src/bin/day9_problem2.rs)  |
| [Day 10](https://adventofcode.com/2021/day/10) |   1    | 2021-12-10  | [Rust](AdventOfCode/aoc2021/src/bin/day10_problem1.rs) |

## cryptopals crypto challenges

|Challenge Set                         | Challenge | Date Solved | Description             | Solution link                                 |
|--------------------------------------|-----------|-------------|-------------------------|-----------------------------------------------|
|[Set 1](https://cryptopals.com/sets/1)| #1        | 2021-11-20  | Convert hex to base64   | [Rust](cryptopals/set1/src/bin/challenge1.rs) |
|                                      | #2        | 2021-12-01  | Fixed XOR               | [Rust](cryptopals/set1/src/bin/challenge2.rs) |
|                                      | #3        | 2021-12-03  | Single-byte XOR cipher  | [Rust](cryptopals/set1/src/bin/challenge3.rs) |

## CTCI

The CTCI sub-folder contains my solutions to exercises from the 5th edition of [Cracking the Coding Interview](http://www.amazon.com/Cracking-Coding-Interview-Programming-Questions/dp/098478280X) by Gayle Laakmann McDowell.

| Exercise  | Description                              | My solutions  |
| ---       | ---                                      | ---           |
| 1.1       | Check for duplicate letters in a string  | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise1/DuplicateLetterChecker.cs)  |
| 1.3       | Check if two strings are permutations    | [C# (readable)](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise3/SimplePermutationChecker.cs) |
|           |                                          | [C# (fast)](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise3/QuickPermutationChecker.cs) |
| 2.1       | Remove duplicate letters from a linked list  | [C# (fast)](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter2/Exercise1/QuickDuplicateLetterRemover.cs) |
|           |                                              | [C# (without temporary buffer)](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter2/Exercise1/DuplicateLetterRemoverWithoutTemporaryBuffer.cs) |
| 3.1       | Implement multiple stacks in a single array  | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter3/Exercise1/StackArray.cs) |
| 3.1       | Repeat as a challenge, using array space more effectively  | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter3/Exercise1Challenge/Stacker.cs) |
| 4.1       | Determine if a binary tree is balanced       | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter4/Exercise1/BinaryTreeBalanceChecker.cs) |
| 5.1       | Overwrite bits in one int from another   | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter5/Exercise1/BitInserter.cs) |
| 9.1       | Number of different ways of hopping up stairs | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter9/Exercise1/StepsSolver.cs) |
| 1.4       | Replacing spaces with "%20" in a string       | [Scala](CTCI/Scala/src/main/scala/ctci/chapter1/Exercise4.scala) |
| 2.2       | kth to last node in a linked list        | [C#](CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter2/Exercise2/Node.cs) |

## Functional Programming In Scala

Exercises from the book
["Functional Programming in Scala"](http://www.manning.com/bjarnason/)
are in [a separate GitHub project](https://github.com/AndrewTweddle/fpinscala)
which was forked from [the original FPInScala repo](https://github.com/fpinscala/fpinscala)
for the book.

# Learnings and notes

## General approach

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
