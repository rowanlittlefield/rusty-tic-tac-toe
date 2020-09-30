use crate::space::Space;
use crate::user_input::UserInput;

pub enum BoardMemento {
  SetSpace(SetSpaceMemento),
  MoveCursor(UserInput),
  NullBoardMemento,
}

impl BoardMemento {
  pub fn has_set_space(&self) -> bool {
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