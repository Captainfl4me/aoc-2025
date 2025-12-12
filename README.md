# 🎄 Advent of Code 2025 🎄

<img src="./img/rustacean-flat-happy.png" width="164">

Link to the website with puzzle: [Advent Of Code 2025](https://adventofcode.com/2025)

## Use template

To create a new day simply run: ```make new-day-NUMBER_OF_DAY```.

> Required [cargo-generate](https://crates.io/crates/cargo-generate)

## Use Algorithm

Algorithm|File|Small description
--|--|--
[DFS](https://en.wikipedia.org/wiki/Depth-first_search) with cached branch|[day-11](./day-11/src/main.rs)|Depth-First-Search search through graph exploring depth first and cached already explored state to optimised the search if the graph converge in one or more spots.
