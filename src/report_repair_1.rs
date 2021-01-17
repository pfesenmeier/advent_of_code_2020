pub fn run(file: &str) -> String {
    for i in file.split_ascii_whitespace() {
        for j in file.split_ascii_whitespace() {
            let first: u32 = i.parse().unwrap();
            let second: u32 = j.parse().unwrap();
            if first + second == 2020 {
                return (first * second).to_string();
            }
        }
    }
    "No match found".to_string()
}
