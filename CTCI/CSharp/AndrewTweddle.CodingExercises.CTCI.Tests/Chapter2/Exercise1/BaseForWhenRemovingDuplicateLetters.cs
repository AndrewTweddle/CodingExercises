using System;
using System.Collections.Generic;
using System.Text;
using AndrewTweddle.CodingExercises.CTCI.Chapter2.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter2.Exercise1
{
    public abstract class BaseForWhenRemovingDuplicateLetters
    {
        /// <summary>
        /// Abstract factory method for creating an IDuplicateLetterRemover instance
        /// </summary>
        /// <returns>An IDuplicateLetterRemover instance</returns>
        protected abstract IDuplicateLetterRemover CreateDuplicateLetterRemover();

        private IDuplicateLetterRemover duplicateLetterRemover;

        public IDuplicateLetterRemover DuplicateLetterRemover
        {
            get
            {
                if (duplicateLetterRemover == null)
                {
                    duplicateLetterRemover = CreateDuplicateLetterRemover();
                }
                return duplicateLetterRemover;
            }
        }

        protected string ConvertToString(LinkedList<char> letters)
        {
            StringBuilder sb = new StringBuilder();
            foreach (char letter in letters)
            {
                sb.Append(letter);
            }
            return sb.ToString();
        }

        protected LinkedList<char> ConvertToLinkedList(string s)
        {
            LinkedList<char> linkedList = new LinkedList<char>();
            foreach (char ch in s)
            {
                linkedList.AddLast(ch);
            }
            return linkedList;
        }

        private void CheckDeduplication(string input, string expectedOutput)
        {
            // Arrange:
            LinkedList<char> linkedList = ConvertToLinkedList(input);

            // Act:
            DuplicateLetterRemover.RemoveDuplicateLetters(linkedList);

            // Assert:
            Assert.AreEqual(ConvertToString(linkedList), expectedOutput);
        }

        [TestMethod]
        public void ThenAnEmptyListIsUnchanged()
        {
            LinkedList<char> linkedList = new LinkedList<char>();
            DuplicateLetterRemover.RemoveDuplicateLetters(linkedList);
            Assert.AreEqual(ConvertToString(linkedList), String.Empty);
        }

        [TestMethod]
        public void ThenTheSameLetterWithDifferentCapitalisationIsNotRemoved()
        {
            CheckDeduplication("aA", "aA");
        }

        [TestMethod]
        public void ThenTheSameLetterTwiceIsDeDuplicated()
        {
            CheckDeduplication("aa", "a");
        }

        [TestMethod]
        public void ThenTheSameLetterInDifferentPartsOfLinkedListIsDeDuplicated()
        {
            CheckDeduplication("abaca", "abc");
        }

        [TestMethod]
        public void ThenMultipleSetsOfDuplicateLettersAreDeDuplicated()
        {
            CheckDeduplication("abccbba", "abc");
        }

        [TestMethod]
        public void ThenNoDeduplicationOccursIfAllLettersAreDifferent()
        {
            CheckDeduplication("abcde", "abcde");
        }
    }
}
