using System;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1Challenge
{
    public class Stack<T>
    {
        public int StackIndex { get; private set; }
        public Stacker<T> Stacker { get; private set; }

        public bool IsEmpty
        {
            get
            {
                return NextInsertPos == FirstInsertPos;
            }
        }

        public int Count
        {
            get
            {
                return Stacker.GetDistanceBetweenPositions(
                    FirstInsertPos, NextInsertPos) - 1;
            }
        }

        public int FirstInsertPos { get; internal set; }
        public int NextInsertPos { get; internal set; }

        public Stack(Stacker<T> stacker, int stackIndex, int firstInsertPos)
        {
            Stacker = stacker;
            StackIndex = stackIndex;
            NextInsertPos = FirstInsertPos = firstInsertPos;
        }

        public void Push(T item)
        {
            Stacker.Push(StackIndex, item);
        }

        public T Pop()
        {
            return Stacker.Pop(StackIndex);
        }

        internal void Shift(int distance)
        {
            FirstInsertPos = GetNormalizedPosition(FirstInsertPos + distance);
            NextInsertPos = GetNormalizedPosition(NextInsertPos + distance);
        }

        private int GetNormalizedPosition(int pos)
        {
            return Stacker.GetNormalizedPosition(pos);
        }
    }
}
