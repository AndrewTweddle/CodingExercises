using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.CodingExercises.CTCI.Chapter2.Exercise2;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter2.Exercise2
{
    [TestClass]
    public class WhenFindingTheKToLastNode
    {
        [TestMethod, ExpectedException(typeof(ArgumentNullException))]
        public void ThenPassingANullRootNodeThrowsASensibleExceptionIfKIsNonZero()
        {
            Node<int> kLastNode = NodeHelper.FindKLastNode<int>(null, 1);
        }

        [TestMethod]
        public void ThenPassingANullRootNodeReturnsNullIfKIsZero()
        {
            Node<int> kLastNode = NodeHelper.FindKLastNode<int>(null, 0);
            Assert.IsNull(kLastNode, 
                "The zeroth to last node should always be null");
        }

        [TestMethod]
        public void ThenTheKthLastNodeWhenThereAreKNodesIsTheRootNode()
        {
            Node<int> rootNode = new Node<int>(1, 2, 3);
            Node<int> kLastNode = NodeHelper.FindKLastNode(rootNode, 3);
            Assert.AreEqual(rootNode, kLastNode);
        }

        [TestMethod]
        public void ThenTheKthLastNodeWhenThereAreKPlus1NodesIsTheSecondNode()
        {
            Node<int> rootNode = new Node<int>(1, 2, 3, 4);
            Node<int> kLastNode = NodeHelper.FindKLastNode(rootNode, 3);
            Assert.AreEqual(kLastNode.Value, 2);
        }

        [TestMethod]
        public void ThenTheFirstToLastNodeIsTheLastNode()
        {
            Node<int> rootNode = new Node<int>(1, 2, 3);
            Node<int> kLastNode = NodeHelper.FindKLastNode(rootNode, 1);
            Assert.IsNull(kLastNode.NextNode);
        }
    }
}
