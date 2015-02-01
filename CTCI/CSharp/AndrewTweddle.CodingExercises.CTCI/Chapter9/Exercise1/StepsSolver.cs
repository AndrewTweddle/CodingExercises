using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace AndrewTweddle.CodingExercises.CTCI.Chapter9.Exercise1
{
    /// <summary>
    /// A solver to determine in how many different ways a boy can hop up n stairs 
    /// if he can hop up 1, 2 or 3 steps on each hop (energetically varying these at will).
    /// </summary>
    /// <remarks>
    /// <para>
    /// It is Mathematically convenient to define the solution f(n) for n remaining steps
    /// to be 1 when there are 0 remaining steps, and 0 for negative numbers of steps.
    /// This allows the formula f(n) = f(n-1) + f(n-2) + f(n-3)
    /// to also hold for the values 1, 2 and 3.
    /// </para>
    /// <para>
    /// For efficiency the solver caches smaller solutions to the problem.
    /// This allows it to be efficiently called for any number 
    /// up to maxN, the maximum value of n configured in the constructor.
    /// </para>
    /// </remarks>
    public class StepsSolver
    {
        private long[] solutionCache;
        private int maxN;
        private bool checkForOverflow;

        /// <summary>
        /// Force a maximum cache size to be chosen, by disallowing the parameterless constructor.
        /// </summary>
        private StepsSolver() { }

        /// <summary>
        /// Configure the solver
        /// </summary>
        /// <param name="maxN">The maximum input size to solve for</param>
        /// <param name="checkForOverflow">
        /// Indicates whether to check for overflow. 
        /// It is only used for unit tests to prove the efficiency of the algorithm.
        /// So disabling overflow checking is really only for experimentation,
        /// and should not be added to production code.
        /// </param>
        public StepsSolver(int maxN, bool checkForOverflow = true)
        {
            this.maxN = maxN;
            this.checkForOverflow = checkForOverflow;
            solutionCache = new long[maxN + 1];
        }

        public long Solve(int n)
        {
            // There is no way to do the impossible, and be on a higher step than the highest:
            if (n < 0) 
            { 
                return 0; 
            }

            // There is only one way to get to the top of the stairs if already there - do nothing!
            if (n == 0)
            {
                return 1;
            }

            if (n > maxN)
            {
                throw new ArgumentOutOfRangeException("n",
                    String.Format("n = {0}, which is greater than the maximum configured value of {1}", 
                    n, maxN));
            }

            long cachedSolution = solutionCache[n];
            if (cachedSolution == 0)
            {
                // Hop 1, 2 or 3 steps and recalculate with the smaller number of steps remaining:
                if (checkForOverflow)
                {
                    checked
                    {
                        cachedSolution = Solve(n - 1) + Solve(n - 2) + Solve(n - 3);
                    }
                }
                else
                {
                    cachedSolution = Solve(n - 1) + Solve(n - 2) + Solve(n - 3);
                }
                solutionCache[n] = cachedSolution;
            }
            return cachedSolution;
        }
    }
}
