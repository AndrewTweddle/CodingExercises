using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringAGameUsingRustPatternScorer : BaseForWhenScoringAGame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new RustBowlingScorer();
        }
    }
}
