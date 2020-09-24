use ansi_term::Colour;
use crate::space::Space;
use crate::cursor::Cursor;
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
        Board {
            cursor: Cursor::new((3, 3)),
            grid: [[Space::Empty;3];3],
        }
    }
    
    pub fn render(&self) {       
        for (i, row) in self.grid.iter().enumerate() {
            for (j, space) in row.iter().enumerate() {
                let is_cursor_pos = self.cursor.coordinates == (i, j);
                let colored_space = match is_cursor_pos {
                    true => Colour::Black.on(Colour::Yellow).paint(space.as_str()),
                    false => Colour::White.on(Colour::Black).paint(space.as_str()),
                };

                print!("{}", colored_space);

                match j >= row.len() - 1 {
                    true => println!(""),
                    false => print!("{}", Colour::White.on(Colour::Black).paint("|")),
                };
            }
        }
    }

    pub fn move_cursor(&mut self, user_input: UserInput) {
        self.cursor.move_cursor(user_input);
    }

    pub fn set_current_space(&mut self, space: Space) -> bool {
        let has_set_space = !self.is_space_occupied(&self.cursor.coordinates);
        if has_set_space {
            self.set_space(space, self.cursor.coordinates)
        }

        has_set_space
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
                spaces.iter().all(|&x| x == *player)
            })        
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_space_occupied_new_board() {
        let board = Board::new();
        let expected = false;
        
        let actual = (0..3).any(|row| {
            (0..3).any(|col| {
                board.is_space_occupied(&(row, col))
            })
        });

        assert_eq!(actual, expected);
    }

    #[test]
    fn is_space_occupied_set_space() {
        let expected = true;

        let actual = (0..3).all(|row| {
            (0..3).all(|col| {
                let mut board = Board::new();
                board.set_space(Space::O, (row, col));
                board.is_space_occupied(&(row, col))
            })
        });

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
                let mut board = Board::new();

                row.iter().for_each(|&coordinates| {
                    board.set_space(*space, coordinates)
                });

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
                let mut board = Board::new();

                row.iter().for_each(|&coordinates| {
                    board.set_space(*space, coordinates)
                });

                match board.get_winner() {
                    Some(player) => player == space,
                    None => false,
                }
            })
        });
        assert_eq!(expected, actual);
    }
}
