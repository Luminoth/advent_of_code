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

    fn distance(&self, other: Position) -> usize {
        ((other.x as i64 - self.x as i64).abs() + (other.y as i64 - self.y as i64).abs()) as usize
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

    fn simulate(&self, map: &Map, minutes: usize, max_time: usize) -> usize {
        assert!(map.is_cell_safe(self.position, minutes));

        // if we've gone for too long, we failed
        if minutes >= max_time {
            //println!("max time {}", max_time);
            return usize::MAX;
        }

        // if we're at the start we can only move down
        if self.position.y == 0 {
            let position = Position::new(self.position.x, self.position.y + 1);
            assert!(map.is_cell_safe(position, minutes + 1));
            return Expedition::new(position).simulate(map, minutes + 1, max_time);
        }

        let end = map.get_end_position();

        // if we're at the end, we're done
        if self.position == end {
            println!("exit at {}", minutes);
            return minutes;
        }

        let mut min_time = usize::MAX;

        let current_distance = self.position.distance(end);

        println!(
            "minute {}, distance = {} from {:?} to {:?}",
            minutes, current_distance, self.position, end
        );

        // north
        let position = Position::new(self.position.x, self.position.y - 1);
        let distance = position.distance(end);
        if distance < current_distance && map.is_cell_safe(position, minutes + 1) {
            //println!("test north {} vs {}", current_distance, distance);
            min_time = Expedition::new(position)
                .simulate(map, minutes + 1, max_time)
                .min(min_time);
        }

        // south
        let position = Position::new(self.position.x, self.position.y + 1);
        let distance = position.distance(end);
        if distance < current_distance && map.is_cell_safe(position, minutes + 1) {
            //println!("test south {} vs {}", current_distance, distance);
            min_time = Expedition::new(position)
                .simulate(map, minutes + 1, max_time)
                .min(min_time);
        }

        // east
        let position = Position::new(self.position.x + 1, self.position.y);
        let distance = position.distance(end);
        if distance < current_distance && map.is_cell_safe(position, minutes + 1) {
            //println!("test east {} vs {}", current_distance, distance);
            min_time = Expedition::new(position)
                .simulate(map, minutes + 1, max_time)
                .min(min_time);
        }

        // west
        let position = Position::new(self.position.x - 1, self.position.y);
        let distance = position.distance(end);
        if distance < current_distance && map.is_cell_safe(position, minutes + 1) {
            //println!("test west {} vs {}", current_distance, distance);
            min_time = Expedition::new(position)
                .simulate(map, minutes + 1, max_time)
                .min(min_time);
        }

        // wait
        if map.is_cell_safe(self.position, minutes + 1) {
            //println!("test wait");
            min_time = self
                .simulate(map, minutes + 1, max_time.min(min_time))
                .min(min_time);
        }

        min_time
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
                let x = 1 + (self.position.x - 1 + minutes) % (map.width() - 2);
                Position::new(x, self.position.y)
            }
            Direction::West => {
                assert!(self.position.x > 0);
                let x =
                    1 + (self.position.x as i64 - 1 - minutes as i64) % (map.width() - 2) as i64;
                Position::new(x as usize, self.position.y)
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
    blizzards: Vec<Blizzard>,
}

impl Map {
    fn new(cells: Vec<Vec<Cell>>, blizzards: Vec<Blizzard>) -> Self {
        Self { cells, blizzards }
    }

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

    fn is_cell_safe(&self, position: Position, minutes: usize) -> bool {
        if !self.get_cell(position).is_open() {
            return false;
        }

        for blizzard in &self.blizzards {
            if blizzard.get_position_at(self, minutes) == position {
                return false;
            }
        }

        true
    }

    fn get_start_position(&self) -> Position {
        for (x, cell) in self.cells[0].iter().enumerate() {
            if cell.is_open() {
                return Position::new(x, 0);
            }
        }
        unreachable!()
    }

    fn get_end_position(&self) -> Position {
        for (x, cell) in self.cells[self.cells.len() - 1].iter().enumerate() {
            if cell.is_open() {
                return Position::new(x, self.cells.len() - 1);
            }
        }
        unreachable!()
    }

    #[cfg(feature = "debugvis")]
    fn render(&self, expedition: &Expedition, minutes: usize) {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let position = Position::new(x, y);

                let mut blizzard_count = 0;
                let mut last_direction = None;
                for blizzard in &self.blizzards {
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

fn part1(map: &Map) {
    let start = map.get_start_position();
    let expedition = Expedition::new(start);

    let total = expedition.simulate(map, 0, usize::MAX);
    println!("done: {}", total);
    //assert!(total == ???);
    println!("Total minutes: {}", total + 1);
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

    let map = Map::new(values, blizzards);
    part1(&map);
}
