using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter3.Exercise1
{
    [TestClass]
    public class WhenCreatingAStackArray
    {
        [TestMethod, ExpectedException(typeof(ArgumentOutOfRangeException))]
        public void ThenTheStackCountMustNotBeZero()
        {
            IStackArray<char> stackArray = new StackArray<char>(0, 100);
        }

        [TestMethod, ExpectedException(typeof(ArgumentOutOfRangeException))]
        public void ThenTheStackCountMustNotBeNegative()
        {
            IStackArray<char> stackArray = new StackArray<char>(-1, 100);
        }

        [TestMethod, ExpectedException(typeof(ArgumentOutOfRangeException))]
        public void ThenTheMaximumStackSizeMustNotBeNegative()
        {
            IStackArray<char> stackArray = new StackArray<char>(10, -1);
        }

        [TestMethod]
        public void ThenTheMaximumStackSizeCanBeZero()
        {
            IStackArray<char> stackArray = new StackArray<char>(10, 0);
        }
    }
}
