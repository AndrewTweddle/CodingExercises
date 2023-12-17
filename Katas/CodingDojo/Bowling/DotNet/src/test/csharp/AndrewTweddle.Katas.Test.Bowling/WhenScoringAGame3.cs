using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringAGame3 : BaseForWhenScoringAGame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new ImmutableBowlingScorerAdapter();
        }
    }
}
