use std::process::exit;

use console::style;

pub fn warn(message: &str) {
  println!("{}", style(message).yellow());
  exit(0);
}

pub fn error(message: &str) {
  println!("{}", style(message).red());
  exit(1)
}
