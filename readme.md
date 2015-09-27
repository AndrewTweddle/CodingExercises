# Coding Exercises

## Overview

This project contains my solutions to various coding exercises.

These have been used to practise coding skills, learn new programming languages, compare languages and prepare for technical interviews.

# Different types of exercises

## CTCI

The CTCI sub-folder contain my solutions to exercises from the 5th edition of [Cracking the Coding Interview](http://www.amazon.com/Cracking-Coding-Interview-Programming-Questions/dp/098478280X) by Gayle Laakmann McDowell.

| Exercise  | Description                              | My solutions  | 
| ---       | ---                                      | ---           | 
| 1.1       | Check for duplicate letters in a string  | [C#](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise1/DuplicateLetterChecker.cs)  |
| 1.3       | Check if two strings are permutations    | [C# (readable)](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise3/SimplePermutationChecker.cs) [C# (fast)](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter1/Exercise3/QuickPermutationChecker.cs) |
| 2.1       | Remove duplicate letters from a linked list  | [C# (fast)](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter2/Exercise1/QuickDuplicateLetterRemover.cs) [C# (without temporary buffer)](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter2/Exercise1/DuplicateLetterRemoverWithoutTemporaryBuffer.cs) |
| 3.1       | Implement multiple stacks in a single array  | [C#](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter3/Exercise1/StackArray.cs) |
| 4.1       | Determine if a binary tree is balanced       | [C#](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter4/Exercise1/BinaryTreeBalanceChecker.cs) |
| 5.1       | Overwrite bits in one int from another   | [C#](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter5/Exercise1/BitInserter.cs) |
| 9.1       | Number of different ways of hopping up stairs | [C#](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI/Chapter9/Exercise1/StepsSolver.cs) |
| 1.4       | Replacing spaces with "%20" in a string       | [Scala](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/Scala/src/main/scala/ctci/chapter1/Exercise4.scala) |


## Functional Programming In Scala

Exercises from the book 
["Functional Programming in Scala"](http://www.manning.com/bjarnason/)
are in [a separate GitHub project](https://github.com/AndrewTweddle/fpinscala)
which was forked from [the original FPInScala repo](https://github.com/fpinscala/fpinscala)
for the book.

## Various Katas found online

### From the CodingDojo.org web site

| Exercise  | Description  | My solutions   | 
| ---       | ---          | ---            | 
| Bowling scorer - [readme](https://github.com/AndrewTweddle/CodingExercises/tree/master/Katas/src/main/csharp/AndrewTweddle.Katas.Bowling) | Ten pin bowling scoring algorithm        | [C# (v1)](https://github.com/AndrewTweddle/CodingExercises/blob/master/Katas/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer.cs) [C# (v2)](https://github.com/AndrewTweddle/CodingExercises/blob/master/Katas/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer2.cs) [F#](https://github.com/AndrewTweddle/CodingExercises/blob/master/Katas/src/main/fsharp/AndrewTweddle.Katas.Bowling.FSharp/FunctionalBowlingScorer.fs) [unit tests](https://github.com/AndrewTweddle/CodingExercises/tree/master/Katas/src/test/csharp/AndrewTweddle.Katas.Test.Bowling) |

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
| Int to binary string | Convert.ToString(value, 2 ) | Useful for making unit test more readable. See [exercise 5.1](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI.Tests/Chapter5/Exercise1/WhenInsertingBits.cs) |
| Binary string to int | Convert.ToInt32(str, 2 )    | The second parameter is the base. See [exercise 5.1](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI.Tests/Chapter5/Exercise1/WhenInsertingBits.cs) |
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
