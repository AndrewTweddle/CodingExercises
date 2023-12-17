using System.Linq;

namespace AndrewTweddle.Katas.Bowling
{
    public class BowlingScorer: IBowlingScorer
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

        private const int LAST_FRAME_INDEX = 9;  // since zero-based

        public int Calculate(string throwSymbols)
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

            int frameIndex = 0;
            int startScoreIndex = 0;

            for (int i = 0; i < throwCount; i++)
            {
                switch (throws[i])
                {
                    case 'X':
                    case 'x':
                        values[i] = 10;
                        if (frameIndex != LAST_FRAME_INDEX)
                        {
                            Frame frame = new Frame(startScoreIndex, i + 2);
                            frames[frameIndex] = frame;
                            frameIndex++;
                            startScoreIndex = i + 1;
                        }
                        break;
                    case '/':
                        values[i] = 10 - values[i-1];  
                            // NB: This assumes that first throw is not a spare

                        if (frameIndex != LAST_FRAME_INDEX)
                        {
                            Frame frame = new Frame(startScoreIndex, i + 1);
                            frames[frameIndex] = frame;
                            frameIndex++;
                            startScoreIndex = i + 1;
                        }
                        break;
                    case '-':
                        values[i] = 0;
                        break;
                    default:
                        values[i] = int.Parse(throwSymbols.Substring(i, 1));
                        break;
                }

                // Finalize a frame after 2 throws (except final frame):
                // NB: Strikes have already closed the frame after first throw.
                if (frameIndex < LAST_FRAME_INDEX && i == startScoreIndex + 1)
                {
                    Frame frame = new Frame(startScoreIndex, i);
                    frames[frameIndex] = frame;
                    frameIndex++;
                    startScoreIndex = i + 1;
                }

                if (frameIndex == LAST_FRAME_INDEX && i == throwCount - 1)
                {
                    // Finalize the tenth and final frame:
                    Frame frame = new Frame(startScoreIndex, i);
                    frames[frameIndex] = frame;
                }
            }
            return frames.Sum(f => f.CalculateScore(values));
        }
    }
}
