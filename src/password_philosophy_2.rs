use crate::password_philosophy_1;

pub fn run(file: &str) -> String {
  let mut valid_rules = 0;
  for line in file.lines() {
      let line = password_philosophy_1::Line::new(line);

      if line.verify_positions() {
          valid_rules += 1;
      }
  }
  valid_rules.to_string()
}