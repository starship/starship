use std::env;
use ansi_term::Color;

pub fn display() {
  let PROMPT_CHAR = "➜ ";
  let COLOR_SUCCESS = Color::Green;
  let COLOR_FAILURE = Color::Red;

  let color = match env::var_os("status") {
    None | "0" => COLOR_SUCCESS,
    _ => COLOR_FAILURE
  };

  // let color = match env::var("status") {
  //   Ok("0") | _ => COLOR_SUCCESS,
  //   Ok("1") => COLOR_FAILURE
  // };

  print!("{}", color.paint(PROMPT_CHAR));
}
