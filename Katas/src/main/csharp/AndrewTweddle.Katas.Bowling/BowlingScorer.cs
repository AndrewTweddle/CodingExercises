using System.Linq;

namespace AndrewTweddle.Katas.Bowling
{
    public static class BowlingScorer
    {
        internal class Frame
        {
            public int StartScoreIndex { get; private set; }
            public int EndScoreIndex { get; private set; }

            public Frame(int startScoreIndex, int endScoreIndex)
            {
                StartScoreIndex = startScoreIndex;
                EndScoreIndex = endScoreIndex;
            }

            public int CalculateScore(int[] values)
            {
                int score = 0;
                for (int i = StartScoreIndex; i <= EndScoreIndex; i++)
                {
                    score += values[i];
                }
                return score;
            }
        }

        public static int Calculate(string throwSymbols)
        {
            char[] throws = throwSymbols.ToCharArray();
            int throwCount = throws.Length;
            int[] values = new int[throwCount]; 
                // i.e. # of pins knocked down in each throw

            /* Complication: 
             * We don't know if 'X33' at the end is 10th frame or 9th and 10th.
             * So we can't know to add 10 + 3 + 3, or (10 + 3 + 3) + (3 + 3).
             * Hence we are forced to keep track of the frame.
             */
            Frame[] frames = new Frame[10];

            int frameNumber = 1;
            int startScoreIndex = 0;

            for (int i = 0; i < throwCount; i++)
            {
                char symbol = throws[i];
                int value = 0;
                switch (symbol)
                {
                    case 'X':
                    case 'x':
                        value = 10;
                        if (frameNumber != 10)
                        {
                            Frame frame = new Frame(startScoreIndex, i + 2);
                            frames[frameNumber - 1] = frame;
                            frameNumber++;
                            startScoreIndex = i + 1;
                        }
                        break;
                    case '/':
                        value = 10 - values[i-1];  
                            // NB: This assumes that first throw is not a spare

                        if (frameNumber != 10)
                        {
                            Frame frame = new Frame(startScoreIndex, i + 1);
                            frames[frameNumber - 1] = frame;
                            frameNumber++;
                            startScoreIndex = i + 1;
                        }
                        break;
                    case '-':
                        value = 0;
                        break;
                    default:
                        value = int.Parse(throwSymbols.Substring(i, 1));
                        break;
                }
                values[i] = value;
                if (frameNumber < 10 && i == startScoreIndex + 1)
                {
                    Frame frame = new Frame(startScoreIndex, i);
                    frames[frameNumber - 1] = frame;
                    frameNumber++;
                    startScoreIndex = i + 1;
                }
                if (frameNumber == 10 && i == throwCount - 1)
                {
                    Frame frame = new Frame(startScoreIndex, i);
                    frames[frameNumber - 1] = frame;
                }
            }
            return frames.Sum(f => f.CalculateScore(values));
        }
    }
}
