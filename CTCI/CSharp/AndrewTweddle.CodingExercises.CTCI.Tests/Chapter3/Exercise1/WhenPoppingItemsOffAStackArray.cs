using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter3.Exercise1
{
    [TestClass]
    public class WhenPoppingItemsOffAStackArray
    {
        [TestMethod, ExpectedException(typeof(InvalidOperationException))]
        public void ThenAnItemMayNotBePoppedOffAnEmptyStack()
        {
            IStackArray<char> stackArray = new StackArray<char>(3, 3);
            stackArray.Pop(1);
        }

        [TestMethod]
        public void ThenItemsMustBePoppedInTheReverseOrderThatTheyWerePushed()
        {
            IStackArray<int> stackArray = new StackArray<int>(10, 3);
            for (int stackNumber = 0; stackNumber < 10; stackNumber++)
            {
                for (int i = 0; i < 3; i++)
                {
                    stackArray.Push(stackNumber, i);
                }
                for (int i = 2; i >= 0; i--)
                {
                    int poppedValue = stackArray.Pop(stackNumber);
                    Assert.AreEqual(poppedValue, i);
                }
            }
        }

        [TestMethod]
        public void ThenTheStackCountsMustEqualTheRemainingItemCount()
        {
            IStackArray<int> stackArray = new StackArray<int>(10, 3);
            for (int stackNumber = 0; stackNumber < 10; stackNumber++)
            {
                for (int countBeforePushing = 0; countBeforePushing < 3; countBeforePushing++)
                {
                    stackArray.Push(stackNumber, countBeforePushing);
                    int count = stackArray.GetCount(stackNumber);
                    Assert.AreEqual(count, countBeforePushing + 1);
                }
                for (int i = 2; i >= 0; i--)
                {
                    int expectedCountAfterPopping = stackArray.Pop(stackNumber);
                    int count = stackArray.GetCount(stackNumber);
                    Assert.AreEqual(count, expectedCountAfterPopping);
                }
            }
        }
    }
}
