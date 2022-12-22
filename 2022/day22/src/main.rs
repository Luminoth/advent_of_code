use std::cell::RefCell;
use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res},
    multi::many_till,
    Finish, IResult,
};

fn modulus(n: i64, m: i64) -> i64 {
    ((n % m) + m) % m
}

#[derive(Debug, Copy, Clone)]
enum Turn {
    Clockwise,
    CounterClockwise,
}

impl From<char> for Turn {
    fn from(v: char) -> Self {
        match v {
            'R' => Self::Clockwise,
            'L' => Self::CounterClockwise,
            _ => unreachable!(),
        }
    }
}

fn parse_turn(input: &str) -> IResult<&str, Turn> {
    map(alt((char('R'), char('L'))), Into::into)(input)
}

#[derive(Debug)]
enum Instruction {
    Turn(Turn),
    Distance(usize),
}

impl From<Turn> for Instruction {
    fn from(v: Turn) -> Self {
        Self::Turn(v)
    }
}

impl From<usize> for Instruction {
    fn from(v: usize) -> Self {
        Self::Distance(v)
    }
}

fn parse_instruction_turn(input: &str) -> IResult<&str, Instruction> {
    map(parse_turn, Into::into)(input)
}

fn parse_instruction_distance(input: &str) -> IResult<&str, Instruction> {
    map(map_res(digit1, str::parse::<usize>), Into::into)(input)
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_instruction_turn, parse_instruction_distance))(input)
}

fn parse_instructions(input: &str) -> IResult<&str, (Vec<Instruction>, &str)> {
    many_till(parse_instruction, tag("\n"))(input)
}

#[derive(Debug, Default, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct Direction {
    x: i64,
    y: i64,
}

impl Direction {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn as_value(&self) -> usize {
        if self.x > 0 {
            0
        } else if self.y > 0 {
            1
        } else if self.x < 0 {
            2
        } else if self.y < 0 {
            3
        } else {
            unreachable!()
        }
    }
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        // NOTE: these are backwards because our y is down, not up
        match turn {
            Turn::Clockwise => {
                let x = self.x;
                self.x = -self.y;
                self.y = x;
            }
            Turn::CounterClockwise => {
                let x = self.x;
                self.x = self.y;
                self.y = -x;
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TileType {
    None,
    Open,
    Wall,
}

impl From<char> for TileType {
    fn from(v: char) -> Self {
        match v {
            ' ' => Self::None,
            '.' => Self::Open,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::None => write!(f, " "),
            Self::Open => write!(f, "."),
            Self::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug, Clone)]
struct Tile {
    r#type: TileType,
    connections: [Position; 4],
}

impl Tile {
    fn is_none(&self) -> bool {
        self.r#type == TileType::None
    }

    fn is_open(&self) -> bool {
        self.r#type == TileType::Open
    }

    fn get_next_position(&self, direction: Direction) -> Position {
        self.connections[direction.as_value()]
    }
}

impl From<char> for Tile {
    fn from(v: char) -> Self {
        Self {
            r#type: v.into(),
            connections: [Position::default(); 4],
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn init_part1(&mut self) {
        let mut tiles = self
            .tiles
            .iter()
            .map(|x| {
                x.iter()
                    .map(|y| RefCell::new(y.clone()))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let height = tiles.len();
        for (y, row) in tiles.iter().enumerate() {
            let width = row.len();
            for (x, tile) in row.iter().enumerate() {
                if !tile.borrow().is_open() {
                    continue;
                }

                // right (0)
                let mut t = (x + 1) % width;
                loop {
                    assert!(t != x);

                    if !row.get(t).unwrap().borrow().is_none() {
                        tile.borrow_mut().connections[0] = Position::new(t, y);
                        break;
                    }

                    t = (t + 1) % width;
                }

                // down (1)
                let mut t = (y + 1) % height;
                loop {
                    assert!(t != y);

                    let row = tiles.get(t).unwrap();
                    if x >= row.len() {
                        t = (t + 1) % height;
                        continue;
                    }

                    if !row.get(x).unwrap().borrow().is_none() {
                        tile.borrow_mut().connections[1] = Position::new(x, t);
                        break;
                    }

                    t = (t + 1) % height;
                }

                // left (2)
                let mut t = modulus(x as i64 - 1, width as i64) as usize;
                loop {
                    assert!(t != x);

                    if !row.get(t).unwrap().borrow().is_none() {
                        tile.borrow_mut().connections[2] = Position::new(t, y);
                        break;
                    }

                    t = modulus(t as i64 - 1, width as i64) as usize;
                }

                // up (3)
                let mut t = modulus(y as i64 - 1, height as i64) as usize;
                loop {
                    assert!(t != y);

                    let row = tiles.get(t).unwrap();
                    if x >= row.len() {
                        t = modulus(t as i64 - 1, height as i64) as usize;
                        continue;
                    }

                    if !row.get(x).unwrap().borrow().is_none() {
                        tile.borrow_mut().connections[3] = Position::new(x, t);
                        break;
                    }

                    t = modulus(t as i64 - 1, height as i64) as usize;
                }
            }
        }

        self.tiles = tiles
            .drain(..)
            .map(|mut x| x.drain(..).map(|x| x.into_inner()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
    }

    fn init_part2(&mut self) {
        // TODO:
    }

    fn get_start_position(&self) -> Position {
        for x in 0..self.tiles[0].len() {
            let position = Position::new(x, 0);
            let tile = self.get_tile(position);
            if tile.is_open() {
                return position;
            }
        }
        unreachable!()
    }

    fn get_tile(&self, position: Position) -> &Tile {
        &self.tiles[position.y][position.x]
    }

    fn get_next_position(
        &self,
        mut position: Position,
        direction: Direction,
        distance: usize,
    ) -> (Position, usize) {
        let mut tile = &self.tiles[position.y][position.x];
        for v in 0..distance {
            let next = tile.get_next_position(direction);
            tile = &self.tiles[next.y][next.x];
            if !tile.is_open() {
                return (position, v + 1);
            }
            position = next;
        }
        (position, distance)
    }
}

impl From<&str> for Map {
    fn from(v: &str) -> Self {
        let tiles = v
            .lines()
            .map(|x| x.chars().map(Into::into).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { tiles }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.tiles {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn part1(mut map: Map, instructions: impl AsRef<[Instruction]>) {
    map.init_part1();

    let mut position = map.get_start_position();
    let mut direction = Direction::new(1, 0);

    for instruction in instructions.as_ref() {
        match instruction {
            Instruction::Distance(distance) => {
                let (p, _d) = map.get_next_position(position, direction, *distance);
                position = p;
            }
            Instruction::Turn(turn) => {
                direction.turn(*turn);
            }
        }
    }

    let total = (1000 * (position.y + 1)) + (4 * (position.x + 1)) + direction.as_value();
    assert!(total == 75254);
    println!("Final password: {}", total);
}

fn part2(mut map: Map, instructions: impl AsRef<[Instruction]>) {
    map.init_part2();

    let mut position = map.get_start_position();
    let mut direction = Direction::new(1, 0);

    for instruction in instructions.as_ref() {
        match instruction {
            Instruction::Distance(distance) => {
                let (p, _d) = map.get_next_position(position, direction, *distance);
                position = p;
            }
            Instruction::Turn(turn) => {
                direction.turn(*turn);
            }
        }
    }

    let total = (1000 * (position.y + 1)) + (4 * (position.x + 1)) + direction.as_value();
    //assert!(total == ???);
    println!("Final password: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let (m, n) = input.split_once("\n\n").unwrap();

    let map = Map::from(m);
    let instructions = all_consuming(parse_instructions)(n).finish().unwrap().1 .0;

    part1(map.clone(), &instructions);
    part2(map, instructions);
}
