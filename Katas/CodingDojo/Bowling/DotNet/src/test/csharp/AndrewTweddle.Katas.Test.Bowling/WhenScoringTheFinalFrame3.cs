using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringTheFinalFrame3 : BaseForWhenScoringTheFinalFrame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new ImmutableBowlingScorerAdapter();
        }
    }
}
