#[derive(Copy, Clone, PartialEq)]
pub enum Space {
    X,
    O,
    Empty,
}

impl Space {
    pub fn as_str(&self) -> &str {
        match &self {
            Space::X => "X",
            Space::O => "O",
            Space::Empty => " ",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_x() {
        let expected = "X";
        let actual = Space::X.as_str();
        assert_eq!(actual, expected);
    }

    #[test]
    fn as_str_o() {
        let expected = "O";
        let actual = Space::O.as_str();
        assert_eq!(actual, expected);
    }

    #[test]
    fn as_str_empty() {
        let expected = " ";
        let actual = Space::Empty.as_str();
        assert_eq!(actual, expected);
    }    
}