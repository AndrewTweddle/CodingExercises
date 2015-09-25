# Bowling Kata (version 1.0)

The following notes are captured from my notepad (physical not electronic!)

## Requirements

1. Provide final score
2. Assume inputs are valid. Input is a string with 1 character per throw:
  1. '-' = 0 throw
  2. 'X' = Strike (add value of following 2 throws to current frame's score, unless one of last 2 throws in tenth/final frame)
  3. '/' = Spare (add value of following throw to current frame's score, unless one of the last 2 throws in tenth/final frame)
  4. Digit = value of throw.
3. If a strike, and not in tenth frame, current frame has a single throw, but score for frame is 10 (for strike) + value of next 2 throws.
4. If a spare, and not in tenth frame, current frame has 2 throws, and score for frame is 10 (for spare) + value of next throw.
5. If a strike, and in tenth frame, throw 2 more throws in the frame and the score for the frame is the sum of these 3 values.
6. If a spare in the tenth frame, throw 1 more throw and the score for the frame is the sum of these 3 values.
7. In all other cases a frame will have 2 throws, and the score for the frame is the sum of these 2 throws.
8. There are 10 frames in a game, and the final score is the sum of the scores for each frame.

## Design

### Challenges

1. The score for a frame may depend on throws which are part of the following frame
2. The tenth frame is treated differently from the previous 9 (in particular if there is a strike or spare in that frame).
3. The symbol for a spare is a '/', but the value of that throw must be such that it and the previous throw sum to ten.
4. Frames may have a variable number of throws in them.

Challenge 4 makes it harder to fold over frames.
Challenge 1 makes it harder to fold over individual chars in the inputs string.

NB: "Folding" is functional programming terminology and corresponds to C#'s Enumerable.Aggregate extension method.

### Ideas

1. To address challenge 1, track start and end throws for scoring purposes.
2. To address challenge 2, I could track which throws are part of a frame. But is this essential?
3. To address challenge 3, I could first transform the array of chars into an array of numbers.
4. Perhaps challenge 4 could be solved by first transforming throws into frames, then folding over the frames.
5. Another option is to fold over the throws, but keep track of the current frame and incomplete past frames (i.e. where score is dependent on future throws).

### Solution options

#### Option 1: Pipeline

Symbol string => array of chars => array of int (throw values) => array of frames (with start and end indices of scoring throws) => Array of scores (by frame) => Sum

#### Option 2: Fold over throws, keeping track of current frame and incomplete past frames

This is idea 5 above. It starts out like the pipeline, but doesn't transform into frames.

#### Option 3: Keep an array of ten frames and update frames as each throw is processed

Pros:
1. Good for future-proofing as the frames array can be returned to the presentation layer for display (even partway through a game).
2. Debugging could be simpler, because we could inspect the data on a frame by frame basis.

Cons:
1. Violates KISS & YAGNI, inventing a requirement.
2. Unit tests could become less readable due to needing to set up dummy frames, even if it's only the last frame being tested.

I'm very loath to invent a spurious future requirement (being able to return partially or fully completed frames instead of just a total score).
Furthermore, there is some logic outside our program, because something/someone has already calculated the spares and strikes.
So we would be encroaching on an outside responsibility.

#### Option 4: One pass through the array of characters. Then one pass through a set of frames to calculate their scores.

To calculate spares and strikes, we need to know the values of individual throws.
(Is this really true? For a strike, we could just look ahead for a spare and add 10 if found, else add the 2 individual throws.)

We need to do a pass through the array of characters to calculate the values for each throw.
We could calculate frames at the same time.
Actually all we need to know for each frame, is the range of indexes which will add values.

### Implementation

1. Trial implementation - discovered that I have to know where a frame begins (and the frame number), as an ending substring of "X33" could be the 10th or the 9th and 10th frames.
2. Set up [unit tests for the 10th frame issues](https://github.com/AndrewTweddle/CodingExercises/blob/master/Katas/src/test/csharp/AndrewTweddle.Katas.Test.Bowling/WhenScoringTheFinalFrame.cs) identified earlier, as well as the 'X33' pattern.
3. [Implementation](https://github.com/AndrewTweddle/CodingExercises/blob/master/Katas/src/main/csharp/AndrewTweddle.Katas.Bowling/BowlingScorer.cs)
4. Further unit tests.


## Compare with other solutions

### Compare with the codernub solution

See [codernub's solution](http://codernub.blogspot.ca/2010/01/c-bowling-kata-as-tdd-exercise.html).

His solution is far more readable than mine.
Admittedly he is solving a simpler problem as he is working from the integer throws directly,
not from a string with 'X' for a strike, '/' for a spare, '-' for a gutter ball.
But I could have done 2 passes through the code - converting the string to an array of int's in the first pass.

I don't like creating stateful classes like he has done i.e. with currentRoll and score member variables.
But I have to admit that it makes the code much more elegant.

Why don't I like using member variables where parameters would do?

Firstly, I have a fear that it will be too easy to corrupt the state, creating hard to find bugs.
In this case it could happen by accidentally calling CalculateScore() multiple times.
But because the code is simple and readable, it's much easier to check that this isn't happening.

Secondly, if the class becomes too big, or if too many member variables are added,
then the behaviour starts to become opaque... the member variables start sharing the drawbacks of global variables.
On the other hand, if the class becomes this big without being broken down further, then it's a sign of another problem.
Making the class stateful doesn't scale with complexity. But it's not meant to.
The complexity should be controlled in other ways.
Can you trust your present and past colleagues to do this?

Thirdly, I like to make dependencies explicit.
If there are no global variables or member variables to depend upon,
then you know that the only external dependencies of a method are on its parameters. 

However my Calculate() method creates something like 7 local variables.
And because a lot of the code updates frameIndex and startScoreIndex, 
it becomes harder to extract helper methods (they would need to box the integer
variables using a ref argument, which feels like a code smell to me).

So my code is stateful anyway, and in a way that makes extracting readable smaller methods hard.
Sure, the statefulness is hidden inside the method, and it's theoretically quite nice that 
the variable's distance from the code that uses it is less. But still - yuck!

### Challenge (should I choose to accept it)

Do a revised bowling solution that is not stateful at the class level, nor has a large "monolithic" method.

I could do this recursively. Pass in the current frame number and the remaining throws (as an array of integers and the index of the next throw).

This would be a bit cleaner in a functional programming language, as the remaining throws could be a linked list instead.
And pattern matching might make the code more succinct.

It would probably also be easier in a language like C where pointer arithmetic is more natural.
In C# it's possible, but not idiomatic. You also need to remember things like using the "fixed" keyword 
to ensure that .Net doesn't move the item you're pointing to during a garbage collection.
I'd prefer to avoid low-level language details like that, as they will distract from the intention of the code.
