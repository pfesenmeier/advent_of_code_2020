pub fn run(file: &str) -> String {
    fn sled(hills: &[&str], pos: u32, trees: u32) -> u32 {
        let hill = hills.into_iter().next();
        match hill {
            Some(slope) => {
                let mut i = 0;
                let next_pos = || (pos + 3) % slope.len() as u32;
                println!("{}", next_pos());
                for c in slope.chars() {
                    if i == pos && c == '#' {
                        let hills = &hills[1..];
                        let pos = next_pos();
                        let trees = trees + 1;
                        return sled(hills, pos, trees);
                    } else if i == pos {
                        let hills = &hills[1..];
                        let pos = next_pos();
                        return sled(hills, pos, trees);
                    }
                    i += 1;
                }
                // should never reach here
                42
            }
            None => {
                return trees;
            }
        }
    }
    let hills: Vec<&str> = file.split_ascii_whitespace().collect();
    sled(&hills, 0, 0).to_string()
}
