use crate::board::Board;
use crate::board_memento::BoardMemento;

pub struct History {
  past_mementos: Vec<BoardMemento>,
  future_mementos: Vec<BoardMemento>,
}

impl History {
  pub fn new() -> History {
    History {
      past_mementos: vec!(),
      future_mementos: vec!(),
    }
  }

  pub fn push(&mut self, board_memento: BoardMemento) {
    let is_successful_set_space_memento = board_memento.has_set_space();
    if !is_successful_set_space_memento {
      panic!("Only push successful set space mementos!");
    }
    
    self.future_mementos.clear();
    self.past_mementos.push(board_memento);
  }

  pub fn back(&mut self, board: &mut Board) {
    let board_memento = self.past_mementos.pop();
    match &board_memento {
      Some(BoardMemento::SetSpace(_)) => {
        let unwrapped_memento = board_memento.unwrap();
        board.revert_set_space(&unwrapped_memento);
        self.future_mementos.push(unwrapped_memento);
      },
      _ => {},
    }
  }

  pub fn forward(&mut self, board: &mut Board) {
    let board_memento = self.future_mementos.pop();
    match &board_memento {
      Some(BoardMemento::SetSpace(_)) => {
        let unwrapped_memento = board_memento.unwrap();
        board.redo_set_space(&unwrapped_memento);
        self.past_mementos.push(unwrapped_memento);
      },
      _ => {},
    }
  }
}