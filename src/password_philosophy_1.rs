pub fn run(file: &str) -> String {
    let mut valid_rules: u32 = 0;
    for line in file.lines() {
        let input: Vec<&str> = line.split(':').collect();
        let rule = Rule::new(input[0]);
        let password = input[1].trim();

        if rule.verify_range(password) {
            valid_rules += 1;
        }
    }

    valid_rules.to_string()
}

struct Rule {
    min: u8,
    max: u8,
    letter: char,
}
impl Rule {
    fn new(range: &str) -> Rule {
        let numbers: Vec<&str> = range.split(|c: char| !c.is_alphanumeric()).collect();
        Rule {
            min: numbers[0].parse().unwrap(),
            max: numbers[1].parse().unwrap(),
            letter: numbers[2].parse().unwrap(),
        }
    }

    fn verify_range(&self, password: &str) -> bool {
        let occurences: u8 = self.get_occurences(&password);

        if occurences >= self.min && occurences <= self.max {
            return true;
        }
        false
    }

    fn get_occurences(&self, password: &str) -> u8 {
        let mut occurences: u8 = 0;
        for c in password.chars() {
            if c == self.letter {
                occurences += 1;
            }
        }
        occurences
    }
}
