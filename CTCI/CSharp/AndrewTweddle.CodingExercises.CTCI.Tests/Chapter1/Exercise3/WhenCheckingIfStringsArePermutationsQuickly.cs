using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter1.Exercise3;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter1.Exercise3
{
    [TestClass]
    public class WhenCheckingIfStringsArePermutationsQuickly: BaseForWhenCheckingIfStringsArePermutations
    {
        protected override CTCI.Chapter1.Exercise3.BasePermutationChecker CreatePermutationChecker()
        {
            return new QuickPermutationChecker();
        }
    }
}
