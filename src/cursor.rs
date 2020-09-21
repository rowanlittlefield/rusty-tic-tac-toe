use crate::user_input::UserInput;

pub struct Cursor {
  board_dimensions: (usize, usize),
  pub coordinates: (usize, usize),
}

impl Cursor {
  pub fn new(board_dimensions: (usize, usize)) -> Cursor {
    Cursor {
      board_dimensions,
      coordinates: (0, 0),
    }
  }

  pub fn move_cursor(&mut self, user_input: UserInput) {
    match user_input {
      UserInput::UP => {
        let is_zero = self.coordinates.0 == 0;
        self.coordinates.0 = if is_zero { self.board_dimensions.0 - 1} else { self.coordinates.0 - 1 };
      },
      UserInput::RIGHT => {
        self.coordinates.1 = (self.coordinates.1 + 1) % self.board_dimensions.1;
      },
      UserInput::DOWN => {
        self.coordinates.0 = (self.coordinates.0 + 1) % self.board_dimensions.0;
      },
      UserInput::LEFT => {
        let is_zero = self.coordinates.1 == 0;
        self.coordinates.1 = if is_zero { self.board_dimensions.1 - 1} else { self.coordinates.1 - 1 };
      },
      _ => { panic!("Invalid UserInput for move_cursor"); },
    }
  }
}