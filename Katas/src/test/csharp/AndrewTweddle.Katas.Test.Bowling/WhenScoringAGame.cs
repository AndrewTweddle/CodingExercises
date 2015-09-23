using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringAGame
    {
        [TestMethod]
        public void ThenAllStrikesScores300()
        {
            string symbols = new string('X', 12);
                // i.e. 10 strikes and 2 bonus throws in the tenth frame
            int expectedScore = 300;
            int actualScore = BowlingScorer.Calculate(symbols);
            Assert.AreEqual(expectedScore, actualScore);
        }
    }
}
