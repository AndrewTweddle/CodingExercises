using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.Katas.Bowling
{
    public interface IBowlingScorer
    {
        int Calculate(string throwSymbols);
    }
}
