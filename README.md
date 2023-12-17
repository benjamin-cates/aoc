# Advent of Code 2023
Here is a collection of my [Advent of Code](https://adventofcode.com/2023) submissions for the year 2023.
All of my solutions are written entirely in Rust and are my own work with no hints.

## Personal times
```
      ----Part 1------     -----Part 2-----
Day       Time    Rank         Time   Rank      Start Time
 17   01:40:48    2343     02:15:59   2407      00:30
 16   00:27:31    1358     00:33:20   1191      00:00
 15   00:07:20    2449     00:31:17   2315      00:00
 14   00:08:32     829     00:48:55   1732      00:00
 13   01:38:38    6401     01:45:48   4673      00:00
 12   01:51:28    6588     04:04:54   4039      00:00
 11   16:43:41   35983     16:56:04  33954      16:28
 10   00:45:29    2840     01:29:02   1516      00:00
  9   02:13:52   12488     02:21:28  12147      01:53
  8   00:11:01    2697     01:14:06   5930      00:00
  7   00:48:16    5283     01:49:30   7686      00:00
  6   01:12:13   13564     01:19:00  13019      00:45
  5   00:19:24    1232     01:31:07   2992      00:00
  4   01:42:02   18398     01:53:21  14031      01:26
  3   14:12:01   60208     14:29:37  50082      13:55
  2   16:34:44   85233     16:40:06  80910      15:58
  1   13:17:42  104479     14:00:19  71691      12:48
```

## Structure
For the file layout, library files are contained in `src` and each day and its corresponding input is stored in `src/bin`. Each day is a two digit number representing the date plus the `.rs` extension. Each input is the same but with the `.txt` extension.

## Compiling
Ensure you have the Rust toolchain installed for your computer (visit [rustup.rs](https://rustup.rs) for more info).
Each day is its own binary application, using the date number to represent each day,
run either `cargo test --bin 13`to test day 13 for example. Or `cargo run --bin 13`
to run day 13 for example.



