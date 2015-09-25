using System;
using AndrewTweddle.Katas.Bowling;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringTheFinalFrame2: BaseForWhenScoringTheFinalFrame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new BowlingScorer2();
        }
    }
}
