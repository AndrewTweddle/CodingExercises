using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter5.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter5.Exercise1
{
    [TestClass]
    public class WhenInsertingBits
    {
        /// <summary>
        /// A utility method that takes as input (for m and n) strings of binary digits 
        /// rather than the corresponding integer values.
        /// It calculates the actual output and converts this to a binary string,
        /// then uses the unit testing assert statement to check that it matches
        /// the expected output, which is also expressed as a binary string.
        /// This improves readability of the unit tests and is also useful for debugging.
        /// </summary>
        /// <param name="mInBinary">The binary string representation of the number to be inserted</param>
        /// <param name="nInBinary">The binary string representation of the number,
        /// some of whose bits are to be replaced
        /// </param>
        /// <param name="i">The index of the starting (lowest) bit position to be replaced</param>
        /// <param name="j">The ending (highest) bit position </param>
        /// <param name="expectedOutputInBinary">The binary representation 
        /// of the expected result of performing the bit insertion
        /// </param>
        private void CheckInsertion(string mInBinary, string nInBinary, 
            short i, short j, string expectedOutputInBinary)
        {
            int m = Convert.ToInt32(mInBinary, 2);
            int n = Convert.ToInt32(nInBinary, 2);
            int actualOutput = BitInserter.InsertBits(m, n, i, j);
            string actualOutputInBinary = Convert.ToString(actualOutput, 2);
            Assert.AreEqual(expectedOutputInBinary, actualOutputInBinary);
        }

        [TestMethod, ExpectedException(typeof(ArgumentException))]
        public void ThenBitPositionIMustNotBeGreaterThanJ()
        {
            int output = BitInserter.InsertBits(1, 25, 2, 1);
        }

        [TestMethod, ExpectedException(typeof(ArgumentException))]
        public void ThenBitPositionIMustNotBeNegative()
        {
            int output = BitInserter.InsertBits(1, 25, -2, 1);
        }

        [TestMethod, ExpectedException(typeof(ArgumentException))]
        public void ThenBitPositionIMustNotBeGreaterThan31()
        {
            int output = BitInserter.InsertBits(1, 25, 32, 32);
        }

        [TestMethod, ExpectedException(typeof(ArgumentException))]
        public void ThenBitPositionJMustNotBeNegative()
        {
            int output = BitInserter.InsertBits(1, 25, 0, -1);
        }

        [TestMethod, ExpectedException(typeof(ArgumentException))]
        public void ThenBitPositionJMustNotBeGreaterThan31()
        {
            int output = BitInserter.InsertBits(1, 25, 0, 32);
        }

        [TestMethod]
        public void ThenJustTheRightMostBitCanBeSet()
        {
            CheckInsertion("1", "100", 0, 0, "101");
        }

        [TestMethod]
        public void ThenJustTheRightMostBitCanBeCleared()
        {
            CheckInsertion("0", "111", 0, 0, "110");
        }

        [TestMethod]
        public void ThenMultipleBitPositionsCanBeInserted()
        {
            CheckInsertion("1010", "11011011", 2, 5, "11101011");
        }

        [TestMethod]
        public void ThenBitPosition31CanBeSet()
        {
            string minIntAsBinary = Convert.ToString(int.MinValue, 2);
            CheckInsertion("1", "0", 31, 31, minIntAsBinary);
              // Note: In twos-complement, int.MinValue is represented by 
              // a 1 in the left-most position and all zeroes thereafter
        }

        [TestMethod]
        public void ThenAllBitPositionsCanBeInserted()
        {
            string mAsBinary = "10110011100011110000111110000010"; // 32 bits long
            CheckInsertion(mAsBinary, "101", 0, 31, mAsBinary);
        }
    }
}
