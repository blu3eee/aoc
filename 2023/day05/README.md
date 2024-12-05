To view the problem, please visit

[Advent Of Code - Day 05](https://adventofcode.com/2023/day/5)

### Solution

`Rust`

#### Part 1:

Part 1 solution was pretty straightforward, after normalizing the data to a vec tor of seeds `Vec<u64>` and `layers_src_to_dest` for the layers from source `seed` to the final `location` `Vec<Vec<(u64, u64, u64)>>`, each item in `layers_src_to_dest` is a layer of source to destination `(source, destiation, range)`. For part 1 solution, we can just take each seed, go through each layer, use the source to find the destination to the next layer until we reach the last layer, which has the destination of the final `location` we need to find. Then compare each of the `location` to find the closest one.

#### Part 2:

In Part 2, we have a more complex situation compared to Part 1. We need to process a range of starting points (`seeds`) and figure out where they end up after going through multiple transformation layers. To make this efficient, we use a technique called **memoization**.

##### Basic Idea:

I used memoization technique to memoize the paths, for each path, we need to find the shortest (lowest) range we can memoize, can we add that memoized path to the corresponding layer.

- **Seed Ranges:** We're given a range of starting points. Each seed in this range will go through several layers of transformations.
- **Layers of Transformations:** Each layer will route us from the soruce to the destination based on certain rules (source to destination mappings).

##### Process:

- **Go Through Each Seed:** For every seed in our range, we see how it moves through each layer of transformation.

- **Check for Memoized (Saved) Paths:** Before moving a seed through a layer, we check if we already know where it will end up (because we calculated this path before). If we have this information saved (memoized), we use it to skip some steps.

- **Memoization:** Whenever we move a seed through a layer for the first time, we save (memoize) this information. This way, if we come across the same situation later, we can just use the saved information instead of calculating it again.

- **Find the Closest Location:** As we move each seed through the layers, we keep track of their final positions. Our goal is to find the closest ending position among all seeds.

**By using memoization, we efficiently find the closest ending location for our range of seeds. This method is much faster than checking every single seed through all the layers without any shortcuts.**
