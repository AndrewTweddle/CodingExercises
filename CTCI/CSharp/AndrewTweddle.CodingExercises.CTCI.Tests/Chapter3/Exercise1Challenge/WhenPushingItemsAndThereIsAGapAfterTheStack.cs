using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1Challenge;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter3.Exercise1Challenge
{
    [TestClass]
    public class WhenPushingItemsAndThereIsAGapAfterTheStack
    {
        private const int RING_BUFFER_SIZE = 9;

        private const int FIRST_ON_STACK_0 = 1;
        private const int NEXT_ON_STACK_0 = 2;
        private const int LAST_ON_STACK_0 = 3;
        private const int FIRST_ON_STACK_1 = 4;
        private const int LAST_ON_STACK_1 = 5;
        private const int FIRST_ON_STACK_2 = 6;

        Stacker<int> stacker;

        [TestInitialize]
        public void PushItemsOntoANewStacker()
        {
            stacker = new Stacker<int>(RING_BUFFER_SIZE);
            stacker[0].Push(FIRST_ON_STACK_0);
            stacker[0].Push(NEXT_ON_STACK_0);
            stacker[0].Push(LAST_ON_STACK_0);
            stacker[1].Push(FIRST_ON_STACK_1);
            stacker[2].Push(FIRST_ON_STACK_2);
        }

        [TestMethod]
        public void ThenNoStacksAreMoved()
        {
            stacker = new Stacker<int>(RING_BUFFER_SIZE);
            int[] expectedFirstInsertPositions = new int[Stacker<int>.STACK_COUNT];
            int[] expectedNextInsertPositions = new int[Stacker<int>.STACK_COUNT];
            for (int i = 0; i < Stacker<int>.STACK_COUNT; i++)
            {
                Stack<int> stack = stacker[i];
                expectedFirstInsertPositions[i] = stack.FirstInsertPos;
                expectedNextInsertPositions[i] = stack.NextInsertPos;
            }
            stacker[1].Push(LAST_ON_STACK_1);
            expectedNextInsertPositions[1] = expectedNextInsertPositions[1] + 1;
            for (int i = 0; i < Stacker<int>.STACK_COUNT; i++)
            {
                Stack<int> stack = stacker[i];
                Assert.AreEqual(expectedFirstInsertPositions[i], stack.FirstInsertPos);
                Assert.AreEqual(expectedNextInsertPositions[i], stack.NextInsertPos);
            }
        }
    }
}
