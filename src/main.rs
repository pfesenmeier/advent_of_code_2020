use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let challenge = Challenge::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    challenge.run();
}

struct Challenge {
    day: u8,
    part: u8,
    input_name: String,
}

impl Challenge {
    fn new(args: &[String]) -> Result<Challenge, &'static str> {
        fn parse_day_part(day_part: &str) -> Result<(u8, u8), &'static str> {
            let nums: Vec<&str> = day_part.split(|input: char| !input.is_digit(10)).collect();
            let day: u8 = nums[0]
                .parse()
                .expect("Could not parse a day from arguments");
            let part: u8 = nums[1]
                .parse()
                .expect("Could not parse a part from arguments");
            Ok((day, part))
        }
        match args.len() {
            3 => {
                let (day, part) =
                    parse_day_part(&args[1]).expect("Error parsing challenge identifier");
                Ok(Challenge {
                    day,
                    part,
                    input_name: String::from(&args[2]),
                })
            }
            2 => {
                let (day, part) =
                    parse_day_part(&args[1]).expect("Error parsing challenge identifier");
                let mut input_name: String = day.to_string();
                input_name.push('-');
                input_name.push_str(&part.to_string());
                input_name.push_str(".txt");
                Ok(Challenge {
                    day,
                    part,
                    input_name,
                })
            }
            _ => Err("Please provide two arguments"),
        }
    }

    fn run(&self) {
        let problem_func = self
            .get_problem_func()
            .expect("Could not match problem argument. Try 'day-part' e.g. '1-2'");
        let file = self.get_file_location();

        let result = problem_func(file);

        println!(
            "The answer to day {} part {} is {}",
            self.day, self.part, &result
        );
    }

    fn get_file_location(&self) -> &str {
        &self.input_name
    }

    fn get_problem_func(&self) -> Option<impl Fn(&str) -> &str> {
        match (self.day, self.part) {
            (1, 1) => Some(report_repair_1),
            _ => None,
        }
    }
}

fn report_repair_1(_file_name: &str) -> &str {
    _file_name
}
