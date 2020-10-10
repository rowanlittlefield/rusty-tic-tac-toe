# rusty-tic-tac-toe

A command line application written in Rust for playing Tic-Tac-Toe.

## Screenshot

(Insert Screenshot)

## How to Run

1. [Clone this repo.](https://git-scm.com/book/en/v2/Git-Basics-Getting-a-Git-Repository#_git_cloning)

2. [Install Rust and Cargo.](https://doc.rust-lang.org/book/ch01-01-installation.html)

3. Navigate to the cloned project directory in your command line. The game can be run from this directory using `cargo run`. The unit tests for the project can be executed using `cargo test`. To compile the project to an executable, run `cargo build --release`. The executable can be found in `./target/release`. On Mac OS, the executable will be `tic_tac_toe`, which you can run from your command line in the project root directory via `./target/release/tic_tac_toe`.

## Functionality & MVP

The control scheme is described below. Type any of the following characters followed by the enter/return key to send input to the application.

* `w` - Move board cursor up by one space.
* `d` - Move board cursor right by one space.
* `s` - Move board cursor down by one space.
* `a` - Move board cursor left by one space.
* Press enter with no preceding key - Set the space currently occupied by the board cursor with the marker (X or O) associated with the current player. If the selected space is already occupied, then nothing will happen.
* `u` - Undo the previous turn. This will decrement the displayed turn number, remove the marker set in the previous turn, move the cursor to the previously set space, and toggle the current user. This command does nothing on turn 1.
* `r` - Redo an undone turn. Reverses all changes from the last undo command. *Note:* The redo cache will be cleared if you undo a command, then set a new space prior to redo.
