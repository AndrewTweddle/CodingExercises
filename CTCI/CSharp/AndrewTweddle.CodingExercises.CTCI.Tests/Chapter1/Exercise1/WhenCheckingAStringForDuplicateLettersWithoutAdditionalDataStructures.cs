using System;
using Microsoft.VisualStudio.TestTools.UnitTesting;
using AndrewTweddle.CodingExercises.CTCI.Chapter1.Exercise1;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter1.Exercise1
{
    [TestClass]
    public class WhenCheckingAStringForDuplicateLettersWithoutAdditionalDataStructures
        : BaseForWhenCheckingAStringForDuplicateLetters
    {
        protected override bool HasDuplicateLetters(string letters)
        {
            return DuplicateLetterChecker.HasDuplicateLettersWithoutAdditionalDataStructures(letters);
        }
    }
}
