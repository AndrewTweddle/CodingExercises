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
    }

    public static class NodeHelper
    {
        public static bool TrySkip<T>(ref Node<T> node, int numberToSkip)
        {
            for (int i = 0; i < numberToSkip; i++)
            {
                if (node.NextNode == null) return false;
                node = node.NextNode;
            }
            return true;
        }

        private static Node<T> FindLastNode<T>(Node<T> startNode, 
            Node<T> rootNode)
        {
            while (startNode.NextNode != null)
            {
                startNode = startNode.NextNode;
                rootNode = rootNode.NextNode;
            }
            return rootNode;
        }

        public static bool TryFindKLastNode<T>(Node<T> rootNode, int k, 
            out Node<T> kLastNode)
        {
            Node<T> endNodeFinder = rootNode;
            bool canSkipKNodes = TrySkip(ref endNodeFinder, k);
            if (canSkipKNodes)
            {
                kLastNode = FindLastNode(endNodeFinder, rootNode);
            }
            else
            {
                kLastNode = null;
            }
            return canSkipKNodes;
        }
    }
}
