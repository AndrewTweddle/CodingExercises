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

        [TestMethod]
        public void ThenARepeatingStrikeFollowedByTwoZeroesScores50()
        {
            string symbols = new string('~', 5).Replace("~", "X--");
            // i.e. A strike in one frame, and two zeroes in the next, repeated
            int expectedScore = 50;
            int actualScore = BowlingScorer.Calculate(symbols);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenKnockingDown5PinsOnEveryThrowScores150()
        {
            string symbols = new string('~', 10).Replace("~", "5/") + "5";
            // i.e. Two 5's in all frames, except three 5's in the tenth.
            int expectedScore = 15*10;
            int actualScore = BowlingScorer.Calculate(symbols);
            Assert.AreEqual(expectedScore, actualScore);
        }
    }
}
