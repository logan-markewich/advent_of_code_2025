# Advent of Code 2025

This year, I'm finally going to tackle advent of code all in rust.

I'm fairly new to rust, so not every solution is going to be ideal as I figure out syntax and patterns.

## Log

### Day 1

I got really tripped up on part two, but I think I eventually got a decent-ish (although confusing) solution.

### Day 2

This one felt a lot easier. Was able to figure out both parts pretty quickly.

I also refactored the repo to have a module per-day, and have the top-level CLI select which day to run and forward args. I might think of a better organization later, we will see.

### Day 3

This one was fun! While the brute force approach is just to compare eveything to everything, it was fun to think about how to optimize this into a single pass.

My solution for part 1 was super hacky, and part 2 forced me to generalize the solution into something cleaner.

### Day 4

Honestly the easiest one so far. There's probably a better way to do this beyond just iterating over a 2D grid (maybe something smarter with adjecnency matrices?) but this was quick and easy enough.
