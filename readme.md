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

# Learnings and notes

## General approach

* Analysis
  * Question the requirements
  * Look for simplifying assumptions
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


## C\# 

| Purpose   | Solution              | Notes                      |
| ---       | ---                   | ---                        | 
| Sets      | HashSet&lt;T&gt;      | Useful methods: Add and Contains. |
| Hash maps | Dictionary&lt;K, V&gt;   | Useful methods: ContainsKey, Keys, Values, enumerator over KeyValuePair&lt;K, V&gt; |
| Test multiple implementations | MSTest: [TestMethod] on base class methods, [TestClass] on derived class | The derived unit test class chooses which implementation to use. The base class defines the common tests. |
| Assert.AreEqual | Parameters: expected then actual |  |
| Int to binary string | Convert.ToString(value, 2 ) | Useful for making unit test more readable. See [exercise 5.1](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI.Tests/Chapter5/Exercise1/WhenInsertingBits.cs) |
| Binary string to int | Convert.ToInt32(str, 2 )    | The second parameter is the base. See [exercise 5.1](https://github.com/AndrewTweddle/CodingExercises/blob/master/CTCI/CSharp/AndrewTweddle.CodingExercises.CTCI.Tests/Chapter5/Exercise1/WhenInsertingBits.cs) |
| ArgumentException    | ArgumentException(message, paramName ) | The parameter name is the second parameter to the constructor. |
