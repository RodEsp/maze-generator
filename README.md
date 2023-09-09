# maze-generator
A maze generator that creates a maze as a player moves through it. 

It works by semi-randomly deciding which neighbors should be connected to the cell that the player moves to. In other words, as a player moves to a new tile in the maze, the generator fills in the walls of that tile. It ensures that there is always at least one way to exit the tile so that the player does not get stuck.

The starting location of the player is randomly chosen from the available tiles at the start of the program. An exit, shown as a green circle when no more than 3 tiles away, is also chosen randomly at the same time.

The generator works with any given number of columns and rows, which can be editted at the top of [main.rs](./src/main.rs) with the `NUM_OF_ROWS` and `NUM_OF_COLS` constants.

# Running the generator

1. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.
1. Clone this repository.
1. Run `cargo run` at its root.

# Playing the game

You can move your player with the arrow keys.

The only objective right now is to find the exit. It will be visible in the maze when the player is at a distance of 3 tiles or less from it.

![maze](https://github.com/RodEsp/maze-generator/assets/1084688/128e838b-cba5-45aa-aa37-ed2c6d7ebc96)
