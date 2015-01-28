using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter4;
using AndrewTweddle.CodingExercises.CTCI.Chapter4.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter4.Exercise1
{
    [TestClass]
    public class WhenCheckingIfABinaryTreeIsBalanced
    {
        [TestMethod]
        public void ThenAnEmptyTreeIsBalanced()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            Assert.IsTrue(BinaryTreeBalanceChecker.IsBinaryTreeBalanced(binaryTree));
        }

        public void ThenABinaryTreeWithASingleNodeIsBalanced()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            binaryTree.RootNode = new BinaryTreeNode<int>(1);
            Assert.IsTrue(BinaryTreeBalanceChecker.IsBinaryTreeBalanced(binaryTree));
        }

        public void ThenABinaryTreeWithASingleNodeWithASingleChildNodeIsBalanced()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            binaryTree.RootNode = new BinaryTreeNode<int>(1);
            binaryTree.RootNode.Left = new BinaryTreeNode<int>(2);
            Assert.IsTrue(BinaryTreeBalanceChecker.IsBinaryTreeBalanced(binaryTree));
        }

        public void ThenABinaryTreeWithASingleNodeWithTwoChildNodesIsBalanced()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            binaryTree.RootNode = new BinaryTreeNode<int>(1);
            binaryTree.RootNode.Left = new BinaryTreeNode<int>(2);
            binaryTree.RootNode.Right = new BinaryTreeNode<int>(3);
            Assert.IsTrue(BinaryTreeBalanceChecker.IsBinaryTreeBalanced(binaryTree));
        }

        public void ThenABinaryTreeCanBeUnbalancedEvenThoughBothItsBranchesAreBalanced()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            binaryTree.RootNode = new BinaryTreeNode<int>(1);
            binaryTree.RootNode.Left = new BinaryTreeNode<int>(2);
            binaryTree.RootNode.Right = new BinaryTreeNode<int>(3);
            binaryTree.RootNode.Right.Left = new BinaryTreeNode<int>(4);
            binaryTree.RootNode.Right.Right = new BinaryTreeNode<int>(3);
            Assert.IsFalse(BinaryTreeBalanceChecker.IsBinaryTreeBalanced(binaryTree));
        }

        public void ThenABinaryTreeIsBalancedIfItsBranchesHaveDepthsDifferingByOne()
        {
            BinaryTree<int> binaryTree = new BinaryTree<int>();
            binaryTree.RootNode = new BinaryTreeNode<int>(1);
            binaryTree.RootNode.Left = new BinaryTreeNode<int>(2);
            binaryTree.RootNode.Left.Right = new BinaryTreeNode<int>(3);
            binaryTree.RootNode.Right = new BinaryTreeNode<int>(4);
            binaryTree.RootNode.Right.Left = new BinaryTreeNode<int>(5);
            binaryTree.RootNode.Right.Right = new BinaryTreeNode<int>(6);
            binaryTree.RootNode.Right.Right.Left = new BinaryTreeNode<int>(7);
            binaryTree.RootNode.Right.Right.Right = new BinaryTreeNode<int>(8);
            Assert.IsFalse(BinaryTreeBalanceChecker.IsBinaryTreeBalanced(binaryTree));
        }

    }
}
