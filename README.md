# Problem
Given a large number of binary files, write a orogram that finds the longest strand of bytes that is identical between two or more files.

Output (Display):
- the length of the strand
- the file names that include the strand
- the offest where the strand appears

## Algorithm Steps
Input: list of filenames:

1. Loop through each file, (i in 0..n) and compare to all rest (j in i+1..n)
2. Update max at each comparison if a longer byte sequence is found. This computes the longest substring that is present between all unique pairs of files. This is the minimum requirement for the stand.
3. Once we know the longest strand between at least two files, finally check all files except the two where longest common bytes were found for this sequence.
4. Each comparison can be made in parallel using threads as long as the check and update of the max found so far is done atomically.

## Running
I used rust to implement this solution and therefore cargo/rustc is required to build.
This repository should be cloned, and all of the testing files to be tested should be placed in the root of the directory as neighbors with `src/`. 

Example:
```shell
LCS/
|  src/
|  |  main.rs
|  |  ...
|  README.md
|  Cargo.lock
|  Cargo.toml
|  sample.1
|  sample.2
|  ... 
```
At the root of the directory (`LCS/`) call `cargo run --release`, to build and execute the project.
Note: `--release` is important for making executable optimizations that result in quick runtime.

## Notes
First algorithm implemented to find longest sub bytes of two files used a O(nm) solutions using dynamic programming. The runtime for this was ok, but I think it could be improved. The second algorithm implemented was creating a suffix tree, but a couple problems arrised from this. The first problem is that the naive suffix tree creating is O(n^2), second is that the tree would be too big for memory, and does not benefit from L1 cache. This second algorithm performs abysmally. 
