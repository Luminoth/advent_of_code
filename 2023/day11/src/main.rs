#[derive(Debug, Clone)]
struct Galaxy {
    coords: (i64, i64),
}

impl From<(i64, i64)> for Galaxy {
    fn from(v: (i64, i64)) -> Self {
        Self { coords: v }
    }
}

impl Galaxy {
    fn manhattan_distance(&self, other: &Galaxy) -> i64 {
        (other.coords.0 - self.coords.0).abs() + (other.coords.1 - self.coords.1).abs()
    }
}

fn expand_y(grid: &mut Vec<Galaxy>, amount: i64) {
    grid.sort_by(|x, y| x.coords.0.cmp(&y.coords.0));

    let mut last_x = 0;
    let mut x_exp = 0;
    for galaxy in grid {
        let galaxy_x = galaxy.coords.0;

        let diff = galaxy_x - last_x;
        if diff > 1 {
            x_exp += (diff - 1) * (amount - 1);
        }
        galaxy.coords.0 += x_exp;

        last_x = galaxy_x;
    }
}

fn expand_x(grid: &mut Vec<Galaxy>, amount: i64) {
    grid.sort_by(|x, y| x.coords.1.cmp(&y.coords.1));

    let mut last_y = 0;
    let mut y_exp = 0;
    for galaxy in grid {
        let galaxy_y = galaxy.coords.1;

        let diff = galaxy_y - last_y;
        if diff > 1 {
            y_exp += (diff - 1) * (amount - 1);
        }
        galaxy.coords.1 += y_exp;

        last_y = galaxy_y;
    }
}

fn calculate_distances(grid: &[Galaxy]) -> i64 {
    let mut total = 0;

    for i in 0..grid.len() - 1 {
        for j in (i + 1)..grid.len() {
            let a = &grid[i];
            let b = &grid[j];

            let distance = a.manhattan_distance(b);
            //println!("distance between {:?} and {:?} is {}", a, b, distance);
            total += distance;
        }
    }

    total
}

fn part1(mut grid: Vec<Galaxy>) {
    expand_x(&mut grid, 2);
    expand_y(&mut grid, 2);

    let total = calculate_distances(&grid);

    assert!(total == 10154062);
    println!("Total: {}", total);
}

fn part2(mut grid: Vec<Galaxy>) {
    expand_x(&mut grid, 1000000);
    expand_y(&mut grid, 1000000);

    let total = calculate_distances(&grid);

    assert!(total == 553083047914);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let row = line
                .chars()
                .enumerate()
                .filter_map(|(x, ch)| {
                    if ch != '#' {
                        return None;
                    }
                    Some(Galaxy::from((x as i64, y as i64)))
                })
                .collect::<Vec<_>>();

            if row.is_empty() {
                return None;
            }
            Some(row)
        })
        .flatten()
        .collect::<Vec<_>>();

    part1(grid.clone());
    part2(grid);
}
