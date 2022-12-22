use std::fmt;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res},
    multi::many_till,
    Finish, IResult,
};

#[derive(Debug)]
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

#[derive(Debug, Default)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Direction {
    x: i64,
    y: i64,
}

impl Direction {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        match turn {
            Turn::Clockwise => {
                let x = self.x;
                self.x = self.y;
                self.y = -x;
            }
            Turn::CounterClockwise => {
                let x = self.x;
                self.x = -self.y;
                self.y = x;
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
struct Tile {
    r#type: TileType,

    up: Position,
    down: Position,
    left: Position,
    right: Position,
}

impl From<char> for Tile {
    fn from(v: char) -> Self {
        Self {
            r#type: v.into(),
            up: Position::default(),
            down: Position::default(),
            left: Position::default(),
            right: Position::default(),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<Tile>>,
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

fn part1(map: &Map, instructions: impl AsRef<[Instruction]>) {
    let instructions = instructions.as_ref();

    let start = Position::default();
    let direction = Direction::new(1, 0);

    println!("{}", map);
    println!("{:?}", instructions);
}

fn main() {
    let input = include_str!("../input.txt");

    let (m, n) = input.split_once("\n\n").unwrap();

    let map = m.into();
    let instructions = all_consuming(parse_instructions)(n).finish().unwrap().1 .0;

    part1(&map, instructions);
}
