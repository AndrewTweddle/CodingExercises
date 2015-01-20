using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter1.Exercise1
{
    [TestClass]
    public class WhenCheckingAStringForDuplicateLetters: BaseForWhenCheckingAStringForDuplicateLetters
    {
        protected override bool HasDuplicateLetters(string letters)
        {
            return Exercise1_1.HasDuplicateLetters(letters);
        }
    }
}
