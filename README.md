Prerequisites:
• Learn as much as possible about Rust language.

Description:
1. Convert mat.in files from task 5 in files in mat.in.x files
    The mat.in.x files are compressed versions of mat.in files.

    Example of mat.in content:
    5x10:11010111001000111110101101110100001111111100000001
    9x5:110010101011010100000001001101010111111100111
    5x3:010101100011101
    ...

    Example of mat.in.x content:
    5x10:D723EB743FC01
    9x5:CAB501357F38
    5x3:563D
    ...

    The binary string from mat.in is split into groups of four bits
each. Each group of four bits its converted to its hexadecimal equivalent.

2. Convert mat.in.x to mat.in files

Proof of successful task completion:
• Demonstrate the program's functionality by testing with large input
files that have more than 100,000 lines, each line having at least a 5x5
matrix.
   Test with large input files that have lots of duplicate lines.
   Develop a caching mechanism for these cases with a configurable
number of cache entries.
