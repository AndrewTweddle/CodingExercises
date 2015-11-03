using System;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1Challenge
{
    /// <summary>
    /// The stacker provides the backing array and coordination code
    /// for stacks that share that array
    /// </summary>
    /// <typeparam name="T"></typeparam>
    public class Stacker<T>
    {
        public const int STACK_COUNT = 3;

        private Stack<T>[] stacks;
        private T[] ringBuffer;

        public Stacker(int size)
        {
            // Don't allow overlap of stacks:
            if (size < STACK_COUNT)
            {
                throw new ArgumentOutOfRangeException("size",
                    "The stacker must have a backing array at least as large as the number of stacks");
            }
            ringBuffer = new T[size];
            stacks = new Stack<T>[STACK_COUNT];
            for (int i = 0; i < STACK_COUNT; i++)
            {
                stacks[i] = new Stack<T>(this, i, i * BufferSize / STACK_COUNT);
            }
        }

        public Stack<T> this[int stackIndex]
        {
            get
            {
                CheckStackIndex(stackIndex);
                return stacks[stackIndex];
            }
        }

        public int BufferSize
        {
            get
            {
                return ringBuffer.Length;
            }
        }

        public bool IsEmpty(int stackIndex)
        {
            CheckStackIndex(stackIndex);
            return stacks[stackIndex].IsEmpty;
        }

        public T Pop(int stackIndex)
        {
            CheckStackIndex(stackIndex);
            if (IsEmpty(stackIndex))
            {
                string message = String.Format(
                    "Cannot pop off empty stack {0}", stackIndex);
                throw new InvalidOperationException(message);
            }
            Stack<T> stack = stacks[stackIndex];
            int itemIndex = GetNormalizedPosition(stack.NextInsertPos - 1);
            T item = ringBuffer[itemIndex];
            ringBuffer[itemIndex] = default(T);
            /* i.e. ensure lingering presence in ring buffer
             * won't prevent GC from collecting the item
             */
            stack.NextInsertPos = itemIndex;
            return item;
        }

        /// <summary>
        /// Push a new item onto one of the sacks
        /// </summary>
        /// <param name="stackIndex"></param>
        /// <param name="item"></param>
        /// <remarks>
        /// If space needs to be made for the new item, then the algorithm 
        /// will only move a single stack, not two adjacent stacks.
        /// If it can move either the current stack left, or the next stack right,
        /// then it will choose the one that 
        /// </remarks>
        public void Push(int stackIndex, T item)
        {
            CheckStackIndex(stackIndex);
            if (IsAdjacentToNextStack(stackIndex))
            {
                MakeSpaceToPushOntoStack(stackIndex);
            }
            PushWhenThereIsAGapAfterThisStack(stackIndex, item);
        }

        private void MakeSpaceToPushOntoStack(int stackIndex)
        {
            int nextStackIndex = GetNormalizedStackIndex(stackIndex + 1);
            bool isAdjacentToPrev = IsAdjacentToPrevStack(stackIndex);
            bool areOtherTwoStacksAdjacent = IsAdjacentToNextStack(nextStackIndex);
            if (isAdjacentToPrev)
            {
                if (areOtherTwoStacksAdjacent)
                {
                    throw new InvalidOperationException(
                        "There is insufficient space in the stacker to add another item");
                }
                MakeSpaceByMovingNextStackToTheRight(stackIndex, nextStackIndex);
                return;
            }
            // So the current stack is not adjacent to the previous stack:
            if (areOtherTwoStacksAdjacent 
                || IsItBestToMoveCurrentStackLeft(stackIndex, nextStackIndex))
            {
                MakeSpaceByMovingTheCurrentStackToTheLeft(stackIndex);
            }
            else
            {
                MakeSpaceByMovingNextStackToTheRight(stackIndex, nextStackIndex);
            }
        }

        private bool IsItBestToMoveCurrentStackLeft(int stackIndex, int nextStackIndex)
        {
            // Move the smaller stack (one of many possible strategies):
            int sizeOfStack = stacks[stackIndex].Count;  // NB: size before pushing
            int sizeOfNextStack = stacks[nextStackIndex].Count;
            return sizeOfStack <= sizeOfNextStack;
        }

        private void MakeSpaceByMovingTheCurrentStackToTheLeft(int stackIndex)
        {
            Stack<T> currStack = stacks[stackIndex];
            Stack<T> nextStack = stacks[GetNextStackIndex(stackIndex)];
            Stack<T> prevStack = stacks[GetPrevStackIndex(stackIndex)];
            int sizeOfCurrStack = currStack.Count + 1;
            int gapAfterCurrStack = GetDistanceBetweenPositions(
                currStack.NextInsertPos, nextStack.FirstInsertPos) - 1;
            int sizeOfPrevStack = prevStack.Count;
            int gapAfterPrevStack = GetDistanceBetweenPositions(
                prevStack.NextInsertPos, currStack.FirstInsertPos) - 1;
            int sizeOfBothStacks = sizeOfPrevStack + sizeOfCurrStack;
            int gapsAfterBothStacks = gapAfterPrevStack + gapAfterCurrStack;
            int idealGapAfterPrevStack = (int) Math.Floor(
                sizeOfPrevStack * sizeOfBothStacks / gapsAfterBothStacks + 0.5);
            int distanceToMove = gapAfterPrevStack - idealGapAfterPrevStack;
            if (distanceToMove <= 0)
            {
                // The ideal gap is unattainable. Just move one space left:
                distanceToMove = 1;
            }
            MoveStackLeft(stackIndex, distanceToMove);
        }

        private void MoveStackLeft(int stackIndex, int distanceToMove)
        {
            if (distanceToMove < 0)
            {
                throw new ArgumentOutOfRangeException("distanceToMove",
                    "A stack cannot be moved a negative distance");
            }
            if (distanceToMove == 0)
            {
                return;
            }
            Stack<T> currStack = stacks[stackIndex];
            int startPos = currStack.FirstInsertPos;
            int count = currStack.Count;
            int overwriteCount = count - distanceToMove;
            for (int i = 0; i < count; i++)
            {
                int srcPos = GetNormalizedPosition(startPos + i);
                int destPos = GetNormalizedPosition(startPos + i - distanceToMove);
                ringBuffer[destPos] = ringBuffer[srcPos];
                if (i >= overwriteCount)
                {
                    ringBuffer[srcPos] = default(T);
                    /* Only really necessary if T is a reference type,
                       so that debris in the ring buffer doesn't prevent 
                       garbage collection of an otherwise unreferenced object
                    */
                }
            }
        }

        private void MakeSpaceByMovingNextStackToTheRight(int stackIndex, int nextStackIndex)
        {
            Stack<T> currStack = stacks[stackIndex];
            Stack<T> nextStack = stacks[nextStackIndex];
            Stack<T> prevStack = stacks[GetPrevStackIndex(stackIndex)];
            int sizeOfCurrStack = currStack.Count + 1;
            int gapAfterCurrStack = GetDistanceBetweenPositions(
                currStack.NextInsertPos, nextStack.FirstInsertPos) - 2;
            int sizeOfNextStack = nextStack.Count;
            int gapAfterNextStack = GetDistanceBetweenPositions(
                nextStack.NextInsertPos, prevStack.FirstInsertPos) - 1;
            int sizeOfBothStacks = sizeOfCurrStack + sizeOfNextStack;
            int gapsAfterBothStacks = gapAfterCurrStack + gapAfterNextStack;
            int idealGapAfterCurrStack = (int)Math.Floor(
                sizeOfCurrStack * sizeOfBothStacks / gapsAfterBothStacks + 0.5);
            int distanceToMove = idealGapAfterCurrStack - gapAfterCurrStack;
            if (distanceToMove <= 0)
            {
                // The ideal gap is unattainable. Just move one space right:
                distanceToMove = 1;
            }
            MoveStackRight(stackIndex, distanceToMove);
        }

        private void MoveStackRight(int stackIndex, int distanceToMove)
        {
            if (distanceToMove < 0)
            {
                throw new ArgumentOutOfRangeException("distanceToMove",
                    "A stack cannot be moved a negative distance");
            }
            if (distanceToMove == 0)
            {
                return;
            }
            Stack<T> currStack = stacks[stackIndex];
            int startPos = currStack.FirstInsertPos;
            int count = currStack.Count;
            int overwriteCount = count - distanceToMove;
            for (int i = count - 1; i >= 0; i--)
            {
                int srcPos = GetNormalizedPosition(startPos + i);
                int destPos = GetNormalizedPosition(startPos + i + distanceToMove);
                ringBuffer[destPos] = ringBuffer[srcPos];
                if (i >= overwriteCount)
                {
                    ringBuffer[srcPos] = default(T);
                    /* Only really necessary if T is a reference type,
                       so that debris in the ring buffer doesn't prevent 
                       garbage collection of an otherwise unreferenced object
                    */
                }
            }
        }

        private void PushWhenThereIsAGapAfterThisStack(int stackIndex, T item)
        {
            Stack<T> stack = stacks[stackIndex];
            ringBuffer[stack.NextInsertPos] = item;
            stack.NextInsertPos = GetNormalizedPosition(stack.NextInsertPos + 1);
        }

        private bool IsAdjacentToPrevStack(int stackIndex)
        {
            int prevStackIndex = GetPrevStackIndex(stackIndex);
            Stack<T> currStack = stacks[stackIndex];
            Stack<T> prevStack = stacks[prevStackIndex];
            return prevStack.NextInsertPos == currStack.FirstInsertPos;
        }

        private bool IsAdjacentToNextStack(int stackIndex)
        {
            int nextStackIndex = GetNextStackIndex(stackIndex);
            Stack<T> currStack = stacks[stackIndex];
            Stack<T> nextStack = stacks[nextStackIndex];
            return currStack.NextInsertPos == nextStack.FirstInsertPos;
        }

        private int GetPrevStackIndex(int stackIndex)
        {
            return GetNormalizedStackIndex(stackIndex - 1);
        }

        private int GetNextStackIndex(int stackIndex)
        {
            return GetNormalizedStackIndex(stackIndex + 1);
        }

        private int GetNormalizedStackIndex(int stackIndex)
        {
            return (stackIndex + STACK_COUNT) % STACK_COUNT;
        }

        private void CheckStackIndex(int stackIndex, string propertyName = "stackIndex")
        {
            if (stackIndex < 0 || stackIndex >= STACK_COUNT)
            {
                throw new ArgumentOutOfRangeException(propertyName, 
                    "The stack index is out of range");
            }
        }

        internal int GetNormalizedPosition(int pos)
        {
            int bufferSize = BufferSize;
            return (pos + bufferSize) % bufferSize;
        }

        internal int GetDistanceBetweenPositions(int startPos, int endPos)
        {
            return GetNormalizedPosition(endPos - startPos + 1);
        }
    }
}
