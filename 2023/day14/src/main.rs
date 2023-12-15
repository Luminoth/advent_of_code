#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug, Clone)]
enum Rock {
    Round(usize, usize),
    Solid(usize, usize),
}

impl From<(char, usize, usize)> for Rock {
    fn from(v: (char, usize, usize)) -> Self {
        match v.0 {
            'O' => Self::Round(v.1, v.2),
            '#' => Self::Solid(v.1, v.2),
            _ => unreachable!(),
        }
    }
}

impl Rock {
    fn get_x(&self) -> usize {
        match self {
            Self::Round(x, _) => *x,
            Self::Solid(x, _) => *x,
        }
    }

    #[allow(dead_code)]
    fn set_x(&mut self, new_x: usize) {
        match self {
            Self::Round(x, _) => *x = new_x,
            Self::Solid(x, _) => *x = new_x,
        }
    }

    fn get_y(&self) -> usize {
        match self {
            Self::Round(_, y) => *y,
            Self::Solid(_, y) => *y,
        }
    }

    fn set_y(&mut self, new_y: usize) {
        match self {
            Self::Round(_, y) => *y = new_y,
            Self::Solid(_, y) => *y = new_y,
        }
    }
}

fn init_cols(rocks: &mut [Rock], width: usize) -> Vec<Vec<&mut Rock>> {
    let mut cols = Vec::with_capacity(width);
    for _ in 0..width {
        cols.push(vec![]);
    }

    for rock in rocks {
        cols[rock.get_x()].push(rock);
    }

    cols
}

#[allow(dead_code)]
fn init_rows(rocks: &mut [Rock], height: usize) -> Vec<Vec<&mut Rock>> {
    let mut rows = Vec::with_capacity(height);
    for _ in 0..height {
        rows.push(vec![]);
    }

    for rock in rocks {
        rows[rock.get_y()].push(rock);
    }

    rows
}

fn tilt_rocks(rocks: &mut [Rock], width: usize, direction: Direction) {
    match direction {
        Direction::North => {
            let mut cols = init_cols(rocks, width);

            //println!("rock cols before: {:?}", rock_cols);

            for col in &mut cols {
                let mut prev_y = 0;
                for (idx, rock) in col.iter_mut().enumerate() {
                    if !matches!(rock, Rock::Round(_, _)) {
                        prev_y = rock.get_y();
                        continue;
                    }

                    if idx == 0 {
                        rock.set_y(0);
                        prev_y = 0;
                        continue;
                    }

                    rock.set_y(prev_y + 1);
                    prev_y += 1;
                }
            }

            //println!("rock cols after: {:?}", rock_cols);
        }
        Direction::West => {}
        Direction::South => {}
        Direction::East => {}
    }
}

fn get_load(rocks: &[Rock], height: usize) -> usize {
    let mut load = 0;

    for rock in rocks {
        if !matches!(rock, Rock::Round(_, _)) {
            continue;
        }

        //println!("consider {} vs {:?}", height, rock);

        let value = height - rock.get_y();
        //println!("value: {}", value);
        load += value;
    }

    load
}

fn part1(mut rocks: Vec<Rock>) {
    let width = rocks.iter().map(|rock| rock.get_x()).max().unwrap() + 1;
    let height = rocks.iter().map(|rock| rock.get_y()).max().unwrap() + 1;

    tilt_rocks(&mut rocks, width, Direction::North);

    let load = get_load(&rocks, height);

    assert!(load == 108935);
    println!("Load: {}", load);
}

fn part2(mut rocks: Vec<Rock>) {
    let width = rocks.iter().map(|rock| rock.get_x()).max().unwrap() + 1;
    let height = rocks.iter().map(|rock| rock.get_y()).max().unwrap() + 1;

    // TODO: this is a big nope, this is too much to actually brute force
    // there has to be a cycle somewhere in here we can stop on
    for _idx in 0..1000000000 {
        //println!("cycle {} running", _idx);

        // TODO: we can just rotate the rocks 90 degrees and then tilt north
        tilt_rocks(&mut rocks, width, Direction::North);
        tilt_rocks(&mut rocks, width, Direction::West);
        tilt_rocks(&mut rocks, width, Direction::South);
        tilt_rocks(&mut rocks, width, Direction::East);
    }

    let load = get_load(&rocks, height);

    //assert!(load == ???);
    println!("Load: {}", load);
}

fn main() {
    let input = include_str!("../input.txt");
    let rocks = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, ch)| {
                    if ch == '.' {
                        None
                    } else {
                        Some(Rock::from((ch, x, y)))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(rocks.clone());
    part2(rocks);
}
