using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter4;
using AndrewTweddle.CodingExercises.CTCI.Chapter4.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter4.Exercise1
{
    [TestClass]
    public class WhenDeterminingTheMinAndMaxDepthOfATree
    {
        [TestMethod]
        public void ThenAnEmptyTreeHasMinAndMaxDepthsOfZero()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            Tuple<int, int> minAndMaxDepths = BinaryTreeBalanceChecker.GetMinAndMaxDepth(binaryTree);
            Assert.AreEqual(minAndMaxDepths.Item1, 0, "The minimum depth should be zero");
            Assert.AreEqual(minAndMaxDepths.Item2, 0, "The maximum depth should be zero");
        }

        [TestMethod]
        public void ThenATreeWithOneNodeHasMinAndMaxDepthsOfOne()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            binaryTree.RootNode = new BinaryTreeNode<int>(1);
            Tuple<int, int> minAndMaxDepths = BinaryTreeBalanceChecker.GetMinAndMaxDepth(binaryTree);
            Assert.AreEqual(minAndMaxDepths.Item1, 1, "The minimum depth should be one");
            Assert.AreEqual(minAndMaxDepths.Item2, 1, "The maximum depth should be one");
        }

        [TestMethod]
        public void ThenATreeWithOneBranchEmptyHasAMinimumDepthOfOne()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            BinaryTreeNode<int> rootNode = new BinaryTreeNode<int>(1);
            binaryTree.RootNode = rootNode;
            rootNode.Left = new BinaryTreeNode<int>(2);

            Tuple<int, int> minAndMaxDepths = BinaryTreeBalanceChecker.GetMinAndMaxDepth(binaryTree);
            Assert.AreEqual(minAndMaxDepths.Item1, 1, "The minimum depth should be one");
        }

        [TestMethod]
        public void ThenATreeWithAShortLeftAndALongRightBranchHasCorrespondingDepths()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            BinaryTreeNode<int> rootNode = new BinaryTreeNode<int>(1);
            binaryTree.RootNode = rootNode;
            rootNode.Left = new BinaryTreeNode<int>(2);
            rootNode.Right = new BinaryTreeNode<int>(3);
            rootNode.Right.Right = new BinaryTreeNode<int>(4);
            rootNode.Right.Right.Right = new BinaryTreeNode<int>(5);

            Tuple<int, int> minAndMaxDepths = BinaryTreeBalanceChecker.GetMinAndMaxDepth(binaryTree);
            Assert.AreEqual(minAndMaxDepths.Item1, 2, "The minimum depth should be two");
            Assert.AreEqual(minAndMaxDepths.Item2, 4, "The maximum depth should be four");
        }

        [TestMethod]
        public void ThenATreeWithALongLeftAndAShortRightBranchHasCorrespondingDepths()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            BinaryTreeNode<int> rootNode = new BinaryTreeNode<int>(1);
            binaryTree.RootNode = rootNode;
            rootNode.Left = new BinaryTreeNode<int>(2);
            rootNode.Left.Left = new BinaryTreeNode<int>(3);
            rootNode.Left.Left.Left = new BinaryTreeNode<int>(4);
            rootNode.Right = new BinaryTreeNode<int>(5);

            Tuple<int, int> minAndMaxDepths = BinaryTreeBalanceChecker.GetMinAndMaxDepth(binaryTree);
            Assert.AreEqual(minAndMaxDepths.Item1, 2, "The minimum depth should be two");
            Assert.AreEqual(minAndMaxDepths.Item2, 4, "The maximum depth should be four");
        }
    }
}
