use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Expedition {
    position: Position,
}

impl Expedition {
    fn new(position: Position) -> Self {
        Self { position }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(v: char) -> Self {
        match v {
            '^' => Self::North,
            'v' => Self::South,
            '>' => Self::East,
            '<' => Self::West,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::North => write!(f, "^"),
            Self::South => write!(f, "v"),
            Self::East => write!(f, ">"),
            Self::West => write!(f, "<"),
        }
    }
}

#[derive(Debug)]
struct Blizzard {
    direction: Direction,
    position: Position,
}

impl fmt::Display for Blizzard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.direction)
    }
}

impl Blizzard {
    fn new(direction: Direction, position: Position) -> Self {
        Self {
            direction,
            position,
        }
    }

    fn advance(&mut self, map: &Map) {
        self.position = match self.direction {
            Direction::North => {
                let mut position = Position::new(self.position.x, self.position.y - 1);
                if !map.get_cell(position).is_open() {
                    position.y = map.height() - 2
                }
                position
            }
            Direction::South => {
                let mut position = Position::new(self.position.x, self.position.y + 1);
                if !map.get_cell(position).is_open() {
                    position.y = 1
                }
                position
            }
            Direction::East => {
                let mut position = Position::new(self.position.x + 1, self.position.y);
                if !map.get_cell(position).is_open() {
                    position.x = 1
                }
                position
            }
            Direction::West => {
                let mut position = Position::new(self.position.x - 1, self.position.y);
                if !map.get_cell(position).is_open() {
                    position.x = map.width() - 2
                }
                position
            }
        };
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CellType {
    Open,
    Wall,
}

impl From<char> for CellType {
    fn from(v: char) -> Self {
        match v {
            '.' => Self::Open,
            '#' => Self::Wall,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for CellType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Open => write!(f, "."),
            Self::Wall => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
struct Cell {
    r#type: CellType,
}

impl From<char> for Cell {
    fn from(v: char) -> Self {
        Self { r#type: v.into() }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.r#type)
    }
}

impl Cell {
    fn is_open(&self) -> bool {
        self.r#type == CellType::Open
    }
}

#[derive(Debug)]
struct Map {
    cells: Vec<Vec<Cell>>,
}

impl From<Vec<Vec<Cell>>> for Map {
    fn from(cells: Vec<Vec<Cell>>) -> Self {
        Self { cells }
    }
}

impl Map {
    #[inline]
    fn width(&self) -> usize {
        self.cells[0].len()
    }

    #[inline]
    fn height(&self) -> usize {
        self.cells.len()
    }

    #[inline]
    fn get_cell(&self, position: Position) -> &Cell {
        &self.cells[position.y][position.x]
    }

    fn get_start_position(&self) -> Position {
        for (x, cell) in self.cells[0].iter().enumerate() {
            if cell.is_open() {
                return Position::new(x, 0);
            }
        }
        unreachable!()
    }

    fn render(&self, blizzards: impl AsRef<[Blizzard]>, expedition: &Expedition) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let position = Position::new(x, y);

                let mut blizzard_count = 0;
                let mut last_direction = None;
                for blizzard in blizzards.as_ref() {
                    if blizzard.position == position {
                        blizzard_count += 1;
                        last_direction = Some(blizzard.direction);
                    }
                }

                if blizzard_count > 0 {
                    assert!(cell.is_open());
                    assert!(position != expedition.position);

                    if blizzard_count > 1 {
                        print!("{}", blizzard_count);
                    } else {
                        print!("{}", last_direction.unwrap());
                    }
                } else if position == expedition.position {
                    assert!(cell.is_open());
                    print!("E");
                } else {
                    print!("{}", cell);
                }
            }
            println!();
        }
    }
}

fn part1(map: &Map, mut blizzards: Vec<Blizzard>) {
    let start = map.get_start_position();
    let expedition = Expedition::new(start);

    map.render(&blizzards, &expedition);

    let mut minutes = 0;
    loop {
        for blizzard in blizzards.iter_mut() {
            blizzard.advance(map);
        }

        println!();
        map.render(&blizzards, &expedition);

        minutes += 1;
        if minutes >= 10 {
            break;
        }
    }

    println!("Total minutes: {}", minutes + 1);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut blizzards = vec![];
    let values = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let cells = line
                .chars()
                .enumerate()
                .map(|(x, mut ch)| {
                    match ch {
                        '^' | 'v' | '>' | '<' => {
                            blizzards.push(Blizzard::new(ch.into(), Position::new(x, y)));
                            ch = '.';
                        }
                        _ => (),
                    }
                    ch.into()
                })
                .collect::<Vec<_>>();

            Some(cells)
        })
        .collect::<Vec<_>>();

    let map = values.into();
    part1(&map, blizzards);
}
