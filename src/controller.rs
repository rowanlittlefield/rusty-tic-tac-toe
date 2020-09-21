use std::io;
use crate::user_input::UserInput;

pub fn get_user_input() -> UserInput {
  let mut input = String::new();
  let mut is_valid_input = false;

  while !is_valid_input {
    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read line");
  
    input = input.trim()
      .parse()
      .unwrap();

    is_valid_input = match map_input(&input) {
      None => false,
      _ => true,
    };
  }

  map_input(&input).unwrap()
}

fn map_input(input: &String) -> Option<UserInput> {
  let input_str = input.as_str();
  println!("input_str {}", input_str);
  match input_str {
    "w" => Some(UserInput::UP),
    "d" => Some(UserInput::RIGHT),
    "s" => Some(UserInput::DOWN), 
    "a" => Some(UserInput::LEFT),
    "" => Some(UserInput::ENTER),
    _ => None,
  }
}