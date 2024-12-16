use std::collections::HashMap;

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
enum Tile {
    #[default]
    Empty,
    Wall,
}

#[derive(Debug, Default, Clone)]
struct Box {
    position: Position,
}

impl Box {
    fn new_at(position: Position) -> Self {
        Self { position }
    }

    fn r#move(&mut self, r#move: Move, map: &Map, boxes: &mut HashMap<Position, Box>) -> bool {
        let new_position = match r#move {
            Move::Up => Position::new(self.position.x, self.position.y - 1),
            Move::Down => Position::new(self.position.x, self.position.y + 1),
            Move::Left => Position::new(self.position.x - 1, self.position.y),
            Move::Right => Position::new(self.position.x + 1, self.position.y),
        };

        if boxes.contains_key(&new_position) {
            let mut r#box = boxes.remove(&new_position).unwrap();
            let moved = r#box.r#move(r#move, map, boxes);
            boxes.insert(r#box.position, r#box);

            if moved {
                self.position = new_position;
            }
            return moved;
        }

        match map.tiles[new_position.y][new_position.x] {
            Tile::Empty => self.position = new_position,
            Tile::Wall => return false,
        }

        true
    }
}

#[derive(Debug, Default, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    start: Position,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        match value {
            '^' => Self::Up,
            'v' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => unreachable!("{}", value),
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Robot {
    position: Position,
}

impl Robot {
    fn new_at(position: Position) -> Self {
        Self { position }
    }

    fn r#move(&mut self, r#move: Move, map: &Map, boxes: &mut HashMap<Position, Box>) {
        let new_position = match r#move {
            Move::Up => Position::new(self.position.x, self.position.y - 1),
            Move::Down => Position::new(self.position.x, self.position.y + 1),
            Move::Left => Position::new(self.position.x - 1, self.position.y),
            Move::Right => Position::new(self.position.x + 1, self.position.y),
        };

        if boxes.contains_key(&new_position) {
            let mut r#box = boxes.remove(&new_position).unwrap();
            let moved = r#box.r#move(r#move, map, boxes);
            boxes.insert(r#box.position, r#box);

            if moved {
                self.position = new_position;
            }
            return;
        }

        match map.tiles[new_position.y][new_position.x] {
            Tile::Empty => self.position = new_position,
            Tile::Wall => (),
        }
    }
}

fn part1(input_map: &str, moves: &[Move]) {
    let mut map = Map::default();
    let mut boxes = HashMap::new();
    for (y, line) in input_map.lines().enumerate() {
        let mut row = vec![Tile::Empty; line.len()];
        for (x, ch) in line.chars().enumerate() {
            let position = Position::new(x, y);

            match ch {
                '.' => (),
                '#' => row[x] = Tile::Wall,
                '@' => map.start = position,
                'O' => {
                    boxes.insert(position, Box::new_at(position));
                }
                _ => unreachable!("{}", ch),
            }
        }
        map.tiles.push(row);
    }

    let mut robot = Robot::new_at(map.start);

    for r#move in moves {
        robot.r#move(*r#move, &map, &mut boxes);
    }

    let mut total = 0;
    for position in boxes.keys() {
        total += position.y * 100 + position.x;
    }

    assert!(total == 1515788);
    println!("Total: {}", total);
}

fn part2(_input_map: &str, _moves: &[Move]) {}

fn main() {
    let input = include_str!("../input.txt");
    let (input_map, moves) = input.split_once("\n\n").unwrap();

    let moves = moves
        .chars()
        .filter_map(|ch| {
            if ch.is_whitespace() {
                return None;
            }
            Some(Move::from(ch))
        })
        .collect::<Vec<_>>();

    part1(input_map, &moves);
    part2(input_map, &moves);
}
