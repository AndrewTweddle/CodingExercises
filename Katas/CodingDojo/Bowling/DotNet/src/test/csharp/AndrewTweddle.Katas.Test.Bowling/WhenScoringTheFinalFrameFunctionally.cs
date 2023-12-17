using AndrewTweddle.Katas.Bowling;
using AndrewTweddle.Katas.Bowling.FSharp;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringTheFinalFrameFunctionally
        : BaseForWhenScoringTheFinalFrame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new FunctionalBowlingScorer();
        }

    }
}
