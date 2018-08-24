Chessrw
========

A rust library for reading and writing PGN files and an utility to read a PGN file, filter it and write to another file, with the option to remove some details (comments, variations, tags).

Usage of the utility
---------------------
```
    chessrw [FLAGS] [OPTIONS] <INPUT> [OUTPUT]

FLAGS:
        --blackwins
        --draw
    -h, --help            Prints help information
        --nocomments
        --noprogress      No progress bar is showed (faster).
        --notags
        --novariations
        --onlymoves       Write only moves (alias for --nocomments --novariations --notags).
    -V, --version         Prints version information
        --whitewins

OPTIONS:
        --minplycount <minplycount>
        --players <players>            A comma separated list of players. Put an * as first character to get only games
                                       between players. Put a +, - or = as first character of a player to get only wins,
                                       loses or draws for that player.

ARGS:
    <INPUT>     Sets the input file to use
    <OUTPUT>    Sets the output file to use
```
