using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringTheFinalFrame: BaseForWhenScoringTheFinalFrame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new BowlingScorer();
        }
    }
}
