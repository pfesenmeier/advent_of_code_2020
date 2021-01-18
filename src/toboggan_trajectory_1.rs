pub fn run(file: &str) -> String {
    fn sled(hills: &[&str], pos: u32, trees: u32) -> u32 {
        let hill = hills.into_iter().next();

        if let None = hill {
          return trees;
        }
        
        let hill = hill.unwrap();
        let downhill = &hills[1..];
        let next_pos = || (pos + 3) % hill.len() as u32;

        let mut i = 0;
        for c in hill.chars() {
            if i == pos && c == '#' {
                return sled(downhill, next_pos(), trees + 1);
            } else if i == pos {
                return sled(downhill, next_pos(), trees);
            }
            i += 1;
        }
        // should never reach here
        42
    }
    let hills: Vec<&str> = file.split_ascii_whitespace().collect();
    sled(&hills, 0, 0).to_string()
}
