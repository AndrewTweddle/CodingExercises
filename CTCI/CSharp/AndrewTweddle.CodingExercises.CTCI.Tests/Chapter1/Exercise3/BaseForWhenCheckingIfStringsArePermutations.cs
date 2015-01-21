using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter1.Exercise3;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter1.Exercise3
{
    public abstract class BaseForWhenCheckingIfStringsArePermutations
    {
        // Abstract factory method:
        /// <summary>
        /// An abstract factory method to create the relevant permutation checker class.
        /// </summary>
        /// <returns>A derived permutation checker instance</returns>
        protected abstract BasePermutationChecker CreatePermutationChecker();

        private BasePermutationChecker permutationChecker;

        protected BasePermutationChecker PermutationChecker
        {
            get
            {
                if (permutationChecker == null)
                {
                    permutationChecker = CreatePermutationChecker();
                }
                return permutationChecker;
            }
        }

        [TestMethod]
        public void ThenTwoEmptyStringsArePermutationsOfOneAnother()
        {
            bool isAPermutation = PermutationChecker.IsAPermutation(String.Empty, "");
            Assert.IsTrue(isAPermutation);
        }

        [TestMethod]
        public void ThenASingleCharacterIsAPermutationOfItself()
        {
            Assert.IsTrue(PermutationChecker.IsAPermutation("a", "a"));
        }

        [TestMethod]
        public void ThenASingleCharacterWithDifferentCapitalizationsIsByDefaultNotAPermutationOfItself()
        {
            Assert.IsFalse(PermutationChecker.IsAPermutation("a", "A"));
        }

        [TestMethod]
        public void ThenASingleCharacterWithDifferentCapitalizationsIsAPermutationOfItselfIfCaseInsensitive()
        {
            Assert.IsTrue(PermutationChecker.IsAPermutation("a", "A", isCaseSensitive: false));
        }

        [TestMethod]
        public void ThenAStringIsAPermutationOfItself()
        {
            string str = "abc";
            Assert.IsTrue(PermutationChecker.IsAPermutation(str, str));
        }

        [TestMethod]
        public void ThenDifferentLengthStringsAreNotPermutationsOfOneAnother()
        {
            Assert.IsFalse(PermutationChecker.IsAPermutation("abc", "abcd"));
        }

        [TestMethod]
        public void ThenStringsWithDifferentQuantitiesOfTheSameLettersAreNotPermutationsOfOneAnother()
        {
            Assert.IsFalse(PermutationChecker.IsAPermutation("aabc", "abcc"));
        }

        [TestMethod]
        public void ThenTheReverseOfAStringIsAPermutationOfIt()
        {
            string str = "abbccc";
            string rev = "cccbba";
            Assert.IsTrue(PermutationChecker.IsAPermutation(str, rev));
        }

        [TestMethod]
        public void ThenItIsNotAPermutationIfThereAreExtraLettersAtTheEndOfTheSecondString()
        {
            string a = "a";
            string b = "abcd";
            Assert.IsFalse(PermutationChecker.IsAPermutation(a, b));
        }

        [TestMethod]
        public void ThenItIsNotAPermutationIfThereAreExtraLettersAtTheBeginningOfTheSecondString()
        {
            string a = "abc";
            string b = "dabc";
            Assert.IsFalse(PermutationChecker.IsAPermutation(a, b));
        }

        [TestMethod]
        public void ThenItIsNotAPermutationIfThereAreExtraLettersAtTheEndOfTheFirstString()
        {
            string a = "abcd";
            string b = "a";
            Assert.IsFalse(PermutationChecker.IsAPermutation(a, b));
        }

        [TestMethod]
        public void ThenItIsNotAPermutationIfThereAreExtraLettersAtTheBeginningOfTheFirstString()
        {
            string a = "dabc";
            string b = "abc";
            Assert.IsFalse(PermutationChecker.IsAPermutation(a, b));
        }

        [TestMethod]
        public void ThenItIsNotAPermutationIfOnlyOneStringIsNonEmpty()
        {
            Assert.IsFalse(PermutationChecker.IsAPermutation(String.Empty, "a"));
        }
    }
}
