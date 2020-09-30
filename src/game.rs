use crate::board::Board;
use crate::board_memento::BoardMemento;
use crate::controller;
use crate::history::History;
use crate::user_input::UserInput;
use crate::space::Space;

pub struct Game {
  board: Board,
  current_player: Space,
  history: History,
}

impl Game {
  pub fn new() -> Game {
    Game {
      board: Board::new(),
      current_player: Space::X,
      history: History::new(),
    }
  }

  pub fn play(&mut self) {
    while !self.board.game_over() {
      self.play_turn();
    }

    self.render_game_over_message();
  }

  fn play_turn(&mut self) {
    let mut next_history_item = BoardMemento::NullBoardMemento;
    while !next_history_item.has_set_space() {
      next_history_item = self.play_tick();
    }

    self.history.push(next_history_item);

    self.current_player = match self.current_player {
      Space::X => Space::O,
      Space::O => Space::X,
      Space::Empty => panic!("Invalid player!"),
    };
  }

  fn play_tick(&mut self) -> BoardMemento {
      clear_terminal();
      self.board.render();
      println!("{}'s turn", &self.current_player.as_str());
    
      let user_input = controller::get_user_input();
      match user_input {
        UserInput::ENTER => self.board.set_current_space(self.current_player),
        UserInput::UNDO => {
          self.history.back(&mut self.board);
          BoardMemento::NullBoardMemento
        },
        UserInput::REDO => {
          self.history.forward(&mut self.board);
          BoardMemento::NullBoardMemento
        },
        _ => self.board.move_cursor(user_input),
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
