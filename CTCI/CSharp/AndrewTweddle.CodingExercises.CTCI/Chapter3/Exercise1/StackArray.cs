using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1
{
    /// <summary>
    /// A data structure that solves the problem of 
    /// implementing multiple stacks using a single array.
    /// </summary>
    /// <remarks>
    /// The original problem was to store 3 stacks in a single array.
    /// The counts array is a second array. So this deviates from the specification.
    /// This can be addressed by using 3 variables instead of a counts array:
    ///     stack0Count, stack1Count, stack2Count.
    /// 
    /// Another issue with this solution is that it doesn't use the space efficiently.
    /// If the used sizes of the stacks is very different, then there will be a lot of unused space.
    /// 
    /// </remarks>
    /// <typeparam name="T">The data type of the items stored in the stacks</typeparam>
    public class StackArray<T>: IStackArray<T>
    {
        private int[] counts;
        private T[] stacks;

        public int StackCount { get; private set; }
        public int MaxStackSize { get; private set; }

        protected StackArray() {}

        public StackArray(int stackCount, int maxStackSize)
        {
            if (stackCount < 1)
            {
                throw new ArgumentOutOfRangeException("stackCount");
            }

            if (maxStackSize < 0)
            {
                throw new ArgumentOutOfRangeException("maxStackSize");
            }

            StackCount = stackCount;
            MaxStackSize = maxStackSize;
            counts = new int[stackCount];
            stacks = new T[stackCount * maxStackSize];
        }

        public int GetCount(int stackNumber)
        {
            CheckStackNumber(stackNumber);
            return counts[stackNumber];
        }

        public void Push(int stackNumber, T item)
        {
            CheckStackNumber(stackNumber);
            int count = GetCount(stackNumber);
            if (count == MaxStackSize)
            {
                throw new IndexOutOfRangeException(
                    "The item can't be pushed onto the stack as the maximum size will be exceeded");
            }
            int index = GetArrayIndex(stackNumber, count + 1);
            stacks[index] = item;
            counts[stackNumber] = count + 1;
        }

        public T Pop(int stackNumber)
        {
            CheckStackNumber(stackNumber);
            int itemCount = counts[stackNumber];
            if (itemCount == 0)
            {
                throw new InvalidOperationException(
                    String.Format(
                        "Unable to pop an item off empty stack number {0}",
                        stackNumber));
            }
            counts[stackNumber] = itemCount - 1;
            int index = GetArrayIndex(stackNumber, itemCount);
            T item = stacks[index];
            return item;
        }

        private int GetArrayIndex(int stackNumber, int count)
        {
            return stackNumber * MaxStackSize + count - 1;
        }

        private void CheckStackNumber(int stackNumber)
        {
            if (stackNumber < 0 || stackNumber >= StackCount)
            {
                throw new IndexOutOfRangeException(
                    String.Format("The stack number must be between 0 and {0}",
                        StackCount - 1));
            }
        }
    }
}
