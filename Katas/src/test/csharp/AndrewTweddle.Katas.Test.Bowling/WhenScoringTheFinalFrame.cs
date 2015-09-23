using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringTheFinalFrame
    {
        [TestMethod]
        public void ThenExtraThrowsDontScoreForBeingAStrike()
        {
            String throwChars = new string('-', 18) + "xxx";
            int expectedScore = 30;  // Zeroes for 9 frames, 30 for 10th frame
            int actualScore = BowlingScorer.Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }
    }
}
