using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringAGame2: BaseForWhenScoringAGame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new BowlingScorer2();
        }
    }
}
