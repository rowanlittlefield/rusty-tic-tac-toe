use crate::space::Space;
use crate::user_input::UserInput;

pub enum BoardMemento {
  SetSpace(SetSpaceMemento),
  MoveCursor(UserInput),
  NullBoardMemento,
}

impl BoardMemento {
  pub fn turn_over(&self) -> bool {
    match self {
      BoardMemento::SetSpace(set_space_memento) => set_space_memento.has_set_space(),
      _ => false
    }
  }
}

pub struct SetSpaceMemento {
  cursor_coordinates: (usize, usize),
  has_set_space: bool,
  space: Space,
}

impl SetSpaceMemento {
  pub fn new(cursor_coordinates: (usize, usize), has_set_space: bool, space: Space) -> SetSpaceMemento {
    SetSpaceMemento {
      cursor_coordinates,
      has_set_space,
      space,
    }
  }
  
  pub fn has_set_space(&self) -> bool {
    self.has_set_space
  }

  pub fn get_coordinates(&self) -> (usize, usize) {
    self.cursor_coordinates
  }

  pub fn get_space(&self) -> Space {
    self.space
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn board_memento_turn_over_return_true() {
    let cursor_coordinates = (0, 0);
    let has_set_space = true;
    let space = Space::X;
    let set_space_memento = SetSpaceMemento::new(cursor_coordinates, has_set_space, space);
    let board_memento = BoardMemento::SetSpace(set_space_memento);
    let expected = true;

    let actual = board_memento.turn_over();

    assert_eq!(actual, expected);
  }

  #[test]
  fn board_memento_turn_over_return_false_scenario_1() {
    let cursor_coordinates = (0, 0);
    let has_set_space = false;
    let space = Space::X;
    let set_space_memento = SetSpaceMemento::new(cursor_coordinates, has_set_space, space);
    let board_memento = BoardMemento::SetSpace(set_space_memento);
    let expected = false;

    let actual = board_memento.turn_over();

    assert_eq!(actual, expected);
  }

  #[test]
  fn board_memento_turn_over_return_false_scenario_2() {
    let user_input = UserInput::DOWN;
    let board_memento = BoardMemento::MoveCursor(user_input);
    let expected = false;

    let actual = board_memento.turn_over();

    assert_eq!(actual, expected);
  }

  #[test]
  fn board_memento_turn_over_return_false_scenario_3() {
    let board_memento = BoardMemento::NullBoardMemento;
    let expected = false;

    let actual = board_memento.turn_over();

    assert_eq!(actual, expected);
  }
}
