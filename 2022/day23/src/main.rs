use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    #[inline]
    fn get_index(&self, width: i64) -> i64 {
        (self.y * width) + self.x
    }

    #[inline]
    fn north_east(&self) -> Position {
        Position::new(self.x + 1, self.y - 1)
    }

    #[inline]
    fn north(&self) -> Position {
        Position::new(self.x, self.y - 1)
    }

    #[inline]
    fn north_west(&self) -> Position {
        Position::new(self.x - 1, self.y - 1)
    }

    #[inline]
    fn east(&self) -> Position {
        Position::new(self.x + 1, self.y)
    }

    #[inline]
    fn west(&self) -> Position {
        Position::new(self.x - 1, self.y)
    }

    #[inline]
    fn south_east(&self) -> Position {
        Position::new(self.x + 1, self.y + 1)
    }

    #[inline]
    fn south(&self) -> Position {
        Position::new(self.x, self.y + 1)
    }

    #[inline]
    fn south_west(&self) -> Position {
        Position::new(self.x - 1, self.y + 1)
    }
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
struct Elf {
    position: RefCell<Position>,
    proposed_position: RefCell<Option<Position>>,
}

impl Elf {
    fn new(position: Position) -> Self {
        Self {
            position: RefCell::new(position),
            proposed_position: RefCell::new(None),
        }
    }

    #[inline]
    fn get_index(&self, width: i64) -> i64 {
        self.position.borrow().get_index(width)
    }

    #[inline]
    fn compare_position(&self, other: &Elf, width: i64) -> Ordering {
        self.get_index(width).cmp(&other.get_index(width))
    }

    #[inline]
    fn compare_proposed_position(&self, other: &Elf, width: i64) -> Ordering {
        let x = self.proposed_position.borrow();
        let y = other.proposed_position.borrow();

        if x.is_none() {
            if y.is_none() {
                return std::cmp::Ordering::Equal;
            }
            return std::cmp::Ordering::Less;
        } else if y.is_none() {
            return std::cmp::Ordering::Greater;
        }

        x.unwrap()
            .get_index(width)
            .cmp(&y.unwrap().get_index(width))
    }

    // returns true if no other elf is at the given position
    #[inline]
    fn consider_move_position(
        &self,
        elves: impl AsRef<[Elf]>,
        width: i64,
        position: Position,
    ) -> bool {
        elves
            .as_ref()
            .binary_search_by(|x| x.get_index(width).cmp(&position.get_index(width)))
            .is_err()
    }

    // returns true if we are going to move and no other elf has proposed the same position as us
    #[inline]
    fn consider_proposed_position(&self, elves: impl AsRef<[Elf]>, width: i64) -> bool {
        if self.proposed_position.borrow().is_none() {
            return false;
        }

        elves
            .as_ref()
            .binary_search_by(|x| self.compare_proposed_position(x, width))
            .is_err()
    }

    fn consider_move_north(&self, elves: impl AsRef<[Elf]>, width: i64) -> bool {
        let position = self.position.borrow();

        let ne = self.consider_move_position(&elves, width, position.north_east());
        let n = self.consider_move_position(&elves, width, position.north());
        let nw = self.consider_move_position(&elves, width, position.north_west());

        if !n && !ne && !nw {
            *self.proposed_position.borrow_mut() = Some(position.north());
            return true;
        }

        false
    }

    fn consider_move_south(&self, elves: impl AsRef<[Elf]>, width: i64) -> bool {
        let position = self.position.borrow();

        let se = self.consider_move_position(&elves, width, position.south_east());
        let s = self.consider_move_position(&elves, width, position.south());
        let sw = self.consider_move_position(&elves, width, position.south_west());

        if !s && !se && !sw {
            *self.proposed_position.borrow_mut() = Some(position.south());
            return true;
        }

        false
    }

    fn consider_move_west(&self, elves: impl AsRef<[Elf]>, width: i64) -> bool {
        let position = self.position.borrow();

        let nw = self.consider_move_position(&elves, width, position.north_west());
        let w = self.consider_move_position(&elves, width, position.west());
        let sw = self.consider_move_position(&elves, width, position.south_west());

        if !w && !nw && !sw {
            *self.proposed_position.borrow_mut() = Some(position.west());
            return true;
        }

        false
    }

    fn consider_move_east(&self, elves: impl AsRef<[Elf]>, width: i64) -> bool {
        let position = self.position.borrow();

        let ne = self.consider_move_position(&elves, width, position.north_east());
        let e = self.consider_move_position(&elves, width, position.east());
        let se = self.consider_move_position(&elves, width, position.south_east());

        if !e && !ne && !se {
            *self.proposed_position.borrow_mut() = Some(position.east());
            return true;
        }

        false
    }

    fn consider_move(
        &self,
        elves: impl AsRef<[Elf]>,
        width: i64,
        directions: &VecDeque<Direction>,
    ) {
        *self.proposed_position.borrow_mut() = None;

        for direction in directions.iter() {
            match direction {
                Direction::North => {
                    if self.consider_move_north(&elves, width) {
                        return;
                    }
                }
                Direction::South => {
                    if self.consider_move_south(&elves, width) {
                        return;
                    }
                }
                Direction::West => {
                    if self.consider_move_west(&elves, width) {
                        return;
                    }
                }
                Direction::East => {
                    if self.consider_move_east(&elves, width) {
                        return;
                    }
                }
            }
        }
    }

    fn r#move(&self, elves: impl AsRef<[Elf]>, width: i64) {
        if self.proposed_position.borrow().is_none() {
            return;
        }

        if !self.consider_proposed_position(elves, width) {
            return;
        }

        println!("move");
        *self.position.borrow_mut() = self.proposed_position.borrow_mut().take().unwrap();
    }
}

fn part1(mut elves: Vec<Elf>, width: i64) {
    let mut directions = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    let mut rounds = 0;
    loop {
        if rounds >= 10 {
            break;
        }

        elves.sort_by(|x, y| x.compare_position(y, width));
        for elf in elves.iter() {
            elf.consider_move(&elves, width, &directions);
        }

        elves.sort_by(|x, y| x.compare_proposed_position(y, width));
        for elf in elves.iter() {
            elf.r#move(&elves, width);
        }

        directions.rotate_left(1);

        println!("== End of Round {} ==", rounds + 1);

        rounds += 1;
    }

    let xmin = elves.iter().map(|x| x.position.borrow().x).min().unwrap();
    let xmax = elves.iter().map(|x| x.position.borrow().x).max().unwrap();
    let ymin = elves.iter().map(|x| x.position.borrow().y).min().unwrap();
    let ymax = elves.iter().map(|x| x.position.borrow().y).max().unwrap();

    let area = (xmax - xmin) * (ymax - ymin);
    let total = area as usize - elves.len();
    //assert!(total == ???);
    println!("Total: {} ({})", total, area);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .enumerate()
        .filter_map(|(y, line)| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            Some(
                line.chars()
                    .enumerate()
                    .filter_map(|(x, ch)| match ch {
                        '.' => None,
                        '#' => Some(Elf::new(Position::new(x as i64, y as i64))),
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect::<Vec<_>>();

    let width = input.lines().nth(1).unwrap().trim().len();

    part1(values, width as i64);
}
