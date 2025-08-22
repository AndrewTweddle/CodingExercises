#set document(
    title: [Problem 71: Ordered Fractions],
    author: "Andrew Tweddle",
    keywords: "Project Euler",
    date: datetime(year: 2025, month: 08, day: 21)
)
#set math.equation(numbering: "(1)")
#show link: underline

= Problem 71: Ordered Fractions

#align(center)[Andrew Tweddle \ 2025-08-21]

== The problem statement

See #link("Project Euler Problem 71")[https://projecteuler.net/problem=71].

== My solution

Consider a proper reduced fraction $n/d < 3/7$ with $d <= 1000000$.

Of all such fractions, the closest to $3/7$ will minimize:

$ 3/7 - n/d = (3d - 7n)/(7d) $

To find this number, we will minimize its numerator and maximize its denominator.

The smallest possible numerator is 1, so set $ 3d - 7n = 1 $ <min_numerator>

Then:

    $ n &= (3d - 1)/7 \
        &= (3(d+2) - 7)/7 \
        &= (3(d + 2))/7 - 1 $

For $n$ to be an integer, this requires that $d + 2$ is divisible by $7$.

A unit change in the numerator is more harmful than a unit change in the denominator.
So maximize the denominator *subject to* the above condition.

To maximize the denominator, $7d$, choose the largest value of $d <= 1000000$ such that $7 divides (d + 2)$.
This is $d + 2 = 999999 = 7 times 142857$.

This gives:

    $ d &= 999997 $

and our answer:

    $ n &= (3(d + 2))/7 - 1 \
        &= 3 times 142857 - 1 \
        &= 428570
    $

We can also be sure that $n/d$ is a reduced fraction. How?

Well, recall from @min_numerator that $3d - 7n = 1$.
This means that $gcd(n, d) = 1$, since any divisor of both $d$ and $n$ also divides $3d - 7n$ and hence must be a divisor of $1$.
But the only positive integer that divides $1$ is $1$ itself.
