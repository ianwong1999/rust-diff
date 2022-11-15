# rust-diff

To learn rust for fun, I tried to implement a unix diff command. It only supports the default `diff file1 file2`.

It used the Myers difference algorithm[1] to find the minimum edit script in O(ND) where D is the number of lines of differences.

# Reference

[1] Myers, E. W. (1986). AnO (ND) difference algorithm and its variations. Algorithmica, 1(1), 251-266.
