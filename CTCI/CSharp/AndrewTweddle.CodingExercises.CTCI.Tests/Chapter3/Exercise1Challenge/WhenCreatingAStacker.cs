using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1Challenge;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter3.Exercise1Challenge
{
    [TestClass]
    public class WhenCreatingAStacker
    {
        private const int RING_BUFFER_SIZE = 13;
        private const int EXPECTED_STACK_OFFSET = RING_BUFFER_SIZE / Stacker<int>.STACK_COUNT;

        Stacker<int> stacker;

        [TestInitialize]
        public void CreateANewStacker()
        {
            stacker = new Stacker<int>(RING_BUFFER_SIZE);
        }

        [TestMethod]
        public void ThenTheEmptyStacksAreSpacedEvenly()
        {
            for (int i = 0; i < Stacker<int>.STACK_COUNT; i++)
            {
                Stack<int> stack = stacker[i];
                Assert.AreEqual(i * EXPECTED_STACK_OFFSET, stack.FirstInsertPos);
            }
        }

        [TestMethod]
        public void ThenAnEmptyStackHasTheSameFirstAndNextInsertPositions()
        {
            stacker[0].Push(1);

            Assert.AreNotEqual(stacker[0].FirstInsertPos, stacker[0].NextInsertPos);
            Assert.AreEqual(stacker[1].FirstInsertPos, stacker[1].NextInsertPos);
        }
    }
}
