use std::io;
use std::error;
use crate::space::Space;
use crate::board::Board;

pub struct Game {
  board: Board,
  player: Space,
}

impl Game {
  pub fn new() -> Game {
    Game {
      board: Board::new(),
      player: Space::X,
    }
  }

  pub fn play(&mut self) {
    clear_terminal();
    
    while !self.board.game_over() {
      self.play_turn();
    }

    self.render_game_over_message();
  }

  fn play_turn(&mut self) {
    self.board.render();
    
    let space = &self.player;
    let current_player = space.as_str();
    println!("{}'s turn", current_player);

    let coordinates = match self.get_coordinates() {
      Ok(value) => value,
      Err(_) => {
        clear_terminal();
        println!("Invalid coordinate(s)! Try again.");
        return ();
      }
    };
    
    self.board.set_space(*space, coordinates);

    self.player = match self.player {
      Space::X => Space::O,
      Space::O => Space::X,
      Space::Empty => panic!("Invalid player!"),
    };

    clear_terminal();
  }

  fn get_coordinates(&self) -> Result<(usize, usize), Box<dyn error::Error>> {
    let row = self.get_guess("row")?;
    let col = self.get_guess("col")?;
    let coordinates = (row, col);
    let is_space_occupied = self.board.is_space_occupied(&coordinates);

    if !is_space_occupied {
      Ok(coordinates)
     } else {
       Err(From::from("Space occupied!"))
     }
  }

  fn get_guess(&self, axis: &str) -> Result<usize, Box<dyn error::Error>> {
    println!("Select {}.", axis);

    let mut coordinate = String::new();

    io::stdin()
      .read_line(&mut coordinate)
      .expect("Failed to read line");

    let coordinate = coordinate.trim()
      .parse()?;

    if coordinate < 3 { // TODO: replace this magic number with a board method
      Ok(coordinate)
    } else {
      Err(From::from("Invalid coordinate!"))
    }
  }

  fn render_game_over_message(&self) {
    self.board.render();
    println!("Game Over!");
    let winner = self.board.get_winner();

    match winner {
      Some(winner) => println!("Player {} wins!", winner.as_str()),
      _ => println!("Cat's game!"),
    };
  }
}

fn clear_terminal() {
  print!("\x1B[2J\x1B[1;1H");
}
