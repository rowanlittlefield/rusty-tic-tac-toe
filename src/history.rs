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

  pub fn number_of_elapsed_turns(&self) -> usize {
    self.past_mementos.len()
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

#[cfg(test)]
mod tests {
  use super::*;
  use crate::space::Space;
  use crate::board_memento::SetSpaceMemento;

  fn create_set_space_board_memento(cursor_coordinates: (usize, usize)) -> BoardMemento {
    let has_set_space = true;
    let space = Space::X;
    let set_space_memento = SetSpaceMemento::new(cursor_coordinates, has_set_space, space);
    BoardMemento::SetSpace(set_space_memento)
  }

  #[test]
  fn number_of_elapsed_turns_new_history() {
    let history = History::new();
    let expected = 0;

    let actual = history.number_of_elapsed_turns();

    assert_eq!(expected, actual);
  }

  #[test]
  fn number_of_elapsed_turns_push() {
    let mut history = History::new();
    let cursor_coordinates = (0, 0);
    let board_memento = create_set_space_board_memento(cursor_coordinates);
    let expected = 1;

    history.push(board_memento);
    let actual = history.number_of_elapsed_turns();

    assert_eq!(expected, actual);
  }

  #[test]
  fn number_of_elapsed_turns_back_and_push() {
    let mut board = Board::new();
    let mut history = History::new();
    let cursor_coordinates_1 = (0, 0);
    let board_memento_1 = create_set_space_board_memento(cursor_coordinates_1);
    let cursor_coordinates_2 = (1, 0);
    let board_memento_2 = create_set_space_board_memento(cursor_coordinates_2);
    let expected = 1;

    history.push(board_memento_1);
    history.back(&mut board);
    history.push(board_memento_2);
    let actual = history.number_of_elapsed_turns();

    assert_eq!(expected, actual);
  }

  #[test]
  fn back_should_enable_setting_board_space_again() {
    let mut board = Board::new();
    let mut history = History::new();
    let expected = true;

    let board_memento = board.set_current_space(Space::X);
    history.push(board_memento);
    history.back(&mut board);
    let board_memento = board.set_current_space(Space::X);
    let actual = board_memento.turn_over();

    assert_eq!(expected, actual);
  }

  #[test]
  fn back_should_return_correct_memento_scenario_1() {
    let mut board = Board::new();
    let mut history = History::new();
    let expected = true;

    let board_memento = board.set_current_space(Space::X);
    history.push(board_memento);
    let board_memento = history.back(&mut board);
    let actual = match board_memento {
      BoardMemento::RevertSetSpace => true,
      _ => false,
    };

    assert_eq!(expected, actual);
  }

  #[test]
  fn back_should_return_correct_memento_scenario_2() {
    let mut board = Board::new();
    let mut history = History::new();
    let expected = true;

    let board_memento = history.back(&mut board);
    let actual = match board_memento {
      BoardMemento::NullBoardMemento => true,
      _ => false,
    };

    assert_eq!(expected, actual);
  }

  #[test]
  fn forward_should_disable_setting_board_space_again() {
    let mut board = Board::new();
    let mut history = History::new();
    let expected = false;

    let board_memento = board.set_current_space(Space::X);
    history.push(board_memento);
    history.back(&mut board);
    history.forward(&mut board);
    let board_memento = board.set_current_space(Space::X);
    let actual = board_memento.turn_over();

    assert_eq!(expected, actual);
    }

  #[test]
  fn forward_should_return_correct_memento_scenario_1() {
    let mut board = Board::new();
    let mut history = History::new();
    let expected = true;

    let board_memento = board.set_current_space(Space::X);
    history.push(board_memento);
    history.back(&mut board);
    let board_memento = history.forward(&mut board);
    let actual = match board_memento {
      BoardMemento::RedoSetSpace => true,
      _ => false,
    };

    assert_eq!(expected, actual);
  }

  #[test]
  fn forward_should_return_correct_memento_scenario_2() {
    let mut board = Board::new();
    let mut history = History::new();
    let expected = true;

    let board_memento = history.forward(&mut board);
    let actual = match board_memento {
      BoardMemento::NullBoardMemento => true,
      _ => false,
    };

    assert_eq!(expected, actual);
  }
}
