using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringTheFinalFrame
    {
        #region Check that bonus throws in final frame are only counted once

        [TestMethod]
        public void ThenExtraThrowsDontScoreForBeingAStrike()
        {
            String throwChars = new string('-', 18) + "xxx";
            int expectedScore = 30;  // Zeroes for 9 frames, 30 for 10th frame
            int actualScore = BowlingScorer.Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenExtraThrowsDontScoreForBeingASpare()
        {
            String throwChars = new string('-', 18) + "x3/";
            int expectedScore = 20;  // Zeroes for 9 frames, final spare has no subsequent throw to increase
            int actualScore = BowlingScorer.Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        #endregion

        #region Count throws correctly based on whether a strike/spare in ninth

        [TestMethod]
        public void ThenTheTenthFrameThrowsAreAddedTwiceIfThereWasAStrikeOnTheNinthFrame()
        {
            String throwChars = new string('-', 16) + "x33";
            int expectedScore = 22;  // Ninth + tenth: (10 + 3 + 3) + (3 + 3)
            int actualScore = BowlingScorer.Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenTheTenthFrameThrowsAreAddedOnceIfThereWasNotAStrikeOnTheNinthFrame()
        {
            String throwChars = new string('-', 18) + "x33";
            int expectedScore = 16;  // Tenth: (10 + 3 + 3)
            int actualScore = BowlingScorer.Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenTheFirstThrowOfTheTenthFrameIsAddedTwiceIfThereWasASpareOnTheNinthFrame()
        {
            String throwChars = new string('-', 14) + "4/33";
            int expectedScore = 19;  // Ninth + tenth: (4 + 6 + 3) + (3 + 3)
            int actualScore = BowlingScorer.Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenTheFirstThrowOfTheTenthFrameIsAddedOnceIfThereWasNotASpareOnTheNinthFrame()
        {
            String throwChars = new string('-', 14) + "4533";
            int expectedScore = 15;  // Ninth + tenth: (4 + 5) + (3 + 3)
            int actualScore = BowlingScorer.Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        #endregion
    }
}
