using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter1.Exercise3
{
    /// <summary>
    /// A base class for checking if two strings are permutations of one another.
    /// The base classes only checks that the strings are of equal length.
    /// All other checking is delegated to the CheckIfAPermutation template method.
    /// Some extra protected utility methods are also provided.
    /// </summary>
    public abstract class BasePermutationChecker
    {
        protected abstract bool CheckIfAPermutation(string a, string b);

        public bool IsAPermutation(string a, string b, 
            bool isCaseSensitive = true)
        {
            if (a.Length != b.Length)
            {
                return false;
            }
            if (!isCaseSensitive)
            {
                a = a.ToLowerInvariant();  // TODO: Consider passing culture info as an extra parameter
                b = b.ToLowerInvariant();
            }
            return CheckIfAPermutation(a, b);
        }

        protected bool AreAllLetterCountsZero(IDictionary<char, int> letterCounts)
        {
            return letterCounts.Values.All(value => value == 0);
        }

        protected void IncrementLetterCount(IDictionary<char, int> letterCounts, char letter)
        {
            AdjustLetterCount(letterCounts, letter, 1);
        }

        protected void DecrementLetterCount(IDictionary<char, int> letterCounts, char letter)
        {
            AdjustLetterCount(letterCounts, letter, -1);
        }

        protected void AdjustLetterCount(
            IDictionary<char, int> letterCounts, char letter, int adjustment)
        {
            int count = letterCounts.ContainsKey(letter) ? letterCounts[letter] : 0;
            count += adjustment;
            if (count == 0)
            {
                letterCounts.Remove(letter);
            }
            else
            {
                letterCounts[letter] = count;
            }
        }
    }
}
