pub fn run(file: &str) -> String {
    let mut valid_rules: u32 = 0;
    for line in file.lines() {
        let line = Line::new(line);

        if line.verify_range() {
            valid_rules += 1;
        }
    }

    valid_rules.to_string()
}

pub struct Line<'a> {
    rule: Rule,
    password: &'a str,
}
impl Line<'_> {
    pub fn new(line: &str) -> Line {
        let input: Vec<&str> = line.split(':').collect();
        Line {
            rule: Rule::new(input[0]),
            password: input[1].trim(),
        }
    }

    pub fn verify_range(&self) -> bool {
        self.rule.verify_range(self.password)
    }

    pub fn verify_positions(&self) -> bool {
        self.rule.verify_positions(self.password)
    }
}

struct Rule {
    first: u8,
    second: u8,
    letter: char,
}
impl Rule {
    fn new(range: &str) -> Rule {
        let numbers: Vec<&str> = range.split(|c: char| !c.is_alphanumeric()).collect();
        Rule {
            first: numbers[0].parse().unwrap(),
            second: numbers[1].parse().unwrap(),
            letter: numbers[2].parse().unwrap(),
        }
    }

    fn verify_range(&self, password: &str) -> bool {
        let occurences: u8 = self.get_occurences(&password);

        if occurences >= self.first && occurences <= self.second {
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

    fn verify_positions(&self, password: &str) -> bool {
        struct ToobogganIndices {
            first: u8,
            second: u8,
        }
        impl ToobogganIndices {
            fn to_0_indices(&self) -> (u8, u8) {
                (self.first - 1, self.second - 1)
            }
        }

        let toobaggon_indices = ToobogganIndices {
            first: self.first,
            second: self.second,
        };

        let (first, second) = toobaggon_indices.to_0_indices();
        let mut i: u8 = 0;
        let mut matches = 0;
        for c in password.chars() {
            if (i == first || i == second) && c == self.letter {
                matches += 1;
            }
            i += 1;
        }
        matches == 1
    }
}
