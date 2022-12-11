# Day 10

I misunderstood the problem for quite some time, not realizing that the addx didn't block the cpu until the increment was performed.

The Register struct is quite neat, but I probably didn't need such elaborate management of before/after state, when the before state was the one being asked for in the problem.

Generating an iterator of states and chaining operations on it felt really nice in the end.

Part 2 was a gimme, and pretty cool for those who don't need screen readers.