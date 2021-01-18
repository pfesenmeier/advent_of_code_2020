use crate::toboggan_trajectory_1;

pub fn run(file: &str) -> String {
  let hills: Vec<&str> = file.split_ascii_whitespace().collect();

    let mut total = toboggan_trajectory_1::Ski { right: 1, down: 1 }.ski(&hills);
    total *= toboggan_trajectory_1::Ski { right: 3, down: 1 }.ski(&hills);
    total *= toboggan_trajectory_1::Ski { right: 5, down: 1 }.ski(&hills);
    total *= toboggan_trajectory_1::Ski { right: 7, down: 1 }.ski(&hills);
    total *= toboggan_trajectory_1::Ski { right: 1, down: 2 }.ski(&hills);
    total.to_string()
}