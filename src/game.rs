use crate::space::Space;
use crate::board::Board;
use crate::controller;
use crate::user_input::UserInput;

pub struct Game {
  board: Board,
  current_player: Space,
}

impl Game {
  pub fn new() -> Game {
    Game {
      board: Board::new(),
      current_player: Space::X,
    }
  }

  pub fn play(&mut self) {
    while !self.board.game_over() {
      self.play_turn();
    }

    self.render_game_over_message();
  }

  fn play_turn(&mut self) {
    let mut is_turn_over = false;
    while !is_turn_over {
      is_turn_over = self.play_tick();
    }

    self.current_player = match self.current_player {
      Space::X => Space::O,
      Space::O => Space::X,
      Space::Empty => panic!("Invalid player!"),
    };
  }

  fn play_tick(&mut self) -> bool {
      clear_terminal();
      self.board.render();
      println!("{}'s turn", &self.current_player.as_str());
    
      let user_input = controller::get_user_input();
      match user_input {
        UserInput::ENTER => {
          let has_set_space = self.board.set_current_space(self.current_player);
          has_set_space
        },
        _ => {
          self.board.move_cursor(user_input);
          false
        },
      }
  }

  fn render_game_over_message(&self) {
    clear_terminal();
    self.board.render();
    println!("Game Over!");
    let winner = self.board.get_winner();

    match winner {
      Some(winner) => println!("Player {} wins!", winner.as_str()),
      None => println!("Cat's game!"),
    };
  }
}

fn clear_terminal() {
  print!("\x1B[2J\x1B[1;1H");
}
