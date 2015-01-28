using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter4.Exercise1
{
    public static class BinaryTreeBalanceChecker
    {
        public static bool IsBinaryTreeBalanced<T>(BinaryTree<T> binaryTree)
        {
            Tuple<int, int> minAndMaxHeight = GetMinAndMaxDepth(binaryTree.RootNode);
            return minAndMaxHeight.Item2 - minAndMaxHeight.Item1 <= 1;
        }

        public static int GetDepth<T>(BinaryTree<T> binaryTree)
        {
            Tuple<int, int> minAndMaxDepth = GetMinAndMaxDepth(binaryTree);
            return minAndMaxDepth.Item2;
        }

        public static Tuple<int, int> GetMinAndMaxDepth<T>(BinaryTree<T> binaryTree)
        {
            return GetMinAndMaxDepth(binaryTree.RootNode);
        }

        public static Tuple<int, int> GetMinAndMaxDepth<T>(BinaryTreeNode<T> node)
        {
            if (node == null)
            {
                return new Tuple<int, int>(0, 0);
            }

            Tuple<int, int> leftHeights = GetMinAndMaxDepth(node.Left);
            Tuple<int, int> rightHeights = GetMinAndMaxDepth(node.Right);
            int minHeight = Math.Min(leftHeights.Item1, rightHeights.Item1) + 1;
            int maxHeight = Math.Max(leftHeights.Item2, rightHeights.Item2) + 1;
            return new Tuple<int, int>(minHeight, maxHeight);
        }
    }
}
