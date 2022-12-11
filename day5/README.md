# Day 5

Parsing the stacks was really annoying with nom, would probably have been easier with some normal inperative code.

The stack handling could probably have been more efficient, but I'm not good enough at Rust lifetimes.

The main.rs files only contains the code for running the program, the solutions and tests are now in problem.rs.
The problem is also split up into submodules instead of sibling modules.