pub struct LoxError{
  pub line: usize,
  pub message: String
}

impl LoxError{
  pub fn error(line: usize, message: String) -> LoxError{
    LoxError { line, message }
  }

  pub fn report(&self, loc: &String) {
    eprintln!("[line {}] Error {}: {}", self.line,loc, self.message)
  }
}