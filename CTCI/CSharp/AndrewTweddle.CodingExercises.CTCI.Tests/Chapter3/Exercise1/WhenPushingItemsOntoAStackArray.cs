using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter3.Exercise1
{
    [TestClass]
    public class WhenPushingItemsOntoAStackArray
    {
        [TestMethod]
        public void ThenTheMaximumStackSizeMayBeReached()
        {
            IStackArray<char> stackArray = new StackArray<char>(3, 3);
            stackArray.Push(1, 'a');
            stackArray.Push(1, 'b');
            stackArray.Push(1, 'c');
        }

        [TestMethod, ExpectedException(typeof(IndexOutOfRangeException))]
        public void ThenTheMaximumStackSizeMustNotBeExceeded()
        {
            IStackArray<char> stackArray = new StackArray<char>(3, 3);
            stackArray.Push(1, 'a');
            stackArray.Push(1, 'b');
            stackArray.Push(1, 'c');
            stackArray.Push(1, 'd');
        }

        [TestMethod]
        public void ThenTheCountMustEqualTheNumberOfItemsPushed()
        {
            IStackArray<char> stackArray = new StackArray<char>(3, 3);
            Assert.AreEqual(stackArray.GetCount(1), 0);
            stackArray.Push(1, 'a');
            Assert.AreEqual(stackArray.GetCount(1), 1);
            stackArray.Push(1, 'b');
            Assert.AreEqual(stackArray.GetCount(1), 2);
        }
    }
}
