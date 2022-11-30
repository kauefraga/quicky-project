use std::process::exit;
use console::style;

pub struct Console {
}

impl Console {
  pub fn new() -> Self {
    Console {}
  }

  pub fn warn(&mut self, message: &str) -> &mut Console {
    println!("{}", style(message).yellow());
    self
  }

  pub fn error(&mut self, message: &str) -> &mut Console {
    println!("{}", style(message).red());
    self
  }

  pub fn tip(&mut self, message: &str) -> &mut Console {
    println!("{}", style(message).green());
    self
  }

  pub fn exit(&mut self, code: i32) {
    exit(code);
  }
}
