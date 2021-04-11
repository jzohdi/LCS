# Problem
Given a large number of binary files, write a orogram that finds the longest strand of bytes that is identical between two or more files.

Output (Display):
- the length of the strand
- the file names that include the strand
- the offest where the strand appears

## Algorithm Steps
Input: list of filenames:

1. Create a list of all possible unique pairs of files
2. Starting with the first pairs calculate the longest common subbytes 
3. Continue to the next pair, but now start with the previously discovered length
4. For each successive pair, update the max length so far (and the bytes, index of the pair found, offset in each file), and use the longest length so far as search starting point
5. Finally check all files except the two where longest common bytes were found for this sequence.

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
At the root of the directory (`LCS/`) call `cargo run`, to build and execute the project.

## Notes
First algorithm implemented to find longest sub bytes of two files used a O(nm) solutions using dynamic programming. The runtime for this was ok, but I think it could be improved. The second algorithm implemented was creating a suffix tree, but a couple problems arrised from this. The first problem is that the naive suffix tree creating is O(n^2), second is that the tree would be too big for memory, and does not benefit from L1 cache. This second algorithm performs abysmally. 