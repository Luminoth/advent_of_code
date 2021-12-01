// there's probably a way to do this with a single regex /shrug

use std::collections::{HashMap, HashSet};

use regex::Regex;

#[derive(Debug, Default, Clone)]
struct Bag {
    pub name: String,
    pub contains: HashMap<String, usize>,

    pub contained_by: HashSet<String>,
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

        Self {
            name,
            contains: bags,
            ..Default::default()
        }
    }

    fn containers_internal(
        bag: &Bag,
        bags: &HashMap<String, Bag>,
        containers: &mut HashSet<String>,
    ) {
        if containers.contains(&bag.name) {
            return;
        }

        for container in &bag.contained_by {
            let bag = bags.get(container).unwrap();
            Bag::containers_internal(bag, bags, containers);

            containers.insert(bag.name.clone());
        }
    }

    pub fn containers(&self, bags: &HashMap<String, Bag>) -> HashSet<String> {
        let mut containers = HashSet::new();

        Bag::containers_internal(self, bags, &mut containers);

        containers
    }

    pub fn contains_total(&self, bags: &HashMap<String, Bag>) -> usize {
        let mut contains = 0;

        for (color, count) in &self.contains {
            let bag = bags.get(color).unwrap();
            contains += count + (count * bag.contains_total(bags));
        }

        contains
    }
}

fn part1(bags: &HashMap<String, Bag>, color: impl AsRef<str>) {
    let bag = bags.get(color.as_ref()).unwrap();

    println!(
        "'{}' bag contained by {} other bags",
        color.as_ref(),
        bag.containers(bags).len()
    );
}

fn part2(bags: &HashMap<String, Bag>, color: impl AsRef<str>) {
    let bag = bags.get(color.as_ref()).unwrap();

    println!(
        "'{}' bag contains {} other bags",
        color.as_ref(),
        bag.contains_total(bags)
    );
}

fn main() {
    let re = Regex::new(r"(?P<container>.+) bags contain (?P<bags>.*).").unwrap();
    let bagsre = Regex::new(r"(?P<count>\d+) (?P<bag>.+) bags?").unwrap();

    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();

    let mut bags: HashMap<String, Bag> = lines
        .iter()
        .map(|line| {
            let caps = re.captures(line).unwrap();

            let bag = Bag::new(caps["container"].to_owned(), &caps["bags"], &bagsre);

            (bag.name.clone(), bag)
        })
        .collect();

    // download that RAM
    let scratch: Vec<Bag> = bags.values().cloned().collect();
    for bag in scratch {
        for contains in bag.contains.keys() {
            bags.get_mut(contains)
                .unwrap()
                .contained_by
                .insert(bag.name.clone());
        }
    }

    part1(&bags, "shiny gold");
    part2(&bags, "shiny gold");
}
