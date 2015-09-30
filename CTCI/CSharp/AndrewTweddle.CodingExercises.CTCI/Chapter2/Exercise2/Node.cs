using System;
using System.Collections.Generic;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter2.Exercise2
{
    /// <summary>
    /// A crude implementation of a single linked list using a Node class.
    /// The Node and its Value are both immutable.
    /// </summary>
    public class Node<T>
    {
        public T Value { get; private set; }
        public Node<T> NextNode { get; private set; }
        public Node(T value, Node<T> nextNode)
        {
            Value = value;
            NextNode = nextNode;
        }

        public Node(params T[] values)
        {
            if (values.Length == 0)
            {
                /* This is a kludge because there is no way to 
                 * represent an empty root node, except as a null reference
                 */
                throw new InvalidOperationException(
                    "No values provided to Node constructor");
            }
            Node<T> lastNode = null;
            for (int i =  values.Length - 1; i > 0; i--)
            {
                T value = values[i];
                lastNode = new Node<T>(value, lastNode);
            }
            Value = values[0];
            NextNode = lastNode;
        }
    }

    public static class NodeHelper
    {
        public static bool TrySkip<T>(ref Node<T> node, int numberToSkip)
        {
            for (int i = 0; i < numberToSkip; i++)
            {
                if (node == null)
                {
                    return false;
                }
                node = node.NextNode;
            }
            return true;
        }

        private static Node<T> AdvanceToEnd<T>(Node<T> startNode, 
            Node<T> rootNode)
        {
            Node<T> node = rootNode;
            while (startNode != null)
            {
                startNode = startNode.NextNode;
                node = node.NextNode;
            }
            return node;
        }

        public static Node<T> FindKLastNode<T>(Node<T> rootNode, int k)
        {
            Node<T> kLastNode = null;
            Node<T> endNodeFinder = rootNode;
            bool canSkipKNodes = TrySkip(ref endNodeFinder, k);
            if (!canSkipKNodes)
            {
                if (rootNode == null)
                {
                    throw new ArgumentNullException("rootNode");
                }
                throw new ArgumentOutOfRangeException("k");
            }
            kLastNode = AdvanceToEnd(endNodeFinder, rootNode);
            return kLastNode;
        }
    }
}
