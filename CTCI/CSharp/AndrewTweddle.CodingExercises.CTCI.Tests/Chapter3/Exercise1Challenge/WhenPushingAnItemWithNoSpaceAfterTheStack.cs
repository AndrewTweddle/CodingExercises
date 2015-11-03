using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1Challenge;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter3.Exercise1Challenge
{
    [TestClass]
    public class WhenPushingAnItemWithNoSpaceAfterTheStack
    {
        Stacker<int> stacker;

        int PREV_STACK_INDEX = 0;
        int CURR_STACK_INDEX = 1;
        int NEXT_STACK_INDEX = 2;

        [TestInitialize]
        public void CreateAFullStacker()
        {
            stacker = new Stacker<int>(6);

            // Fill up the stacker:
            stacker[0].Push(0);
            stacker[0].Push(1);
            stacker[1].Push(2);
            stacker[1].Push(3);
            stacker[2].Push(4);
            stacker[2].Push(5);
        }

        [TestMethod, ExpectedException(typeof(InvalidOperationException))]
        public void ThenItIsAnErrorIfTheStackerIsFull()
        {
            // Try to push one more item:
            stacker[CURR_STACK_INDEX].Push(-1);
        }

        [TestMethod]
        public void ThenTheNextStackIsShiftedIfThePrevStackIsAdjacent()
        {
            // Make room on the next stack
            stacker.Pop(NEXT_STACK_INDEX);

            Stack<int> currStack = stacker[CURR_STACK_INDEX];
            Stack<int> nextStack = stacker[NEXT_STACK_INDEX];

            int expectedCurrStackFirstInsertPos = currStack.FirstInsertPos;
            int expectedCurrStackNextInsertPos 
                = (currStack.NextInsertPos + 1) % stacker.BufferSize;
            int expectedNextStackFirstInsertPos 
                = (nextStack.FirstInsertPos + 1) % stacker.BufferSize;
            int expectedNextStackNextInsertPos 
                = (nextStack.NextInsertPos + 1) % stacker.BufferSize;

            // Push one more item:
            stacker[CURR_STACK_INDEX].Push(-1);

            // Check that the current stack didn't shift, but was added to:
            Assert.AreEqual(expectedCurrStackFirstInsertPos, currStack.FirstInsertPos);
            Assert.AreEqual(expectedCurrStackNextInsertPos, currStack.NextInsertPos);

            // Check that the next stack shifted along:
            Assert.AreEqual(expectedNextStackFirstInsertPos, nextStack.FirstInsertPos);
            Assert.AreEqual(expectedNextStackNextInsertPos, nextStack.NextInsertPos);
        }

        [TestMethod]
        public void ThenAStackIsMovedLeftIfTheNextAndPrevStacksAreAdjacent()
        {
            // Ensure that only the previous stack has a gap after it:
            stacker.Pop(PREV_STACK_INDEX);

            Stack<int> prevStack = stacker[PREV_STACK_INDEX];
            Stack<int> currStack = stacker[CURR_STACK_INDEX];
            Stack<int> nextStack = stacker[NEXT_STACK_INDEX];

            int expectedPrevStackFirstInsertPos = prevStack.FirstInsertPos;
            int expectedPrevStackNextInsertPos = prevStack.NextInsertPos;
            int expectedCurrStackFirstInsertPos = (currStack.FirstInsertPos - 1) % stacker.BufferSize;
            int expectedCurrStackNextInsertPos = currStack.NextInsertPos;  // Move left cancels the push
            int expectedNextStackFirstInsertPos = nextStack.FirstInsertPos;
            int expectedNextStackNextInsertPos = nextStack.NextInsertPos;

            // Push one more item:
            stacker[CURR_STACK_INDEX].Push(-1);

            // Check that the current stack shifted left and was added to:
            Assert.AreEqual(expectedCurrStackFirstInsertPos, currStack.FirstInsertPos);
            Assert.AreEqual(expectedCurrStackNextInsertPos, currStack.NextInsertPos);

            // Check that the previous and next stack didn't move:
            Assert.AreEqual(expectedPrevStackFirstInsertPos, prevStack.FirstInsertPos);
            Assert.AreEqual(expectedPrevStackNextInsertPos, prevStack.NextInsertPos);
            Assert.AreEqual(expectedNextStackFirstInsertPos, nextStack.FirstInsertPos);
            Assert.AreEqual(expectedNextStackNextInsertPos, nextStack.NextInsertPos);
        }
    }
}
