# Based on [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)

## Rules
1. Any live cell with two or three live neighbours survives.
2. Any dead cell with three live neighbours becomes a live cell.
3. All other live cells die in the next generation. Similarly, all other dead cells stay dead.
4. In this version, the board is infinite in size and wraps around. (Torus)

## How to run
Clone the repo
```
cargo run
```
