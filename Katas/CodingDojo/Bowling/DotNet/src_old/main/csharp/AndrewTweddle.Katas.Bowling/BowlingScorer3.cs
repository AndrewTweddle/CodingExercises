using System;
using System.Collections.Generic;

namespace AndrewTweddle.Katas.Bowling
{
    /// <summary>
    /// A bowling scorer which "adapts" BowlingScorer3 
    /// to implement the IBowlingScorer interface.
    /// </summary>
    public class ImmutableBowlingScorerAdapter : IBowlingScorer
    {
        public int Calculate(string throwSymbols)
        {
            BowlingScorer3 scorer = new BowlingScorer3(throwSymbols);
            return scorer.Calculate();
        }
    }

    /// <summary>
    /// An immutable bowling scorer with no large methods
    /// </summary>
    /// <remarks>
    /// Goal: Refactor BowlingScorer2 slightly, so that method signatures
    /// don't have to "thread" the "symbols" parameter through all the calls.
    /// 
    /// Use a private readonly member variable so that the class is
    /// clearly immutable, making it easier to reason about (and thread-safe).
    /// 
    /// Pros: The "readonly" modifier informs that "symbols" is immutable.
    ///       Declutter method signatures by removing the "symbols" parameter.
    /// Cons: The dependency on "symbols" is non-local to the method.
    /// 
    /// </remarks>
    public class BowlingScorer3
    {
        private const int LAST_FRAME = 10;
        private readonly string symbols;

        protected BowlingScorer3() {}

        public BowlingScorer3(string symbols)
        {
            this.symbols = symbols;
        }

        public int Calculate()
        {
            int score = 0;
            int throwIndexForFrame = 0;
            for (int frameNumber = 1; frameNumber <= LAST_FRAME; frameNumber++)
            {
                score += CalculateFrameScore(throwIndexForFrame);
                throwIndexForFrame
                    = GetIndexOfNextFrame(frameNumber, throwIndexForFrame);
            }
            return score;
        }

        public int CalculateFrameScore(int indexOfFirstThrowInFrame)
        {
            if (IsStrike(indexOfFirstThrowInFrame))
            {
                return ScoreFrameWithAStrike(indexOfFirstThrowInFrame);
            }
            if (IsSpare(indexOfFirstThrowInFrame))
            {
                return ScoreFrameWithASpare(indexOfFirstThrowInFrame);
            }
            return ScoreANormalFrame(indexOfFirstThrowInFrame);
        }

        public bool IsStrike(int indexOfFirstThrowInFrame)
        {
            return symbols[indexOfFirstThrowInFrame] == 'X';
        }

        public int ScoreFrameWithAStrike(int indexOfFirstThrowInFrame)
        {
            return 10 + SumOfTwoThrows(indexOfFirstThrowInFrame + 1);
        }

        public bool IsSpare(int indexOfFirstThrowInFrame)
        {
            return symbols[indexOfFirstThrowInFrame + 1] == '/';
        }

        public int ScoreFrameWithASpare(int indexOfFirstThrowInFrame)
        {
            return 10 + GetPinsDown(indexOfFirstThrowInFrame + 2);
        }

        public int ScoreANormalFrame(int indexOfFirstThrowInFrame)
        {
            return SumOfTwoThrows(indexOfFirstThrowInFrame);
        }

        public int GetIndexOfNextFrame(int frameNumber, 
            int indexOfFirstThrowInThisFrame)
        {
            if (frameNumber == LAST_FRAME)
            {
                return symbols.Length;
            }
            if (IsStrike(indexOfFirstThrowInThisFrame))
            {
                return indexOfFirstThrowInThisFrame + 1;
            }
            return indexOfFirstThrowInThisFrame + 2;
        }

        #region Shared utility methods

        public int SumOfTwoThrows(int firstIndex)
        {
            if (symbols[firstIndex + 1] == '/')
                return 10;  // Spare
            return GetPinsDown(firstIndex)
                + GetPinsDown(firstIndex + 1);
        }

        public int GetPinsDown(int index)
        {
            return charToPins[symbols[index]];
        }

        #endregion

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
    }
}
