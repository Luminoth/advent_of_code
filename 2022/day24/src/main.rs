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

    fn get_position_at(&self, map: &Map, minutes: usize) -> Position {
        if minutes == 0 {
            return self.position;
        }

        match self.direction {
            Direction::North => {
                let y =
                    1 + (self.position.y as i64 - 1 - minutes as i64) % (map.height() - 2) as i64;
                Position::new(self.position.x, y as usize)
            }
            Direction::South => {
                let y = 1 + (self.position.y - 1 + minutes) % (map.height() - 2);
                Position::new(self.position.x, y)
            }
            Direction::East => {
                let x =
                    1 + (self.position.x as i64 - 1 + minutes as i64) % (map.width() - 2) as i64;
                Position::new(x as usize, self.position.y)
            }
            Direction::West => {
                let x = 1 + (self.position.x - 1 - minutes) % (map.width() - 2);
                Position::new(x, self.position.y)
            }
        }
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

    fn get_start_position(&self) -> Position {
        for (x, cell) in self.cells[0].iter().enumerate() {
            if cell.is_open() {
                return Position::new(x, 0);
            }
        }
        unreachable!()
    }

    fn render(&self, blizzards: impl AsRef<[Blizzard]>, expedition: &Expedition, minutes: usize) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let position = Position::new(x, y);

                let mut blizzard_count = 0;
                let mut last_direction = None;
                for blizzard in blizzards.as_ref() {
                    let blizzard_position = blizzard.get_position_at(self, minutes);
                    if blizzard_position == position {
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

fn part1(map: &Map, blizzards: impl AsRef<[Blizzard]>) {
    let start = map.get_start_position();
    let expedition = Expedition::new(start);

    let mut minutes = 0;

    println!("== Initial State == ");
    map.render(&blizzards, &expedition, minutes);

    loop {
        minutes += 1;

        println!();
        println!("== After Mintute {} == ", minutes);
        map.render(&blizzards, &expedition, minutes);

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
