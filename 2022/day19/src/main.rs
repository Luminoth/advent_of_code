use regex::Regex;

const MAX_TIME: usize = 24;

// TODO: honestly I'm just trying to avoid another best path solution here :/

#[derive(Debug)]
struct Blueprint {
    id: usize,

    // ore
    ore_robot_cost: usize,

    // ore
    clay_robot_cost: usize,

    // (ore, clay)
    obsidian_robot_cost: (usize, usize),

    // (ore, obsidian)
    geode_robot_cost: (usize, usize),
}

impl From<(usize, usize, usize, (usize, usize), (usize, usize))> for Blueprint {
    fn from(v: (usize, usize, usize, (usize, usize), (usize, usize))) -> Self {
        Self {
            id: v.0,
            ore_robot_cost: v.1,
            clay_robot_cost: v.2,
            obsidian_robot_cost: v.3,
            geode_robot_cost: v.4,
        }
    }
}

#[derive(Debug)]
struct Factory<'a> {
    blueprint: &'a Blueprint,

    ore: usize,
    ore_robots: usize,

    clay: usize,
    clay_robots: usize,

    obsidian: usize,
    obsidian_robots: usize,

    geodes: usize,
    geode_robots: usize,
}

impl<'a> Factory<'a> {
    fn new(blueprint: &'a Blueprint) -> Self {
        Self {
            blueprint,
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0,
        }
    }

    fn can_craft_geode_robot(&self) -> bool {
        self.ore >= self.blueprint.geode_robot_cost.0
            && self.obsidian >= self.blueprint.geode_robot_cost.1
    }

    fn can_craft_obsidian_robot(&self) -> bool {
        self.ore >= self.blueprint.obsidian_robot_cost.0
            && self.clay >= self.blueprint.obsidian_robot_cost.1
    }

    fn can_craft_clay_robot(&self) -> bool {
        self.ore >= self.blueprint.clay_robot_cost
    }

    fn can_craft_ore_robot(&self) -> bool {
        self.ore >= self.blueprint.ore_robot_cost
    }

    // should craft a robot if it can contribute to the effort
    // without delaying the construction of a better robot
    // (geode robots have to wait for obsidian, obsidian robots have to wait for clay)

    fn should_craft_obsidian_robot(&self) -> bool {
        !(self.ore + self.ore_robots >= self.blueprint.geode_robot_cost.0
            && self.obsidian + self.obsidian_robots >= self.blueprint.geode_robot_cost.1)
    }

    fn should_craft_clay_robot(&self) -> bool {
        !(self.ore + self.ore_robots >= self.blueprint.geode_robot_cost.0
            && self.obsidian + self.obsidian_robots >= self.blueprint.geode_robot_cost.1)
            && !(self.ore + self.ore_robots >= self.blueprint.obsidian_robot_cost.0
                && self.clay + self.clay_robots >= self.blueprint.obsidian_robot_cost.1)
    }

    fn should_craft_ore_robot(&self) -> bool {
        !(self.ore + self.ore_robots >= self.blueprint.geode_robot_cost.0
            && self.obsidian + self.obsidian_robots >= self.blueprint.geode_robot_cost.1)
            && !(self.ore + self.ore_robots >= self.blueprint.obsidian_robot_cost.0
                && self.clay + self.clay_robots >= self.blueprint.obsidian_robot_cost.1)
            && !(self.ore + self.ore_robots >= self.blueprint.clay_robot_cost)
    }

    fn max_geodes(&mut self, mut time: usize) -> usize {
        loop {
            if time == 0 {
                break;
            }

            println!("== Minute {} ==", MAX_TIME - time + 1);

            // start craft robots

            let crafting_geode_robots = if self.can_craft_geode_robot() {
                println!(
                    "Spend {} ore and {} obsidian to start building a geode-cracking robot.",
                    self.blueprint.geode_robot_cost.0, self.blueprint.geode_robot_cost.1
                );

                self.ore -= self.blueprint.geode_robot_cost.0;
                self.obsidian -= self.blueprint.geode_robot_cost.1;

                1
            } else {
                0
            };

            let crafting_obsidian_robots =
                if self.should_craft_obsidian_robot() && self.can_craft_obsidian_robot() {
                    println!(
                        "Spend {} ore and {} clay to start building an obsidian-collecting robot.",
                        self.blueprint.obsidian_robot_cost.0, self.blueprint.obsidian_robot_cost.1
                    );

                    self.ore -= self.blueprint.obsidian_robot_cost.0;
                    self.clay -= self.blueprint.obsidian_robot_cost.1;

                    1
                } else {
                    0
                };

            let crafting_clay_robots =
                if self.should_craft_clay_robot() && self.can_craft_clay_robot() {
                    println!(
                        "Spend {} ore to start building a clay-collecting robot.",
                        self.blueprint.clay_robot_cost
                    );

                    self.ore -= self.blueprint.clay_robot_cost;

                    1
                } else {
                    0
                };

            let crafting_ore_robots = if self.should_craft_ore_robot() && self.can_craft_ore_robot()
            {
                println!(
                    "Spend {} ore to start building an ore-collecting robot.",
                    self.blueprint.ore_robot_cost
                );

                self.ore -= self.blueprint.ore_robot_cost;

                1
            } else {
                0
            };

            // robots collect ores

            if self.ore_robots > 0 {
                self.ore += self.ore_robots;

                println!(
                    "{} ore-collecting robot collects {} ore; you now have {} ore.",
                    self.ore_robots, self.ore_robots, self.ore
                );
            }

            if self.clay_robots > 0 {
                self.clay += self.clay_robots;

                println!(
                    "{} clay-collecting robot collects {} clay; you now have {} clay.",
                    self.clay_robots, self.clay_robots, self.clay
                );
            }

            if self.obsidian_robots > 0 {
                self.obsidian += self.obsidian_robots;

                println!(
                    "{} obsidian-collecting robot collects {} obsidian; you now have {} obsidian.",
                    self.obsidian_robots, self.obsidian_robots, self.obsidian
                );
            }

            if self.geode_robots > 0 {
                self.geodes += self.geode_robots;

                println!(
                    "{} geode-cracking robot cracks {} geode; you now have {} open geode.",
                    self.geode_robots, self.geode_robots, self.geodes
                );
            }

            time -= 1;

            // robots finish

            if crafting_ore_robots > 0 {
                self.ore_robots += crafting_ore_robots;

                println!(
                    "The new ore-collecting robot is ready; you now have {} of them.",
                    self.ore_robots
                );
            }

            if crafting_clay_robots > 0 {
                self.clay_robots += crafting_clay_robots;

                println!(
                    "The new clay-collecting robot is ready; you now have {} of them.",
                    self.clay_robots
                );
            }

            if crafting_obsidian_robots > 0 {
                self.obsidian_robots += crafting_obsidian_robots;

                println!(
                    "The new obsidian-collecting robot is ready; you now have {} of them.",
                    self.obsidian_robots
                );
            }

            if crafting_geode_robots > 0 {
                self.geode_robots += crafting_geode_robots;

                println!(
                    "The new geode-cracking robot is ready; you now have {} of them.",
                    self.geode_robots
                );
            }

            println!();
        }

        self.geodes
    }
}

fn part1(blueprints: impl AsRef<[Blueprint]>) {
    let mut total = 0;
    for blueprint in blueprints.as_ref() {
        let mut factory = Factory::new(blueprint);

        let geodes = factory.max_geodes(MAX_TIME);
        let quality = blueprint.id * geodes;
        println!(
            "blueprint {} got {} geodes ({})",
            blueprint.id, geodes, quality
        );

        total += quality;

        break;
    }
    println!("Total quality: {}", total);
}

fn main() {
    let re = Regex::new(r"Blueprint (\d+):\s*Each ore robot costs (\d+) ore.\s*Each clay robot costs (\d+) ore.\s*Each obsidian robot costs (\d+) ore and (\d+) clay.\s*Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();

    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let captures = re.captures(x).unwrap();

            let id = captures.get(1).unwrap().as_str().parse().unwrap();
            let ore_cost = captures.get(2).unwrap().as_str().parse().unwrap();
            let clay_cost = captures.get(3).unwrap().as_str().parse().unwrap();
            let obsidian_cost = (
                captures.get(4).unwrap().as_str().parse().unwrap(),
                captures.get(5).unwrap().as_str().parse().unwrap(),
            );
            let geode_cost = (
                captures.get(6).unwrap().as_str().parse().unwrap(),
                captures.get(7).unwrap().as_str().parse().unwrap(),
            );

            Some((id, ore_cost, clay_cost, obsidian_cost, geode_cost).into())
        })
        .collect::<Vec<_>>();

    part1(values);
}
