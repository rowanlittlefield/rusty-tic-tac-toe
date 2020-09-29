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

  pub fn move_cursor(&mut self, user_input: &UserInput) {
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new_should_instantiate_cursor_with_correct_coordinates() {
    let board_dimensions = (2, 2);
    let cursor = Cursor::new(board_dimensions);
    let expected = (0, 0);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

  #[test]
  fn move_cursor_up() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (1, 0),
    };
    let expected = (0, 0);

    cursor.move_cursor(UserInput::UP);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

  #[test]
  fn move_cursor_up_wrap() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (0, 0),
    };
    let expected = (1, 0);

    cursor.move_cursor(UserInput::UP);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

    #[test]
  fn move_cursor_right() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (0, 0),
    };
    let expected = (0, 1);

    cursor.move_cursor(UserInput::RIGHT);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

  #[test]
  fn move_cursor_right_wrap() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (0, 1),
    };
    let expected = (0, 0);

    cursor.move_cursor(UserInput::RIGHT);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

    #[test]
  fn move_cursor_down() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (0, 0),
    };
    let expected = (1, 0);

    cursor.move_cursor(UserInput::DOWN);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

  #[test]
  fn move_cursor_down_wrap() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (1, 0),
    };
    let expected = (0, 0);

    cursor.move_cursor(UserInput::DOWN);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

    #[test]
  fn move_cursor_left() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (0, 1),
    };
    let expected = (0, 0);

    cursor.move_cursor(UserInput::LEFT);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

  #[test]
  fn move_cursor_left_wrap() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (0, 0),
    };
    let expected = (0, 1);

    cursor.move_cursor(UserInput::LEFT);

    let actual = cursor.coordinates;
    assert_eq!(actual, expected);
  }

  #[test]
  #[should_panic]
  fn move_cursor_invalid_user_input() {
    let mut cursor = Cursor {
      board_dimensions: (2, 2),
      coordinates: (0, 0),
    };

    cursor.move_cursor(UserInput::ENTER);
  } 
}
