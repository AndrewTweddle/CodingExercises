using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter1.Exercise1
{
    public abstract class BaseForWhenCheckingAStringForDuplicateLetters
    {
        /// <summary>
        /// Template method for checking for duplicates.
        /// Derived unit test classes must override this 
        /// to specify the implementation being tested.
        /// </summary>
        /// <param name="letters"></param>
        /// <returns></returns>
        protected abstract bool HasDuplicateLetters(string letters);

        [TestMethod]
        public void ThenAStringWithTheSameLetterConsecutivelyHasDuplicates()
        {
            Assert.IsTrue(HasDuplicateLetters("aa"));
            // Use consecutive letters to check for off-by-one errors (e.g. in nested for loops)
        }


        [TestMethod]
        public void ThenAnEmptyStringHasNoDuplicates()
        {
            Assert.IsFalse(HasDuplicateLetters(String.Empty));
        }

        [TestMethod]
        public void ThenASingleCharStringHasNoDuplicates()
        {
            Assert.IsFalse(HasDuplicateLetters("a"));
        }

        [TestMethod]
        public void ThenAStringWithDifferentCapitalisationsOfTheSameLetterHasNoDuplicates()
        {
            Assert.IsFalse(HasDuplicateLetters("Aa"));
        }

        [TestMethod]
        public void ThenAStringWithTheSameLetterAndCapitalisationHasDuplicates()
        {
            Assert.IsTrue(HasDuplicateLetters("aba"));
            // Put the letters at the edge of the string to test boundary behaviour
        }

        [TestMethod]
        public void ThenAStringWithAllDifferentLettersHasNoDuplicates()
        {
            Assert.IsFalse(HasDuplicateLetters("abcde"));
        }
    }
}
