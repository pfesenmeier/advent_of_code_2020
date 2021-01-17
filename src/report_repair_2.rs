pub fn run(file: &str) -> String {
    for i in file.split_ascii_whitespace() {
        for j in file.split_ascii_whitespace() {
            for k in file.split_ascii_whitespace() {
                let first: u32 = i.parse().unwrap();
                let second: u32 = j.parse().unwrap();
                let third: u32 = k.parse().unwrap();
                if first + second + third == 2020 {
                    return (first * second * third).to_string();
                }
            }
        }
    }
    "No match found".to_string()
}
