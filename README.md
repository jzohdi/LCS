# Core Software Engineering Challenge
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

