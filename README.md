# Advent of Code 2017 (Rust)

Ramblings and thoughts about each problem/solution are given below

## Day 1: Inverse Captcha
This solution performs pretty well. My laptop (i5 dual-core 2.60GHz + 16GB RAM) can sum 1 million random digits in ~3ms and benchmarks show O(n) growth as input length increases (run `cargo bench` to see!).

Just a note -- the command line interface takes a string input, so summing up a number in a file (e.g. the input from the AoC website) requires something like this:

`./1_inverse_captcha $(cat input.txt)`

## Day 2: Corruption Checksum
Not much to say about this one. Input parsing is pretty fragile, but everything seems okay outside of that. Complexity is O(nm) for an nxm input, and I can't think of any way to really improve this.  