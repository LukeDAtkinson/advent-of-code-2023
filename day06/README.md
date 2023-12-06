# Advent of Code 2023 Day 06

You don't need to write any code to solve day 06.
It can be solved analytically

Let $T$ be the time available in the race.
Let $R$ be the record for the distance covered.
Let $x$ be the time you hold the button.

The velocity of the boat is $x$.

The boat moves for time $T - x$.

Therefore the distance covered by the boat is given by
$$x(T-x) = Tx - x^2.$$

We are looking for any values of x such that this value is greater than the record:
$$Tx - x^2 > R.$$

Rearranging this we get:
$$-x^2 + Tx - R > 0.$$

We can find the roots of this quadratic equation with:
$$x = {-T \pm \sqrt{T^2-4R} \over -2}.$$
$$= {T \mp \sqrt{T^2-4R} \over 2}.$$

Therefore we simply need to find how many integers satisfy
$$\lfloor{T - \sqrt{T^2-4R} \over 2}\rfloor < x <
\lceil{T + \sqrt{T^2-4R} \over 2}\rceil$$
which is just equal to
$$\lceil{T + \sqrt{T^2-4R} \over 2}\rceil - \lfloor{T - \sqrt{T^2-4R} \over 2}\rfloor.$$

At this point it is trivial to plug the numbers into a calculator (with enough precision)
and get your result.
