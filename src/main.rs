use std::env;
use std::char;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let input = fs::read_to

    parse_name(filename);
}

enum Part {
    A,
    B,
}
struct Solution {
    day: u8,
    part: Part,
}
fn parse_name(file_name: &str) -> Result<Solution, > {
//   if file_name.len() != 3 { return None };

  let day: u8 = file_name[0..2].parse().expect("Failed to parse a day.")?;
  println!("{}", day);
}