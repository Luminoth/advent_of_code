use std::fmt;

use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};

#[derive(Debug, Copy, Clone)]
struct Coord {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Coord {
    fn from(v: (usize, usize)) -> Self {
        Self::new(v.0, v.1)
    }
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn parse_val(input: &str) -> IResult<&str, usize> {
    map_res(digit1, str::parse)(input)
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    map(separated_pair(parse_val, char(','), parse_val), Into::into)(input)
}

#[derive(Debug, Clone)]
struct Path {
    coords: Vec<Coord>,
}

impl From<Vec<Coord>> for Path {
    fn from(coords: Vec<Coord>) -> Self {
        Self { coords }
    }
}

fn parse_path(input: &str) -> IResult<&str, Path> {
    map(separated_list1(tag(" -> "), parse_coord), Into::into)(input)
}

#[derive(Debug, Clone)]
struct Grid {
    rocks: Vec<Vec<bool>>,
    sand: Vec<Vec<bool>>,
}

impl From<Vec<Path>> for Grid {
    fn from(paths: Vec<Path>) -> Self {
        let mut rocks = vec![];
        let mut sand = vec![];

        for path in paths {
            for coords in path.coords.windows(2) {
                let miny = coords[0].y.min(coords[1].y);
                let maxy = coords[0].y.max(coords[1].y);
                for y in miny..=maxy {
                    if y >= rocks.len() {
                        rocks.resize(y + 1, vec![false; 1000]);
                        sand.resize(y + 1, vec![false; 1000]);
                    }

                    let minx = coords[0].x.min(coords[1].x);
                    let maxx = coords[0].x.max(coords[1].x);
                    for x in minx..=maxx {
                        rocks[y][x] = true;
                    }
                }
            }
        }

        Self { rocks, sand }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.rocks.len() {
            for x in 0..self.rocks[y].len() {
                if self.sand[y][x] {
                    assert!(!self.rocks[y][x]);

                    write!(f, "o")?;
                } else if self.rocks[y][x] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn is_occupied(&self, coord: Coord) -> bool {
        self.rocks[coord.y][coord.x] || self.sand[coord.y][coord.x]
    }

    fn add_floor(&mut self) {
        self.rocks.push(vec![false; 1000]);
        self.rocks.push(vec![true; 1000]);
        self.sand.push(vec![false; 1000]);
        self.sand.push(vec![false; 1000]);
    }

    fn simulate(&mut self) -> bool {
        let start = Coord::new(500, 0);
        if self.is_occupied(start) {
            return false;
        }

        let mut current = start;
        loop {
            // are we falling into the void now?
            if current.y >= self.rocks.len() - 1 {
                return false;
            }

            // down ?
            let p = Coord::new(current.x, current.y + 1);
            if !self.is_occupied(p) {
                current = p;
                continue;
            }

            // down left ?
            let p = Coord::new(current.x - 1, current.y + 1);
            if !self.is_occupied(p) {
                current = p;
                continue;
            }

            // down right ?
            let p = Coord::new(current.x + 1, current.y + 1);
            if !self.is_occupied(p) {
                current = p;
                continue;
            }

            // come to rest
            self.sand[current.y][current.x] = true;

            return true;
        }
    }
}

fn part1(mut grid: Grid) {
    let mut total = 0;
    loop {
        if !grid.simulate() {
            break;
        }

        //println!("{}", grid);

        total += 1;
    }

    //println!("{}", grid);

    assert!(total == 795);
    println!("Total sand: {}", total);
}

fn part2(mut grid: Grid) {
    grid.add_floor();

    let mut total = 0;
    loop {
        if !grid.simulate() {
            break;
        }

        //println!("{}", grid);

        total += 1;
    }

    //println!("{}", grid);

    assert!(total == 30214);
    println!("Total sand: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .map(|x| all_consuming(parse_path)(x).finish().unwrap().1)
        .collect::<Vec<_>>();

    let grid = values.clone().into();
    part1(grid);

    let grid = values.into();
    part2(grid);
}
