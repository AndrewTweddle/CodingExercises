using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringAGameUsingPatternScorer : BaseForWhenScoringAGame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new PatternBowlingScorer();
        }
    }
}
