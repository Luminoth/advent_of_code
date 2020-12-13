use std::io::{self, BufRead};

use regex::Regex;

#[derive(Debug)]
struct PasswordValidator {
    pub character: char,
    pub min: usize,
    pub max: usize,
    pub password: String,
}

impl PasswordValidator {
    pub fn is_valid_part_one(&self) -> bool {
        let count = self.password.matches(self.character).count();
        count >= self.min && count <= self.max
    }

    pub fn is_valid_part_two(&self) -> bool {
        if self.password.len() < self.max - 1 {
            return false;
        }

        let first = self.password.chars().nth(self.min - 1).unwrap() == self.character;
        let second = self.password.chars().nth(self.max - 1).unwrap() == self.character;
        (first || second) && first != second
    }
}

fn part1(validators: impl AsRef<[PasswordValidator]>) {
    let validators = validators.as_ref();

    let mut valid = 0;
    for validator in validators {
        if validator.is_valid_part_one() {
            valid += 1;
        }
    }

    println!(
        "Found {} part one valid passwords out of {}",
        valid,
        validators.len()
    );
}

fn part2(validators: impl AsRef<[PasswordValidator]>) {
    let validators = validators.as_ref();

    let mut valid = 0;
    for validator in validators {
        if validator.is_valid_part_two() {
            valid += 1;
        }
    }

    println!(
        "Found {} part two valid passwords out of {}",
        valid,
        validators.len()
    );
}

fn main() {
    let re = Regex::new(r"(?P<min>\d+)-(?P<max>\d+) (?P<character>.): (?P<password>.*)").unwrap();

    println!("Enter password entries one line at a time:");
    let validators: Vec<PasswordValidator> = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let caps = re.captures(&line).unwrap();

            PasswordValidator {
                character: caps["character"].to_owned().chars().next().unwrap(),
                min: caps["min"].parse().unwrap(),
                max: caps["max"].parse().unwrap(),
                password: caps["password"].to_owned(),
            }
        })
        .collect();

    part1(&validators);
    part2(&validators);
}
