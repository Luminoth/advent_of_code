// there's probably a way to do this with a single regex /shrug

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
struct Bag {
    pub name: String,
    pub bags: HashMap<String, usize>,
}

impl Bag {
    pub fn new(name: String, bagdefs: impl AsRef<str>, re: &Regex) -> Self {
        let mut bags = HashMap::new();
        if bagdefs.as_ref() != "no other bags" {
            for bagdef in bagdefs.as_ref().split(',') {
                let caps = re.captures(bagdef.trim()).unwrap();
                bags.insert(caps["bag"].to_owned(), caps["count"].parse().unwrap());
            }
        }

        Self { name, bags }
    }
}

fn main() {
    let re = Regex::new(r"(?P<container>.+) bags contain (?P<bags>.*).").unwrap();
    let bagsre = Regex::new(r"(?P<count>\d+) (?P<bag>.+) bags?").unwrap();

    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();

    let bags: HashMap<String, Bag> = lines
        .iter()
        .map(|line| {
            let caps = re.captures(&line).unwrap();

            let bag = Bag::new(caps["container"].to_owned(), &caps["bags"], &bagsre);

            (bag.name.clone(), bag)
        })
        .collect();

    println!("{:?}", bags);
}
