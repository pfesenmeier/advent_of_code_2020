pub fn run(file: &str) -> String {
  
  let hills: Vec<&str> = file.split_ascii_whitespace().collect();
  let ski = Ski { right: 3, down: 1 };
  ski.ski(&hills).to_string()
}
pub struct Ski {
    pub right: u32,
    pub down: usize,
}

impl Ski {
    pub fn ski (&self, hills: &[&str]) -> u32 {
      self.sled(hills, 0, 0)
    }
    fn sled(&self, hills: &[&str], pos: u32, trees: u32) -> u32 {
        let hill = hills.into_iter().next();

        if let None = hill {
            return trees;
        }

        let hill = hill.unwrap();
        let downhill = &hills[self.down..];
        let next_pos = || (pos + self.right) % hill.len() as u32;

        let mut i = 0;
        for c in hill.chars() {
            if i == pos && c == '#' {
                return self.sled(downhill, next_pos(), trees + 1);
            } else if i == pos {
                return self.sled(downhill, next_pos(), trees);
            }
            i += 1;
        }
        // should never reach here
        42
    }
}
