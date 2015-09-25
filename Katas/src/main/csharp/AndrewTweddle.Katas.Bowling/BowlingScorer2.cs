using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.Katas.Bowling
{
    public class BowlingScorer2 : IBowlingScorer
    {
        private const int LAST_FRAME = 10;

        private Dictionary<char, int> charToPins = new Dictionary<char, int>
        {
            {'-', 0 },
            {'1', 1 },
            {'2', 2 },
            {'3', 3 },
            {'4', 4 },
            {'5', 5 },
            {'6', 6 },
            {'7', 7 },
            {'8', 8 },
            {'9', 9 },
            {'X', 10 },
        };

        public int GetPinsDown(string symbols, int index)
        {
            return charToPins[symbols[index]];
        }

        public int SumOfTwoThrows(string symbols, int firstIndex)
        {
            if (symbols[firstIndex + 1] == '/')
                return 10;  // Spare
            return GetPinsDown(symbols, firstIndex)
                + GetPinsDown(symbols, firstIndex + 1);
        }

        public bool IsStrike(string symbols, int indexOfFirstThrowInFrame)
        {
            return symbols[indexOfFirstThrowInFrame] == 'X';
        }

        public bool IsSpare(string symbols, int indexOfFirstThrowInFrame)
        {
            return symbols[indexOfFirstThrowInFrame + 1] == '/';
        }

        public int ScoreFrameWithAStrike(string symbols, 
            int indexOfFirstThrowInFrame)
        {
            return 10 + SumOfTwoThrows(symbols, indexOfFirstThrowInFrame + 1);
        }

        public int ScoreFrameWithASpare(string symbols,
            int indexOfFirstThrowInFrame)
        {
            return 10 + GetPinsDown(symbols, indexOfFirstThrowInFrame + 2);
        }

        public int ScoreANormalFrame(string symbols,
            int indexOfFirstThrowInFrame)
        {
            return SumOfTwoThrows(symbols, indexOfFirstThrowInFrame);
        }

        public int CalculateFrameScore(string symbols,
            int indexOfFirstThrowInFrame)
        {
            if (IsStrike(symbols, indexOfFirstThrowInFrame))
            {
                return ScoreFrameWithAStrike(symbols, indexOfFirstThrowInFrame);
            }
            if (IsSpare(symbols, indexOfFirstThrowInFrame))
            {
                return ScoreFrameWithASpare(symbols, indexOfFirstThrowInFrame);
            }
            return ScoreANormalFrame(symbols, indexOfFirstThrowInFrame);
        }

        public int GetIndexOfNextFrame(string symbols,
            int indexOfFirstThrowInThisFrame)
        {
            if (IsStrike(symbols, indexOfFirstThrowInThisFrame))
            {
                return indexOfFirstThrowInThisFrame + 1;
            }
            return indexOfFirstThrowInThisFrame + 2;
        }

        public int Calculate(string symbols)
        {
            int score = 0;
            int throwIndexForFrame = 0;
            for (int frameNumber = 1; frameNumber <= LAST_FRAME; frameNumber++)
            {
                score += CalculateFrameScore(symbols, throwIndexForFrame);
                throwIndexForFrame 
                    = GetIndexOfNextFrame(symbols, throwIndexForFrame);
            }
            return score;
        }
    }
}
