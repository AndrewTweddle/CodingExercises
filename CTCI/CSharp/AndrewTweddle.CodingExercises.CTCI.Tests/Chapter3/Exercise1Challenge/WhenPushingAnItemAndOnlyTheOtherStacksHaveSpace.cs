using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1Challenge;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter3.Exercise1Challenge
{
    [TestClass]
    public class WhenPushingAnItemAndOnlyTheOtherStacksHaveSpace
    {
        Stacker<int> stacker;

        int PREV_STACK_INDEX = 0;
        int CURR_STACK_INDEX = 1;
        int NEXT_STACK_INDEX = 2;

        public Stack<int> PrevStack
        {
            get
            {
                return stacker[PREV_STACK_INDEX];
            }
        }

        public Stack<int> CurrStack
        {
            get
            {
                return stacker[CURR_STACK_INDEX];
            }
        }

        public Stack<int> NextStack
        {
            get
            {
                return stacker[NEXT_STACK_INDEX];
            }
        }

        [TestInitialize]
        public void CreateAFullStacker()
        {
            stacker = new Stacker<int>(9);

            // Add one item per stack:
            PrevStack.Push(0);
            NextStack.Push(1);
            CurrStack.Push(2);
        }

        [TestMethod]
        public void ThenTheNextStackIsMovedRightIfTheCurrentStackHasMoreItems()
        {
            // Add 2 more items to the current stack, 
            // so that it abuts the next stack and is bigger than it:
            CurrStack.Push(3);
            CurrStack.Push(4);

            // Ensure preconditions are satisfied:

            // Make sure there are gaps:
            Assert.AreNotEqual(NextStack.NextInsertPos, PrevStack.FirstInsertPos);
            Assert.AreNotEqual(PrevStack.NextInsertPos, CurrStack.FirstInsertPos);

            // Make sure there is no gap after the current stack:
            Assert.AreEqual(CurrStack.NextInsertPos, NextStack.FirstInsertPos);

            // Expect the current stack to grow, but not move:
            int expectedCurrStackFirstInsertPos = CurrStack.FirstInsertPos;
            int expectedCurrStackNextInsertPos
                = (CurrStack.NextInsertPos + 1) % stacker.BufferSize;
            int expectedPrevStackFirstInsertPos = PrevStack.FirstInsertPos;
            int expectedPrevStackNextInsertPos = PrevStack.NextInsertPos;

            // Expect the next stack to be shifted right 
            // (but we don't care how much by, so just expect it to be different):
            int priorNextStackFirstInsertPos = NextStack.FirstInsertPos;
            int priorNextStackNextInsertPos = NextStack.NextInsertPos;

            CurrStack.Push(5);

            // Check that the current stack was added to:
            Assert.AreEqual(expectedCurrStackFirstInsertPos, CurrStack.FirstInsertPos);
            Assert.AreEqual(expectedCurrStackNextInsertPos, CurrStack.NextInsertPos);

            // Check that the previous stack didn't move:
            Assert.AreEqual(expectedPrevStackFirstInsertPos, PrevStack.FirstInsertPos);
            Assert.AreEqual(expectedPrevStackNextInsertPos, PrevStack.NextInsertPos);

            // Check that the next stack has shifted:
            Assert.AreNotEqual(priorNextStackFirstInsertPos, NextStack.FirstInsertPos);
            Assert.AreNotEqual(priorNextStackNextInsertPos, NextStack.NextInsertPos);
        }

        [TestMethod]
        public void ThenTheCurrentStackIsMovedLeftIfTheNextStackHasMoreItems()
        {
            NextStack.Push(8);  // So that it has more items than CurrStack

            // Force the current stack to be pushed forward 2 spaces:
            PrevStack.Push(3);
            PrevStack.Push(4);
            PrevStack.Push(5);
            PrevStack.Push(6);
            PrevStack.Pop();
            // PrevStack has 3 items and a gap

            // Make sure there are gaps (so that preconditions are satisfied):
            Assert.AreNotEqual(NextStack.NextInsertPos, PrevStack.FirstInsertPos);
            Assert.AreNotEqual(PrevStack.NextInsertPos, CurrStack.FirstInsertPos);

            int expectedCurrStackFirstInsertPos 
                = (CurrStack.FirstInsertPos - 1 + stacker.BufferSize) % stacker.BufferSize;
            int expectedCurrStackNextInsertPos = CurrStack.NextInsertPos; 
                // i.e. shift left, but grow by 1, to cancel out
            int expectedNextStackFirstInsertPos = NextStack.FirstInsertPos;
            int expectedNextStackNextInsertPos = NextStack.NextInsertPos;
            int expectedPrevStackFirstInsertPos = PrevStack.FirstInsertPos;
            int expectedPrevStackNextInsertPos = PrevStack.NextInsertPos;

            // Now test the pushing:
            CurrStack.Push(5);

            // Check that the current stack shifted left and was added to:
            Assert.AreEqual(expectedCurrStackFirstInsertPos, CurrStack.FirstInsertPos);
            Assert.AreEqual(expectedCurrStackNextInsertPos, CurrStack.NextInsertPos);

            // Check that the previous and next stack didn't move:
            Assert.AreEqual(expectedPrevStackFirstInsertPos, PrevStack.FirstInsertPos);
            Assert.AreEqual(expectedPrevStackNextInsertPos, PrevStack.NextInsertPos);
            Assert.AreEqual(expectedNextStackFirstInsertPos, NextStack.FirstInsertPos);
            Assert.AreEqual(expectedNextStackNextInsertPos, NextStack.NextInsertPos);
        }

        [TestMethod]
        public void ThenTheCurrentStackIsMovedLeftIfTheNextStackHasTheSameNumberOfItems()
        {
            // Force the current stack to be pushed forward 2 spaces:
            PrevStack.Push(3);
            PrevStack.Push(4);
            PrevStack.Push(5);
            PrevStack.Push(6);
            PrevStack.Pop();
            // PrevStack has 3 items and a gap

            // Make sure there are gaps (so that preconditions are satisfied):
            Assert.AreEqual(NextStack.Count, CurrStack.Count);
            Assert.AreNotEqual(NextStack.NextInsertPos, PrevStack.FirstInsertPos);
            Assert.AreNotEqual(PrevStack.NextInsertPos, CurrStack.FirstInsertPos);

            int expectedCurrStackFirstInsertPos
                = (CurrStack.FirstInsertPos - 1 + stacker.BufferSize) % stacker.BufferSize;
            int expectedCurrStackNextInsertPos = CurrStack.NextInsertPos;
            // i.e. shift left, but grow by 1, to cancel out
            int expectedNextStackFirstInsertPos = NextStack.FirstInsertPos;
            int expectedNextStackNextInsertPos = NextStack.NextInsertPos;
            int expectedPrevStackFirstInsertPos = PrevStack.FirstInsertPos;
            int expectedPrevStackNextInsertPos = PrevStack.NextInsertPos;

            // Now test the pushing:
            CurrStack.Push(5);

            // Check that the current stack shifted left and was added to:
            Assert.AreEqual(expectedCurrStackFirstInsertPos, CurrStack.FirstInsertPos);
            Assert.AreEqual(expectedCurrStackNextInsertPos, CurrStack.NextInsertPos);

            // Check that the previous and next stack didn't move:
            Assert.AreEqual(expectedPrevStackFirstInsertPos, PrevStack.FirstInsertPos);
            Assert.AreEqual(expectedPrevStackNextInsertPos, PrevStack.NextInsertPos);
            Assert.AreEqual(expectedNextStackFirstInsertPos, NextStack.FirstInsertPos);
            Assert.AreEqual(expectedNextStackNextInsertPos, NextStack.NextInsertPos);
        }
    }
}
