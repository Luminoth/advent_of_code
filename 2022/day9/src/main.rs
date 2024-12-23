use std::cell::RefCell;
use std::fmt;

#[derive(Debug, Default, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    #[inline]
    fn distance(&self, other: &Position) -> (i32, i32) {
        (other.x - self.x, other.y - self.y)
    }

    #[inline]
    fn is_touching(&self, other: &Position) -> bool {
        let distance = self.distance(other);
        distance.0.abs() <= 1 && distance.1.abs() <= 1
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Direction {
    fn from(v: &str) -> Self {
        match v {
            "U" => Self::Up,
            "D" => Self::Down,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    direction: Direction,
    amount: usize,
}

impl Instruction {
    fn new(direction: Direction, amount: usize) -> Self {
        Self { direction, amount }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let d = match self.direction {
            Direction::Up => 'U',
            Direction::Down => 'D',
            Direction::Left => 'L',
            Direction::Right => 'R',
        };
        write!(f, "{} {}", d, self.amount)
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<usize>>,

    knots: Vec<RefCell<Position>>,
    start: Position,
}

impl Grid {
    fn new(width: usize, height: usize, start: Position, knot_count: usize) -> Self {
        let mut grid = Vec::with_capacity(height);
        for _ in 0..height {
            grid.push(vec![0; width]);
        }
        grid[start.y as usize][start.x as usize] = 1;

        #[allow(clippy::let_and_return)]
        let this = Self {
            grid,
            knots: vec![RefCell::new(start); knot_count],
            start,
        };

        #[cfg(feature = "debugvis")]
        {
            println!("== Initial State == ");
            println!("{}", this);
        }

        this
    }

    fn update(&mut self, instruction: Instruction) {
        let mut remaining = instruction.amount;
        loop {
            if remaining < 1 {
                break;
            }

            // move the head according to the instruction
            {
                let mut head = self.knots.first().unwrap().borrow_mut();
                match instruction.direction {
                    Direction::Up => head.y += 1,
                    Direction::Down => head.y -= 1,
                    Direction::Left => head.x -= 1,
                    Direction::Right => head.x += 1,
                }
            }
            remaining -= 1;

            // move the other knots to follow the knot in front of them
            for (i, knot) in self.knots.iter().skip(1).enumerate() {
                let parent = self.knots.get(i).unwrap().borrow();

                // if we're touching our parent we don't need to move
                if knot.borrow().is_touching(&parent) {
                    #[cfg(feature = "debugvis")]
                    println!("{}", self);
                    continue;
                }

                let distance = knot.borrow().distance(&parent);
                {
                    let mut knot = knot.borrow_mut();
                    knot.x += distance.0.clamp(-1, 1);
                    knot.y += distance.1.clamp(-1, 1);
                }

                let knot = knot.borrow();
                assert!(knot.is_touching(&parent));

                // track the cells the tail touches
                if i == self.knots.len() - 2 {
                    self.grid[knot.y as usize][knot.x as usize] += 1;
                }

                #[cfg(feature = "debugvis")]
                println!("{}", self);
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let head = self.knots[0].borrow();

        for (y, row) in self.grid.iter().enumerate() {
            let y = y as i32;
            for (x, v) in row.iter().enumerate() {
                let x = x as i32;

                let ch = if x == head.x && y == head.y {
                    'H'
                } else {
                    let mut t = None;
                    for (i, knot) in self.knots.iter().skip(1).enumerate() {
                        let knot = knot.borrow();

                        if x == knot.x && y == knot.y {
                            t = Some(i + 1);
                            break;
                        }
                    }

                    if let Some(t) = t {
                        t.to_string().chars().next().unwrap()
                    } else if x == self.start.x && y == self.start.y {
                        assert!(self.grid[y as usize][x as usize] >= 1);
                        's'
                    } else if *v == 0 {
                        '.'
                    } else {
                        assert!(*v >= 1);
                        '#'
                    }
                };

                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn part1(width: usize, height: usize, start: Position, instructions: impl AsRef<[Instruction]>) {
    let mut grid = Grid::new(width, height, start, 2);

    for instruction in instructions.as_ref() {
        #[cfg(feature = "debugvis")]
        println!("== {} == ", instruction);
        grid.update(*instruction);
    }

    let count = grid.grid.iter().flatten().filter(|&&x| x >= 1).count();
    assert!(count == 6503);
    println!("Visited at least once count: {}", count);
}

fn part2(width: usize, height: usize, start: Position, instructions: impl AsRef<[Instruction]>) {
    let mut grid = Grid::new(width, height, start, 10);

    for instruction in instructions.as_ref() {
        #[cfg(feature = "debugvis")]
        println!("== {} == ", instruction);
        grid.update(*instruction);
    }

    let count = grid.grid.iter().flatten().filter(|&&x| x >= 1).count();
    assert!(count == 2724);
    println!("Visited at least once count: {}", count);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut y = 0;
    let mut max_y = 0;
    let mut min_y = 0;
    let mut x = 0;
    let mut max_x = 0;
    let mut min_x = 0;

    let values = input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let (d, a) = line.split_once(' ').unwrap();
            let i = Instruction::new(d.into(), a.parse().unwrap());

            match i.direction {
                Direction::Up => y += i.amount as i32,
                Direction::Down => y -= i.amount as i32,
                Direction::Left => x -= i.amount as i32,
                Direction::Right => x += i.amount as i32,
            }

            max_y = y.max(max_y);
            min_y = y.min(min_y);
            max_x = x.max(max_x);
            min_x = x.min(min_x);

            Some(i)
        })
        .collect::<Vec<_>>();

    let height = ((max_y - min_y).abs() + 1) as usize;
    let width = ((max_x - min_x).abs() + 1) as usize;
    let start = Position::new(min_x.abs(), min_y.abs());

    part1(width, height, start, &values);
    part2(width, height, start, &values);
}
