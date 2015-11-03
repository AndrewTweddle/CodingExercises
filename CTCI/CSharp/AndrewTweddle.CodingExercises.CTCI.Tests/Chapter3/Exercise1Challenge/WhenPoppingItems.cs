using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1Challenge;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter3.Exercise1Challenge
{
    [TestClass]
    public class WhenPoppingItems
    {
        private const int RING_BUFFER_SIZE = 9;

        private const int FIRST_ON_STACK_0 = 1;
        private const int NEXT_ON_STACK_0 = 2;
        private const int LAST_ON_STACK_0 = 3;
        private const int LAST_ON_STACK_1 = 4;
        private const int LAST_ON_STACK_2 = 5;

        Stacker<int> stacker;

        [TestInitialize]
        public void PushItemsOntoANewStacker()
        {
            stacker = new Stacker<int>(RING_BUFFER_SIZE);
            stacker[0].Push(FIRST_ON_STACK_0);
            stacker[0].Push(NEXT_ON_STACK_0);
            stacker[0].Push(LAST_ON_STACK_0);
            stacker[1].Push(LAST_ON_STACK_1);
            stacker[2].Push(LAST_ON_STACK_2);
        }

        [TestMethod]
        public void ThenTheItemPoppedOffEachStackIsTheSameAsTheLastItemPushed()
        {
            Assert.AreEqual(LAST_ON_STACK_0, stacker[0].Pop());
            Assert.AreEqual(LAST_ON_STACK_2, stacker[2].Pop());
            Assert.AreEqual(LAST_ON_STACK_1, stacker[1].Pop());
        }

        [TestMethod]
        public void ThenTheItemsArePoppedOffAStackInTheReverseOfTheOrderPushed()
        {
            Stack<int> stack = stacker[0];
            Assert.AreEqual(LAST_ON_STACK_0, stack.Pop());
            Assert.AreEqual(NEXT_ON_STACK_0, stack.Pop());
            Assert.AreEqual(FIRST_ON_STACK_0, stack.Pop());
            Assert.IsTrue(stack.IsEmpty);
        }

        [TestMethod, ExpectedException(typeof(InvalidOperationException))]
        public void ThenItIsNotPermittedToPopMoreItemsThanPushed()
        {
            Stack<int> stack = stacker[0];
            stack.Pop();
            stack.Pop();
            stack.Pop();
            Assert.IsTrue(stack.IsEmpty);
            stack.Pop();
        }
    }
}
