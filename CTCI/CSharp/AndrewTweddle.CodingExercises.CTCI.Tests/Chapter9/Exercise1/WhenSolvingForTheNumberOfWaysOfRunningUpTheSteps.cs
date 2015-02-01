using System;
using System.Diagnostics;
using AndrewTweddle.CodingExercises.CTCI.Chapter9.Exercise1;
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace AndrewTweddle.CodingExercises.CTCI.Tests.Chapter9.Exercise1
{
    [TestClass]
    public class WhenSolvingForTheNumberOfWaysOfRunningUpTheSteps
    {
        [TestMethod, ExpectedException(typeof(ArgumentOutOfRangeException))]
        public void ThenTheSolverCannotSolveForValuesGreaterThanTheMaximumItIsConfiguredFor()
        {
            int maxN = 50;
            StepsSolver solver = new StepsSolver(maxN);
            solver.Solve(maxN + 1);
        }

        [TestMethod]
        public void ThenTheSolverCanSolveForTheMaximumValueOfNItIsConfiguredFor()
        {
            int maxN = 50;
            StepsSolver solver = new StepsSolver(maxN);
            solver.Solve(maxN);
        }

        [TestMethod]
        public void ThenANegativeNumberOfRemainingStepsIsImpossibleAndReturnsZero()
        {
            StepsSolver solver = new StepsSolver(-1);

            // Test for the first 4 negative numbers, due to 
            // the formula being expressed in terms of the previous 3 numbers:
            for (int i = 1; i <= 4; i++)
            {
                long solution = solver.Solve(-i);
                Assert.AreEqual(0, solution, 
                    String.Format("The solver should return zero for an input of -{0}", i));
            }
        }

        [TestMethod]
        public void ThenThereIsOnlyOneWayOfGettingToTheTopIfAlreadyThereWhichIsToDoNothing()
        {
            StepsSolver solver = new StepsSolver(0);
            Assert.AreEqual(1, solver.Solve(0));
        }

        [TestMethod]
        public void ThenThereIsOneWayOfHoppingToTheTopIfOneStepAway()
        {
            int n = 1;
            StepsSolver solver = new StepsSolver(n);
            Assert.AreEqual(1, solver.Solve(n));
        }

        [TestMethod]
        public void ThenThereAreTwoWaysOfHoppingToTheTopIfTwoStepsAway()
        {
            // i.e. A hop of 2, or two hops of 1 step each
            int n = 2;
            StepsSolver solver = new StepsSolver(n);
            Assert.AreEqual(2, solver.Solve(n));
        }

        [TestMethod]
        public void ThenThereAreFourWaysOfHoppingToTheTopIfThreeStepsAway()
        {
            // i.e. A hop of 3, a hop of 2 then 1, a hop of 1 then 2, or 3 hops of 1 step each
            int n = 3;
            StepsSolver solver = new StepsSolver(n);
            Assert.AreEqual(4, solver.Solve(n));
        }

        [TestMethod]
        public void ThenTheSolverSolvesALargeProblemEfficiently()
        {
            // Note : This occasionally times out for values of n over 9000
            int n = 1000;
            TimeSpan maxDuration = TimeSpan.FromSeconds(1);
            Stopwatch swatch = Stopwatch.StartNew();
            StepsSolver solver = new StepsSolver(n, checkForOverflow: false);
            solver.Solve(n);
            swatch.Stop();
            Assert.IsTrue(swatch.Elapsed < maxDuration);
        }

        [TestMethod]
        public void ThenTheSolverCanBeRunRepeatedlyOnValuesOfNUpToItsMaximumConfiguredValue()
        {
            int maxN = 50;
            int smallN = 10;
            int mediumN = 20;
            StepsSolver solver = new StepsSolver(maxN);
            long smallSolution = solver.Solve(smallN);
            long largeSolution = solver.Solve(maxN);

            // Check that it still gives the same answer for the small solution:
            Assert.AreEqual(smallSolution, solver.Solve(smallN));

            // Check that it still gives the same answer for the large solution:
            Assert.AreEqual(largeSolution, solver.Solve(maxN));

            // Check that it can be run on an intermediate value after solving for the maximum value:
            long mediumSolution = solver.Solve(mediumN);
        }
    }
}
