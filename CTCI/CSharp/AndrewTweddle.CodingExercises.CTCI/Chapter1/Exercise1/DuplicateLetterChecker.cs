using System;
using System.Collections.Generic;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter1.Exercise1
{
    public static class DuplicateLetterChecker
    {
        /// <summary>
        /// This method checks for duplicates letters 
        /// by using a HashSet to track letters previously found
        /// </summary>
        /// <param name="letters">The string of letters to check for duplicate letters</param>
        /// <returns>A boolean value indicating whether any duplicate letters were found</returns>
        public static bool HasDuplicateLetters(string letters)
        {
            HashSet<char> prevChars = new HashSet<char>();
            foreach (char ch in letters)
            {
                if (prevChars.Contains(ch)) return true;
                prevChars.Add(ch);
            }
            return false;
        }

        /// <summary>
        /// This method does not use any intermediate storage 
        /// to determine whether the string has duplicate letters.
        /// </summary>
        /// <param name="letters">The string of letters to check for duplicate letters</param>
        /// <returns>A boolean value indicating whether any duplicate letters were found</returns>
        public static bool HasDuplicateLettersWithoutAdditionalDataStructures(string letters)
        {
            int length = letters.Length;
            for (int i = 0; i < length - 1; i++)
                for (int j = i + 1; j < length; j++)
                    if (letters[j] == letters[i])
                        return true;
            return false;
        }
    }
}
