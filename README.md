# Advent of Code 2021 in Rust

this repository is a mildly disorganized archive of my [Rust](https://rust-lang.org)y solutions to [Advent of Code 2021](https://adventofcode.com/2021). the solutions which i could be bothered to maintain are in the crate workspace (folders named `d*`). those which i could not be bothered to maintain are in `old/`, and are probably still functional but you can't run them if you clone this repo (unless you take them out and fix them up yourself).

## directory structure

inside a single day's solution usually looks like this:

```
d*
├── Cargo.toml    project file
└── src
   ├── bin
   │  ├── p1.rs   part one solution
   │  └── p2.rs   part two solution
   └── lib.rs     definition of common structures and functions
```

for solutions where parts 1 and 2 are very similar, or where a solution to part 2 could also solve part 1, there is usually only a single `main.rs` file instead of a `bin/` directory.

## running the solutions

to run the solution for, for example, day 20 part 2:
```
$ cd d20
$ cat ../input/20.txt | cargo run --bin p2
```
