pub fn run(file: &str) -> String {
  
  let hills: Vec<&str> = file.split_ascii_whitespace().collect();
  let ski = Ski { right: 3, down: 1 };
  ski.sled(&hills, 0, 0).to_string()
}
pub struct Ski {
    right: u32,
    down: usize,
}

impl Ski {
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
