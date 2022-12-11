# Day 2

First time trying out nom for parsing the input, would probably have been fine without it.

Using the TryFrom (and From) trait has been a pretty convinient way of parsing nested structures.

I extracted the parsing code to a separate module, which I will eventually go all in on the future.

I wanted to create types that represented the domain, but I should probably have done so for the "battle result" concept (win, loss, draw) that's currently embedded in battle.rs.