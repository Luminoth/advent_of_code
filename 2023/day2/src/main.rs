use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumString)]
enum Color {
    #[strum(ascii_case_insensitive)]
    Red,

    #[strum(ascii_case_insensitive)]
    Green,

    #[strum(ascii_case_insensitive)]
    Blue,
}

#[derive(Debug)]
struct Game {
    id: usize,
    results: Vec<HashMap<Color, usize>>,
}

impl From<&str> for Game {
    fn from(v: &str) -> Self {
        let re = Regex::new(r"Game (?P<id>\d+): (?<pulls>.*)+").unwrap();
        let caps = re.captures(v).unwrap();

        let id = caps["id"].parse().unwrap();

        // this is probably all doable in the regex :shrug:
        let mut results = vec![];
        let pulls = caps["pulls"].split(';');
        for pull in pulls {
            let mut cubes = HashMap::new();
            let counts = pull.split(',');
            for count in counts {
                let parts = count.trim().split_once(' ').unwrap();

                let count = parts.0.parse().unwrap();
                let color = Color::from_str(parts.1).unwrap();
                let old = cubes.insert(color, count);
                assert!(old.is_none());
            }
            results.push(cubes);
        }

        Self { id, results }
    }
}

impl Game {
    fn is_possible(&self, max_red: usize, max_green: usize, max_blue: usize) -> bool {
        for result in &self.results {
            if let Some(count) = result.get(&Color::Red) {
                if *count > max_red {
                    return false;
                }
            }

            if let Some(count) = result.get(&Color::Green) {
                if *count > max_green {
                    return false;
                }
            }

            if let Some(count) = result.get(&Color::Blue) {
                if *count > max_blue {
                    return false;
                }
            }
        }

        true
    }

    fn get_min_needed(&self) -> (usize, usize, usize) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for result in &self.results {
            if let Some(count) = result.get(&Color::Red) {
                red = red.max(*count);
            }

            if let Some(count) = result.get(&Color::Green) {
                green = green.max(*count);
            }

            if let Some(count) = result.get(&Color::Blue) {
                blue = blue.max(*count);
            }
        }

        (red, green, blue)
    }

    fn get_min_power(&self) -> usize {
        let (red, green, blue) = self.get_min_needed();
        red * green * blue
    }
}

fn part1(games: impl AsRef<[Game]>) {
    let mut sum = 0;

    for game in games.as_ref() {
        if game.is_possible(12, 13, 14) {
            sum += game.id;
        }
    }

    assert!(sum == 2545);
    println!("Sum: {}", sum);
}

fn part2(games: impl AsRef<[Game]>) {
    let mut sum = 0;

    for game in games.as_ref() {
        sum += game.get_min_power();
    }

    assert!(sum == 78111);
    println!("Sum: {}", sum);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input.lines().map(Game::from).collect::<Vec<_>>();

    part1(&values);
    part2(values);
}
