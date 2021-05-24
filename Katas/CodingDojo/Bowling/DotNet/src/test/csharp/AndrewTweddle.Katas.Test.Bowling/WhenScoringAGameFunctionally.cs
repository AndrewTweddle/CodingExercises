using System;
using AndrewTweddle.Katas.Bowling;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.Katas.Bowling.FSharp;

namespace AndrewTweddle.Katas.Test.Bowling
{
    [TestClass]
    public class WhenScoringAGameFunctionally: BaseForWhenScoringAGame
    {
        protected override IBowlingScorer GetScorer()
        {
            return new FunctionalBowlingScorer();
        }
    }
}
