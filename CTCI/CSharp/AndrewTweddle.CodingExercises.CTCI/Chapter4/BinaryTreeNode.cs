using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter4
{
    public class BinaryTreeNode<T>
    {
        private BinaryTreeNode() { }
        public BinaryTreeNode(T value)
        {
            Value = value;
        }
        public T Value { get; set; }
        public BinaryTreeNode<T> Left { get; set; }
        public BinaryTreeNode<T> Right { get; set; }
    }
}
