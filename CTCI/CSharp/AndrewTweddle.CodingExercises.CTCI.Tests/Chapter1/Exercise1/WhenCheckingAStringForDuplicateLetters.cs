using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter1.Exercise1
{
    [TestClass]
    public class WhenCheckingAStringForDuplicateLetters
    {
        [TestMethod]
        public void ThenAnEmptyStringHasNoDuplicates()
        {
            Assert.IsFalse(Exercise1_1.HasDuplicateLetters(String.Empty));
        }

        [TestMethod]
        public void ThenASingleCharStringHasNoDuplicates()
        {
            Assert.IsFalse(Exercise1_1.HasDuplicateLetters("a"));
        }

        [TestMethod]
        public void ThenAStringWithDifferentCapitalisationsOfTheSameLetterHasNoDuplicates()
        {
            Assert.IsFalse(Exercise1_1.HasDuplicateLetters("Aa"));
        }

        [TestMethod]
        public void ThenAStringWithTheSameLetterAndCapitalisationHasDuplicates()
        {
            Assert.IsTrue(Exercise1_1.HasDuplicateLetters("aba"));
             // Put the letters at the edge of the string to test boundary behaviour
        }

        [TestMethod]
        public void ThenAStringWithAllDifferentLettersHasNoDuplicates()
        {
            Assert.IsFalse(Exercise1_1.HasDuplicateLetters("abcde"));
        }
    }
}
