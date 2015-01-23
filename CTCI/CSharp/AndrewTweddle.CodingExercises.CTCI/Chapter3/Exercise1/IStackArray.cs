using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter3.Exercise1
{
    public interface IStackArray<T>
    {
        int GetCount(int stackNumber);
        void Push(int stackNumber, T item);
        T Pop(int stackNumber);
    }
}
