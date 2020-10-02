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
    let is_turn_over_memento = board_memento.turn_over();
    if !is_turn_over_memento {
      panic!("Only push mementos that correspond to the end of a turn!");
    }
    
    self.future_mementos.clear();
    self.past_mementos.push(board_memento);
  }

  pub fn back(&mut self, board: &mut Board) -> BoardMemento {
    let board_memento = self.past_mementos.pop();
    match board_memento {
      Some(BoardMemento::SetSpace(_)) => {
        let unwrapped_memento = board_memento.unwrap();
        let revert_memento = board.revert_set_space(&unwrapped_memento);
        self.future_mementos.push(unwrapped_memento);
        revert_memento
      },
      _ => BoardMemento::NullBoardMemento,
    }
  }

  pub fn forward(&mut self, board: &mut Board) -> BoardMemento {
    let board_memento = self.future_mementos.pop();
    match board_memento {
      Some(BoardMemento::SetSpace(_)) => {
        let unwrapped_memento = board_memento.unwrap();
        let redo_memento = board.redo_set_space(&unwrapped_memento);
        self.past_mementos.push(unwrapped_memento);
        redo_memento
      },
      _ => BoardMemento::NullBoardMemento,
    }
  }
}
