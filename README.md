# rusty-tic-tac-toe

A command line application written in Rust for playing Tic-Tac-Toe.

## Screenshot

![Screen Shot 2020-10-10 at 1 59 42 PM](https://user-images.githubusercontent.com/30376211/95661877-e54f0a00-0b00-11eb-9524-4ae50933be81.png)

## How to Run

1. [Clone this repo.](https://git-scm.com/book/en/v2/Git-Basics-Getting-a-Git-Repository#_git_cloning)

2. [Install Rust and Cargo.](https://doc.rust-lang.org/book/ch01-01-installation.html)

3. Navigate to the cloned project directory in your command line. The game can be run from this directory using `cargo run`. The unit tests for the project can be executed using `cargo test`. To compile the project to an executable, run `cargo build --release`. The executable can be found in `./target/release`. On macOS, the executable will have the name `tic_tac_toe`, which you can run from your command line in the project root directory via `./target/release/tic_tac_toe`.

## Functionality & MVP

### Controls
The control scheme is described below. Type any of the following characters followed by the enter/return key to send input to the application.

* `w` - Move board cursor up by one space.
* `d` - Move board cursor right by one space.
* `s` - Move board cursor down by one space.
* `a` - Move board cursor left by one space.
* Press enter with no preceding key - Set the space currently occupied by the board cursor with the marker (X or O) associated with the current player. If the selected space is already occupied, then nothing will happen.
* `u` - Undo the previous turn. This will decrement the displayed turn number, remove the marker set in the previous turn, move the cursor to the previously set space, and toggle the current user. This command does nothing on turn 1.
* `r` - Redo an undone turn. Reverses all changes from the last undo command. *Note:* The redo cache will be cleared if you undo a command, then set a new space prior to redo.

### Game Completion
The logic for checking for game completion (see the `game_over` and `get_winner` methods on the `Board` struct in `./src/board.rs`) uses a naive algorithm that iterates over each row, column and diagonal twice (once for the X marker and once for the O marker) to check for three of the same marker in a row, running in O(n<sup>2</sup>) time complexity (where n is the length of a row/column). There are several alternate approaches that have reduced time complexity (at the cost of space complexity), but I did not implement them as the performance is sufficient for the current case (always a 3x3 board) and optimizing the scalability of this logic is not the primary focus of this project.

### History Feature
The history feature (undo and redo commands) utilize the well-known [memento pattern](https://en.wikipedia.org/wiki/Memento_pattern), where the `Board` struct (see `./src/board.rs`) acts as the *originator*, the `BoardMemento` struct (see `./src/board_memento.rs`) acts as the *memento*, and the `History` struct (see `./src/history.rs`) acts as the *caretaker*.
