# Day 7

An acyclic undirected graph is a pretty good representation for a file system.
This is hard to do in safe Rust since it would involve the both the child and the parent referring to each other.
I would've liked to use the id_tree crate for this.

The current solution is kind of hacky - it requires 2 passes and uses a lot of code to do so.

I didn't account for the fact that the traversal in the input and example was always depth first, alphabetically.
This allows for a much more simple solution.