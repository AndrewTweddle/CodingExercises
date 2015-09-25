using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.CodingExercises.CTCI.Chapter2.Exercise2;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter2.Exercise2
{
    [TestClass]
    public class WhenFindingTheKToLastNode
    {
        [TestMethod, ExpectedException(typeof(ArgumentNullException))]
        public void ThenPassingANullRootNodeThrowsASensibleException()
        {
            Node<int> kLastNode;
            NodeHelper.TryFindKLastNode(null, 1, out kLastNode);
        }
    }
}
