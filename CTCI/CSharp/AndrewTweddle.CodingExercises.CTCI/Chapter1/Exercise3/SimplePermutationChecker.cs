using System;
using System.Collections.Generic;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter1.Exercise3
{
    public class SimplePermutationChecker: BasePermutationChecker
    {
        protected override bool CheckIfAPermutation(string a, string b)
        {
            IDictionary<char, int> letterCounts = new Dictionary<char, int>();
            foreach (char chA in a) IncrementLetterCount(letterCounts, chA);
            foreach (char chB in b) DecrementLetterCount(letterCounts, chB);
            return AreAllLetterCountsZero(letterCounts);
        }
    }
}
