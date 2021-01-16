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

struct Challenge<'a> {
    day: u8,
    part: u8,
    input_name: &'a str,
}

impl Challenge<'_> {
    fn new(args: &[String]) -> Result<Challenge, &'static str> {
        match args.len() {
            3 => {
              let nums: Vec<&str> = args[1].split(|input: char| { !input.is_digit(10)}).collect();
              let day: u8 = nums[0].parse().expect("Could not parse a day from arguments");
              let part: u8 = nums[1].parse().expect("Could not parse a part from arguments");
              Ok(Challenge {
                  day,
                  part,
                  input_name: &args[2],
              })
            },
            _ => Err("Please provide two arguments"),
        }
    }

    fn run(&self) {
        let problem_func = self
            .get_problem_func()
            .expect("Could not match problem argument. Try 'day-part' e.g. '1-2'");
        let file = self.get_file_location();

        let result = problem_func(file);

        println!("The answer to day {} part {} is {}", self.day, self.part, &result);
    }

    fn get_file_location(&self) -> &str {
        self.input_name
    }

    fn get_problem_func(&self) -> Option<impl Fn(&str) -> &str> {
        match (self.day, self.part) {
            (1, 1) => Some(report_repair_1),
            _ => None,
        }
    }
}

fn report_repair_1(_file_name: &str) -> &str {
    "42"
}
