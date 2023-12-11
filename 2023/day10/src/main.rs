use std::cell::RefCell;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString)]
enum PipeType {
    #[strum(serialize = ".")]
    Ground,

    #[strum(serialize = "S")]
    Start,

    #[strum(serialize = "|")]
    Vertical,

    #[strum(serialize = "-")]
    Horizontal,

    #[strum(serialize = "L")]
    NEBend,

    #[strum(serialize = "J")]
    NWBend,

    #[strum(serialize = "7")]
    SWBend,

    #[strum(serialize = "F")]
    SEBend,
}

#[derive(Debug)]
struct Pipe {
    r#type: PipeType,
    coords: (usize, usize),

    min_distance: RefCell<Option<usize>>,
}

impl From<(usize, usize, char)> for Pipe {
    fn from(v: (usize, usize, char)) -> Self {
        Self {
            r#type: PipeType::from_str(&v.2.to_string()).unwrap(),
            coords: (v.0, v.1),
            min_distance: RefCell::new(None),
        }
    }
}

impl Pipe {
    fn resolve_type(&self, _grid: &[Vec<Pipe>], cheat: PipeType) -> PipeType {
        if self.r#type != PipeType::Start {
            return self.r#type;
        }

        // TODO:

        cheat
    }

    fn is_start(&self) -> bool {
        self.r#type == PipeType::Start
    }

    fn start_direction(&self, grid: &[Vec<Pipe>], start_cheat: PipeType) -> Direction {
        let r#type = self.resolve_type(grid, start_cheat);
        match r#type {
            PipeType::Vertical => Direction::Up,
            PipeType::Horizontal => Direction::Right,
            PipeType::NEBend | PipeType::SEBend => Direction::Left,
            PipeType::NWBend | PipeType::SWBend => Direction::Right,
            _ => unreachable!("unexpected start pipe: {:?}", r#type),
        }
    }

    fn reverse_direction(&self, grid: &[Vec<Pipe>], start_cheat: PipeType) -> Direction {
        let r#type = self.resolve_type(grid, start_cheat);
        match r#type {
            PipeType::Vertical => Direction::Down,
            PipeType::Horizontal => Direction::Left,
            PipeType::NEBend | PipeType::NWBend => Direction::Down,
            PipeType::SEBend | PipeType::SWBend => Direction::Up,
            _ => unreachable!("unexpected reverse pipe: {:?}", r#type),
        }
    }

    fn next<'a>(
        &self,
        grid: &'a [Vec<Pipe>],
        start_cheat: PipeType,
        direction: Direction,
    ) -> (&'a Pipe, Direction) {
        let r#type = self.resolve_type(grid, start_cheat);
        match r#type {
            PipeType::Vertical => match direction {
                Direction::Up => (
                    grid.get(self.coords.1 - 1)
                        .unwrap()
                        .get(self.coords.0)
                        .unwrap(),
                    Direction::Up,
                ),
                Direction::Down => (
                    grid.get(self.coords.1 + 1)
                        .unwrap()
                        .get(self.coords.0)
                        .unwrap(),
                    Direction::Down,
                ),
                _ => unreachable!("invalid vertical direction {:?}", direction),
            },
            PipeType::Horizontal => match direction {
                Direction::Left => (
                    grid.get(self.coords.1)
                        .unwrap()
                        .get(self.coords.0 - 1)
                        .unwrap(),
                    Direction::Left,
                ),
                Direction::Right => (
                    grid.get(self.coords.1)
                        .unwrap()
                        .get(self.coords.0 + 1)
                        .unwrap(),
                    Direction::Right,
                ),
                _ => unreachable!("invalid horizontal direction {:?}", direction),
            },
            PipeType::NEBend => match direction {
                Direction::Down => (
                    grid.get(self.coords.1)
                        .unwrap()
                        .get(self.coords.0 + 1)
                        .unwrap(),
                    Direction::Right,
                ),
                Direction::Left => (
                    grid.get(self.coords.1 - 1)
                        .unwrap()
                        .get(self.coords.0)
                        .unwrap(),
                    Direction::Up,
                ),
                _ => unreachable!("invalid nebend direction {:?}", direction),
            },
            PipeType::SEBend => match direction {
                Direction::Up => (
                    grid.get(self.coords.1)
                        .unwrap()
                        .get(self.coords.0 + 1)
                        .unwrap(),
                    Direction::Right,
                ),
                Direction::Left => (
                    grid.get(self.coords.1 + 1)
                        .unwrap()
                        .get(self.coords.0)
                        .unwrap(),
                    Direction::Down,
                ),
                _ => unreachable!("invalid sebend direction {:?}", direction),
            },
            PipeType::NWBend => match direction {
                Direction::Down => (
                    grid.get(self.coords.1)
                        .unwrap()
                        .get(self.coords.0 - 1)
                        .unwrap(),
                    Direction::Left,
                ),
                Direction::Right => (
                    grid.get(self.coords.1 - 1)
                        .unwrap()
                        .get(self.coords.0)
                        .unwrap(),
                    Direction::Up,
                ),
                _ => unreachable!("invalid nwbend direction {:?}", direction),
            },
            PipeType::SWBend => match direction {
                Direction::Up => (
                    grid.get(self.coords.1)
                        .unwrap()
                        .get(self.coords.0 - 1)
                        .unwrap(),
                    Direction::Left,
                ),
                Direction::Right => (
                    grid.get(self.coords.1 + 1)
                        .unwrap()
                        .get(self.coords.0)
                        .unwrap(),
                    Direction::Down,
                ),
                _ => unreachable!("invalid swbend direction {:?}", direction),
            },
            _ => unreachable!("unexpected pipe: {:?}", r#type),
        }
    }
}

fn part1(grid: &[Vec<Pipe>], start_cheat: PipeType) {
    let start = grid
        .iter()
        .find_map(|row| row.iter().find(|pipe| pipe.r#type == PipeType::Start))
        .unwrap();

    //println!("forward");
    let mut node = start;
    let mut direction = start.start_direction(grid, start_cheat);
    let mut distance = 0;
    loop {
        (node, direction) = node.next(grid, start_cheat, direction);
        //println!("next node: {:?}", node);
        if node.is_start() {
            break;
        }

        distance += 1;

        let node_distance = (*node.min_distance.borrow()).unwrap_or(usize::MAX);
        node.min_distance.replace(Some(node_distance.min(distance)));
    }

    //println!("reverse");
    let mut node = start;
    let mut direction = start.reverse_direction(grid, start_cheat);
    let mut distance = 0;
    loop {
        (node, direction) = node.next(grid, start_cheat, direction);
        //println!("next node: {:?}", node);
        if node.is_start() {
            break;
        }

        distance += 1;

        let node_distance = (*node.min_distance.borrow()).unwrap_or(usize::MAX);
        node.min_distance.replace(Some(node_distance.min(distance)));
    }

    let max_distance = grid
        .iter()
        .flat_map(|row| row.iter().filter_map(|pipe| (*pipe.min_distance.borrow())))
        .max()
        .unwrap();

    assert!(max_distance == 6800);
    println!("Max distance: {}", max_distance);
}

fn part2(_grid: &[Vec<Pipe>]) {
    // TODO:
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Pipe::from((x, y, c)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    //let start_cheat = PipeType::SEBend;
    let start_cheat = PipeType::NEBend;
    part1(&grid, start_cheat);
    part2(&grid);
}
