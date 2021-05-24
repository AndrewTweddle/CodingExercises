using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringAGame: BaseForWhenScoringAGame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new BowlingScorer();
        }
    }
}
