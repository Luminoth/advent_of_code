use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Plant {
    r#type: char,
    x: isize,
    y: isize,
    sides: usize,
}

impl Plant {
    fn next_to(&self, other: Self) -> bool {
        let x_distance = (other.x - self.x).abs();
        let y_distance = (other.y - self.y).abs();

        // cross but not diagonal
        x_distance <= 1 && y_distance <= 1 && x_distance != y_distance
    }
}

type Regions = HashMap<char, Vec<Vec<Plant>>>;

fn part1(regions: &Regions) {
    let mut total = 0;
    for (plant, plots) in regions {
        for plot in plots {
            let area = plot.len();
            let perimeter = plot.iter().map(|plant| plant.sides).sum::<usize>();
            let cost = area * perimeter;
            println!("region for {} is {} * {}: {}", plant, area, perimeter, cost);

            total += cost;
        }
    }

    println!("total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut regions: Regions = Regions::new();
    for (y, line) in input.lines().enumerate() {
        for (x, plant) in line.chars().enumerate() {
            let mut plant = Plant {
                r#type: plant,
                x: x as isize,
                y: y as isize,
                sides: 4,
            };

            let plots = regions.entry(plant.r#type).or_default();

            // TODO: this is failing on left edges of plots:
            // ...
            // |....F...|
            // |   FF...|
            // ...
            // so we probably need to compare to the *map*
            // not the list of plants we've already added

            let mut found = false;
            for plot in plots.iter_mut() {
                for p in plot.iter_mut() {
                    if p.next_to(plant) {
                        p.sides -= 1;
                        plant.sides -= 1;
                        found = true;
                    }
                }

                if found {
                    plot.push(plant);
                    break;
                }
            }

            if !found {
                println!(
                    "adding new plant {} at {:?}",
                    plant.r#type,
                    (plant.x, plant.y)
                );
                plots.push(vec![plant]);
            }
        }
    }

    part1(&regions);
}
