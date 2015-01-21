using System;
using System.Collections.Generic;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter1.Exercise3
{
    public class QuickPermutationChecker: BasePermutationChecker
    {
        protected override bool CheckIfAPermutation(string a, string b)
        {
            IDictionary<char, int> partialLetterCounts = new Dictionary<char, int>();
            int nextBIndex = 0;
            int bLength = b.Length;
            foreach (char chA in a)
            {
                // Check if this letter was previously found in string b:
                if (partialLetterCounts.ContainsKey(chA))
                {
                    DecrementLetterCount(partialLetterCounts, chA);
                    continue;
                }
                
                // Look for this character in the remainder of string b,
                // but keep track of other letters found while searching for it:
                bool isFound = false;

                for (int bIndex = nextBIndex; bIndex < bLength; bIndex++)
                {
                    char chB = b[bIndex];
                    if (chA == chB)
                    {
                        nextBIndex = bIndex + 1;
                        isFound = true;
                        break;
                    }
                    
                    // Keep track of the letter in partialLetterCounts:
                    IncrementLetterCount(partialLetterCounts, chB);
                }

                // If chA could not be found, then the strings are not permutations:
                if (!isFound)
                {
                    return false;
                }
            }

            // The following are extra safety checks. 
            // Strictly speaking, they shouldn't be necessary, 
            // as the string lengths would be different, 
            // and the base class checks for that.

            // Check if there are still unconsumed characters in string b:
            if (nextBIndex < bLength)
            {
                return false;
            }

            // Check that there aren't still unmatched, previously tracked letters:
            return AreAllLetterCountsZero(partialLetterCounts);
        }
    }
}
