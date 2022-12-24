use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd)]
struct Position {
    // y-first for Ord
    y: i64,
    x: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
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
    id: usize,
    position: RefCell<Position>,
    proposed_position: RefCell<Option<Position>>,
}

impl Elf {
    fn new(id: usize, position: Position) -> Self {
        Self {
            id,
            position: RefCell::new(position),
            proposed_position: RefCell::new(None),
        }
    }

    #[inline]
    fn compare_position(&self, other: &Elf) -> Ordering {
        self.position.borrow().cmp(&other.position.borrow())
    }

    #[inline]
    fn compare_proposed_position(&self, other: &Elf) -> Ordering {
        let x = self.proposed_position.borrow();
        let y = other.proposed_position.borrow();

        if y.is_none() {
            if x.is_none() {
                std::cmp::Ordering::Equal
            } else {
                std::cmp::Ordering::Greater
            }
        } else if x.is_none() {
            std::cmp::Ordering::Less
        } else {
            x.unwrap().cmp(&y.unwrap())
        }
    }

    // returns true if no other elf is at the given position
    #[inline]
    fn consider_move_position(&self, elves: impl AsRef<[Elf]>, position: Position) -> bool {
        elves
            .as_ref()
            .binary_search_by(|x| x.position.borrow().cmp(&position))
            .is_err()
    }

    // returns true if we are going to move and no other elf has proposed the same position as us
    #[inline]
    fn consider_proposed_position(&self, elves: impl AsRef<[Elf]>) -> bool {
        if self.proposed_position.borrow().is_none() {
            return false;
        }

        // TODO: this is awful but we have to ignore outself here
        elves
            .as_ref()
            .iter()
            .filter(|x| x.id != self.id)
            .collect::<Vec<_>>()
            .binary_search_by(|x| self.compare_proposed_position(x))
            .is_err()
    }

    fn consider_move_north(&self, elves: impl AsRef<[Elf]>) -> bool {
        let position = self.position.borrow();

        let ne = self.consider_move_position(&elves, position.north_east());
        let n = self.consider_move_position(&elves, position.north());
        let nw = self.consider_move_position(&elves, position.north_west());

        if n && ne && nw {
            *self.proposed_position.borrow_mut() = Some(position.north());
            return true;
        }

        false
    }

    fn consider_move_south(&self, elves: impl AsRef<[Elf]>) -> bool {
        let position = self.position.borrow();

        let se = self.consider_move_position(&elves, position.south_east());
        let s = self.consider_move_position(&elves, position.south());
        let sw = self.consider_move_position(&elves, position.south_west());

        if s && se && sw {
            *self.proposed_position.borrow_mut() = Some(position.south());
            return true;
        }

        false
    }

    fn consider_move_west(&self, elves: impl AsRef<[Elf]>) -> bool {
        let position = self.position.borrow();

        let nw = self.consider_move_position(&elves, position.north_west());
        let w = self.consider_move_position(&elves, position.west());
        let sw = self.consider_move_position(&elves, position.south_west());

        if w && nw && sw {
            *self.proposed_position.borrow_mut() = Some(position.west());
            return true;
        }

        false
    }

    fn consider_move_east(&self, elves: impl AsRef<[Elf]>) -> bool {
        let position = self.position.borrow();

        let ne = self.consider_move_position(&elves, position.north_east());
        let e = self.consider_move_position(&elves, position.east());
        let se = self.consider_move_position(&elves, position.south_east());

        if e && ne && se {
            *self.proposed_position.borrow_mut() = Some(position.east());
            return true;
        }

        false
    }

    fn consider_move(&self, elves: impl AsRef<[Elf]>, directions: &VecDeque<Direction>) {
        *self.proposed_position.borrow_mut() = None;

        for direction in directions.iter() {
            match direction {
                Direction::North => {
                    if self.consider_move_north(&elves) {
                        //println!("{:?} proposes north", self.position);
                        return;
                    }
                }
                Direction::South => {
                    if self.consider_move_south(&elves) {
                        //println!("{:?} proposes south", self.position);
                        return;
                    }
                }
                Direction::West => {
                    if self.consider_move_west(&elves) {
                        //println!("{:?} proposes west", self.position);
                        return;
                    }
                }
                Direction::East => {
                    if self.consider_move_east(&elves) {
                        //println!("{:?} proposes east", self.position);
                        return;
                    }
                }
            }
        }
    }

    fn r#move(&self, elves: impl AsRef<[Elf]>) {
        if self.proposed_position.borrow().is_none() {
            //println!("in a good spot at {:?}", self.position.borrow());
            return;
        }

        if !self.consider_proposed_position(elves) {
            /*println!(
                "{:?} collided at {:?}",
                self.position.borrow(),
                self.proposed_position.borrow().unwrap()
            );*/
            return;
        }

        /*println!(
            "move from {:?} to {:?}",
            self.position.borrow(),
            self.proposed_position.borrow().unwrap()
        );*/
        *self.position.borrow_mut() = self.proposed_position.borrow_mut().take().unwrap();
    }
}

fn get_bounds(elves: impl AsRef<[Elf]>) -> ((i64, i64), (i64, i64)) {
    let elves = elves.as_ref();

    // TODO: this could be better
    let xmin = elves.iter().map(|x| x.position.borrow().x).min().unwrap();
    let xmax = elves.iter().map(|x| x.position.borrow().x).max().unwrap();
    let ymin = elves.iter().map(|x| x.position.borrow().y).min().unwrap();
    let ymax = elves.iter().map(|x| x.position.borrow().y).max().unwrap();

    ((xmin, ymin), (xmax, ymax))
}

// this assumes the elves are sorted by position
#[cfg(feature = "debugvis")]
fn print_elves(elves: impl AsRef<[Elf]>) {
    let elves = elves.as_ref();
    let mut current = 0;

    let ((xmin, ymin), (xmax, ymax)) = get_bounds(&elves);
    for y in ymin..=ymax {
        for x in xmin..=xmax {
            if current >= elves.len() {
                print!(".");
                continue;
            }

            let position = Position::new(x, y);
            let elf = elves.get(current).unwrap();
            if *elf.position.borrow() == position {
                print!("#");
                current += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn part1(mut elves: Vec<Elf>) {
    let mut directions = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);

    elves.sort_by(|x, y| x.compare_position(y));

    #[cfg(feature = "debugvis")]
    {
        println!("== Initial State ==");
        print_elves(&elves);
    }

    let mut rounds = 0;
    loop {
        if rounds >= 3 {
            break;
        }

        // elves are sorted by position
        for elf in elves.iter() {
            elf.consider_move(&elves, &directions);
        }

        elves.sort_by(|x, y| x.compare_proposed_position(y));
        for elf in elves.iter() {
            elf.r#move(&elves);
        }

        directions.rotate_left(1);

        elves.sort_by(|x, y| x.compare_position(y));

        #[cfg(feature = "debugvis")]
        {
            println!();
            println!("== End of Round {} ==", rounds + 1);
            print_elves(&elves);
        }

        rounds += 1;
    }

    #[cfg(feature = "debugvis")]
    println!();

    let ((xmin, ymin), (xmax, ymax)) = get_bounds(&elves);

    let area = (xmax - xmin) * (ymax - ymin);
    let total = area as usize - elves.len();
    //assert!(total == ???);
    println!("Total: {} ({})", total, area);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut count = 0;
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
                        '#' => {
                            let elf = Elf::new(count, Position::new(x as i64, y as i64));
                            count += 1;
                            Some(elf)
                        }
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .flatten()
        .collect::<Vec<_>>();

    part1(values);
}
