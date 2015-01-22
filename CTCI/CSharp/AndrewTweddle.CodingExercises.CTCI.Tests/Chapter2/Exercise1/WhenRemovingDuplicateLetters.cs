using System;
using AndrewTweddle.CodingExercises.CTCI.Chapter2.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter2.Exercise1
{
    [TestClass]
    public class WhenRemovingDuplicateLetters: BaseForWhenRemovingDuplicateLetters
    {
        protected override IDuplicateLetterRemover CreateDuplicateLetterRemover()
        {
            return new DuplicateLetterRemover();
        }
    }
}
