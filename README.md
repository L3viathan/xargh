# xargh

`xargh` is a tiny `xargs` clone with a small subset of the feature set of `xargs`.


## Why?

1. I can never remember how to use xargs, especially when replacing parts of
   the command line with the incoming lines.
2. Yet another small Rust project to learn it better.


The input is always taken on stdin, and each element is in a seperate line.
No 0-byte or whitespace splitting.
