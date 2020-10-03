use ansi_term::Colour;
use crate::board_memento::{BoardMemento, SetSpaceMemento};
use crate::cursor::Cursor;
use crate::space::Space;
use crate::user_input::UserInput;

const ROWS: [[(usize, usize);3];8] = [
    [(0, 0), (0, 1), (0, 2)],
    [(1, 0), (1, 1), (1, 2)],
    [(2, 0), (2, 1), (2, 2)],

    [(0, 0), (1, 0), (2, 0)],
    [(0, 1), (1, 1), (2, 1)],
    [(0, 2), (1, 2), (2, 2)],

    [(0, 0), (1, 1), (2, 2)],
    [(0, 2), (1, 1), (2, 0)],
];

pub struct Board {
    cursor: Cursor,
    grid: [[Space;3];3],
}

impl Board {
    pub fn new() -> Board {
        let grid = [[Space::Empty;3];3];
        let cursor_dimensions = (grid[0].len(), grid[1].len());
        let cursor = Cursor::new(cursor_dimensions);
        
        Board {
            cursor,
            grid,
        }
    }
    
    pub fn render(&self) {       
        let cursor_coordinates = self.cursor.get_coordinates();
        
        for (i, row) in self.grid.iter().enumerate() {
            for (j, space) in row.iter().enumerate() {
                let is_cursor_pos = cursor_coordinates == (i, j);
                let colored_space = match is_cursor_pos {
                    true => Colour::Black.on(Colour::Yellow).paint(space.as_str()),
                    false => Colour::White.on(Colour::Black).paint(space.as_str()),
                };

                print!("{}", colored_space);

                let is_end_of_row = j >= row.len() - 1;
                match is_end_of_row {
                    true => println!(""),
                    false => print!("{}", Colour::White.on(Colour::Black).paint("|")),
                };
            }
        }
    }

    pub fn move_cursor(&mut self, user_input: UserInput) -> BoardMemento {
        self.cursor.move_cursor(&user_input);
        BoardMemento::MoveCursor(user_input)
    }

    pub fn set_current_space(&mut self, space: Space) -> BoardMemento {
        let cursor_coordinates = self.cursor.get_coordinates();
        let can_set_space = !self.is_space_occupied(&cursor_coordinates);
        if can_set_space {
            self.set_space(space, cursor_coordinates)
        }

        let set_space_memento = SetSpaceMemento::new(cursor_coordinates, can_set_space, space);
        BoardMemento::SetSpace(set_space_memento)
    }

    fn is_space_occupied(&self, coordinates: &(usize, usize)) -> bool {
        self.grid[coordinates.0][coordinates.1] != Space::Empty
    }

    fn set_space(&mut self, space: Space, coordinates: (usize, usize)) {
        self.grid[coordinates.0][coordinates.1] = space;
    }

    pub fn game_over(&self) -> bool {
        let has_three_in_a_row = match self.get_winner() {
            Some(_winner) => true,
            None => false,
        };

        let all_occupied = self.grid.iter().all(|&row| {
            row.iter().all(|&space| space != Space::Empty)
        });

        has_three_in_a_row || all_occupied
    }

    pub fn get_winner(&self) -> Option<&Space> {
        [Space::X, Space::O].iter().find(|&player| {
            ROWS.iter().any(|&row| {
                let mut spaces = [Space::Empty;3];
                for (i, coordinates) in row.iter().enumerate() {
                    spaces[i] = self.grid[coordinates.0][coordinates.1];
                }
                spaces.iter().all(|x| x == player)
            })        
        })
    }

    pub fn revert_set_space(&mut self, board_memento: &BoardMemento) -> BoardMemento {
        match board_memento {
            BoardMemento::SetSpace(set_space_memento) => {
                let coordinates = set_space_memento.get_coordinates();
                self.set_cursor_position(coordinates);
                self.set_space(Space::Empty, coordinates)
            },
            _ => panic!("Only set space memento allowed!"),
        };

        BoardMemento::NullBoardMemento
    }

    fn set_cursor_position(&mut self, coordinates: (usize, usize)) {
        self.cursor.set_coordinates(coordinates);
    }

    pub fn redo_set_space(&mut self, board_memento: &BoardMemento) -> BoardMemento {
        match board_memento {
            BoardMemento::SetSpace(set_space_memento) => {
                let coordinates = set_space_memento.get_coordinates();
                let space = set_space_memento.get_space();
                self.set_cursor_position(coordinates);
                self.set_space(space, coordinates)
            },
            _ => panic!("Only set space memento allowed!"),
        };

        BoardMemento::NullBoardMemento
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_cursor_should_move_the_cursor() {
        let mut board = Board::new();
        let expected = (0, 1);

        board.move_cursor(UserInput::RIGHT);

        let actual = board.cursor.get_coordinates();
        assert_eq!(actual, expected);
    }

    #[test]
    fn move_cursor_should_return_a_board_memento() {
        let mut board = Board::new();
        let expected = true;

        let memento = board.move_cursor(UserInput::RIGHT);

        let actual = match memento {
            BoardMemento::MoveCursor(UserInput::RIGHT) => true,
            _ => false
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_current_space_once_should_return_correct_memento() {
        let mut board = Board::new();
        let expected = true;

        let memento = board.set_current_space(Space::X);

        let actual = match memento {
            BoardMemento::SetSpace(_) => memento.turn_over(),
            _ => false,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn set_current_space_twice_should_return_correct_memento() {
        let mut board = Board::new();
        let expected = true;

        board.set_current_space(Space::X);
        let memento = board.set_current_space(Space::O);

        let actual = match memento {
            BoardMemento::SetSpace(_) => !memento.turn_over(),
            _ => false,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn game_over_new_board() {
        let expected = false;

        let actual = Board::new().game_over();
        assert_eq!(expected, actual);
    }

    #[test]
    fn game_over_minimum_game_over_boards() {
        let expected = true;

        let actual = ROWS.iter().all(|&row| {
            [Space::X, Space::O].iter().all(|space| {
                let mut grid = [[Space::Empty;3];3];
                row.iter().for_each(|&coordinates| {
                    grid[coordinates.0][coordinates.1] = *space;
                });

                let cursor_dimensions = (grid[0].len(), grid[1].len());
                let cursor = Cursor::new(cursor_dimensions);
                let board = Board {
                    cursor,
                    grid,
                };

                board.game_over()
            })
        });
        assert_eq!(expected, actual);
    }

    #[test]
    fn is_winner_new_board() {
        let expected = true;

        let actual = match Board::new().get_winner() {
            None => true,
            _ => false,
        };
        assert_eq!(expected, actual)
    }

    #[test]
    fn is_winner_minimum_game_over_boards() {
        let expected = true;

        let actual = ROWS.iter().all(|&row| {
            [Space::X, Space::O].iter().all(|space| {
                let mut grid = [[Space::Empty;3];3];
                row.iter().for_each(|&coordinates| {
                    grid[coordinates.0][coordinates.1] = *space;
                });

                let cursor_dimensions = (grid[0].len(), grid[1].len());
                let cursor = Cursor::new(cursor_dimensions);
                let board = Board {
                    cursor,
                    grid,
                };

                match board.get_winner() {
                    Some(player) => player == space,
                    None => false,
                }
            })
        });
        assert_eq!(expected, actual);
    }
}
