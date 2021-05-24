using System;
using AndrewTweddle.Katas.Bowling;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringAGame3 : BaseForWhenScoringAGame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new ImmutableBowlingScorerAdapter();
        }
    }
}
