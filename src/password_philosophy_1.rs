pub fn run(file: &str) -> String {
    let mut valid_rules: u32 = 0;
    for line in file.lines() {
        let line = Line::new(line);

        if line.verify_password() {
            valid_rules += 1;
        }
    }

    valid_rules.to_string()
}

struct Line<'a> {
    rule: Rule,
    password: &'a str,
}
impl Line<'_> {
    fn new(line: &str) -> Line {
        let input: Vec<&str> = line.split(':').collect();
        Line {
        rule: Rule::new(input[0]),
        password:  input[1].trim(),
        }
    }

    fn verify_password(&self) -> bool {
        self.rule.verify_range(self.password)
    }
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
