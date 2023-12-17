using AndrewTweddle.Katas.Bowling;

namespace AndrewTweddle.Katas.Test.Bowling
{
    public abstract class BaseForWhenScoringTheFinalFrame
    {
        protected abstract IBowlingScorer GetScorer();

        #region Check that bonus throws in final frame are only counted once

        [TestMethod]
        public void ThenExtraThrowsDontScoreForBeingAStrike()
        {
            String throwChars = new string('-', 18) + "XXX";
            int expectedScore = 30;
            // Zeroes for 9 frames, 30 for 10th frame
            int actualScore = GetScorer().Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenExtraThrowsDontScoreForBeingASpare()
        {
            String throwChars = new string('-', 18) + "X3/";
            int expectedScore = 20;
            // Zeroes for 9 frames, final spare has no subsequent throw
            int actualScore = GetScorer().Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        #endregion

        #region Count throws correctly based on whether a strike/spare in ninth

        [TestMethod]
        public void ThenTenthFrameThrowsAreAddedTwiceIfAStrikeOnTheNinthFrame()
        {
            String throwChars = new string('-', 16) + "X33";
            int expectedScore = 22;  // Ninth + tenth: (10 + 3 + 3) + (3 + 3)
            int actualScore = GetScorer().Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenTenthFrameThrowsAreAddedOnceIfNoStrikeInNinthFrame()
        {
            String throwChars = new string('-', 18) + "X33";
            int expectedScore = 16;  // Tenth: (10 + 3 + 3)
            int actualScore = GetScorer().Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenFirstThrowOfTenthFrameIsAddedTwiceIfNinthIsASpare()
        {
            String throwChars = new string('-', 16) + "4/33";
            int expectedScore = 19;  // Ninth + tenth: (4 + 6 + 3) + (3 + 3)
            int actualScore = GetScorer().Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        [TestMethod]
        public void ThenFirstThrowOfTenthFrameIsAddedOnceIfNoSpareOnTheNinth()
        {
            String throwChars = new string('-', 16) + "4533";
            int expectedScore = 15;  // Ninth + tenth: (4 + 5) + (3 + 3)
            int actualScore = GetScorer().Calculate(throwChars);
            Assert.AreEqual(expectedScore, actualScore);
        }

        #endregion
    }
}
