use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;

const CHAMBER_WIDTH: i64 = 7;
const MAX_ROUNDS: usize = 3; //2022;

#[derive(Debug, Copy, Clone)]
enum JetDirection {
    Left,
    Right,
}

#[derive(Debug)]
struct Rock {
    // top-left corner
    x: i64,
    y: i64,

    width: i64,
    height: i64,

    // TODO: this could just be a single u8
    // or a fixed array of 9 elements
    pixels: Vec<bool>,
}

impl fmt::Display for Rock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = (y * self.width) + x;
                write!(f, "{}", if self.pixels[idx as usize] { '#' } else { ' ' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Rock {
    fn new_horizontal(bottom: i64) -> Self {
        let height = 1;
        Self {
            x: 2,
            y: bottom + 3 + height,
            width: 4,
            height,
            pixels: vec![true; 4],
        }
    }

    fn new_cross(bottom: i64) -> Self {
        let height = 3;
        Self {
            x: 2,
            y: bottom + 3 + height,
            width: 3,
            height,
            pixels: vec![false, true, false, true, true, true, false, true, false],
        }
    }

    fn new_el(bottom: i64) -> Self {
        let height = 3;
        Self {
            x: 2,
            y: bottom + 3 + height,
            width: 3,
            height,
            pixels: vec![false, false, true, false, false, true, true, true, true],
        }
    }

    fn new_vertical(bottom: i64) -> Self {
        let height = 4;
        Self {
            x: 2,
            y: bottom + 3 + height,
            width: 1,
            height,
            pixels: vec![true; 4],
        }
    }

    fn new_square(bottom: i64) -> Self {
        let height = 2;
        Self {
            x: 2,
            y: bottom + 3 + height,
            width: 2,
            height,
            pixels: vec![true; 4],
        }
    }

    fn new(round: usize, bottom: i64) -> Self {
        match round % 5 {
            0 => Self::new_horizontal(bottom),
            1 => Self::new_cross(bottom),
            2 => Self::new_el(bottom),
            3 => Self::new_vertical(bottom),
            4 => Self::new_square(bottom),
            _ => unreachable!(),
        }
    }

    fn top(&self) -> i64 {
        self.y
    }

    fn bottom(&self) -> i64 {
        self.y + self.height - 1
    }

    fn left(&self) -> i64 {
        self.x
    }

    fn right(&self) -> i64 {
        self.x + self.width - 1
    }

    // render the rock to a buffer
    fn render(&self, mut _buffer: impl AsMut<[char]>, _width: i64, _height: i64, _falling: bool) {
        /*for y in 0..self.height {
            for x in 0..self.width {
                let idx = ((self.height - y - 1) * self.width) + x;

                let bx = self.left() + x;
                let by = height - (self.bottom() + y) - 1;
                let bidx = (by * width) + bx;

                if self.pixels[idx as usize] {
                    buffer.as_mut()[bidx as usize] = if falling { '@' } else { '#' };
                }
            }
        }*/
    }

    fn push(&mut self, direction: JetDirection, chamber: &Chamber) {
        match direction {
            JetDirection::Left => {
                if self.left() > 0 {
                    self.x -= 1;
                }

                if chamber.intersects_rock(self) {
                    self.x += 1;
                }
            }
            JetDirection::Right => {
                if self.right() < CHAMBER_WIDTH {
                    self.x += 1;
                }

                if chamber.intersects_rock(self) {
                    self.x -= 1;
                }
            }
        }
    }

    fn fall(&mut self, chamber: &Chamber) -> bool {
        if self.bottom() <= 1 {
            println!("hit the ground");
            return false;
        }

        self.y -= 1;

        if chamber.intersects_rock(self) {
            println!("hit a rock");

            self.y += 1;
            return false;
        }

        true
    }

    // TODO: this still needs some work
    fn intersects(&self, other: &Rock) -> bool {
        if self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
        {
            println!("simple intersection");

            // bounding box intersects, check pixels
            let minx = self.left().min(other.left());
            let maxx = self.right().max(other.right());
            let miny = self.top().min(other.top());
            let maxy = self.bottom().max(other.bottom());

            for ay in miny..maxy {
                let yoff = ay - self.top();
                if yoff < 0 || yoff >= self.height {
                    continue;
                }

                for ax in minx..maxx {
                    let xoff = ax - self.left();
                    if xoff < 0 || xoff >= self.width {
                        continue;
                    }

                    let ai = (yoff * self.width) + xoff;

                    for by in miny..maxy {
                        let yoff = by - other.top();
                        if yoff < 0 || yoff >= other.height {
                            continue;
                        }

                        for bx in minx..maxx {
                            let xoff = bx - other.left();
                            if xoff < 0 || xoff >= other.width {
                                continue;
                            }

                            let bi = (yoff * other.width) + xoff;

                            if self.pixels[ai as usize] && other.pixels[bi as usize] {
                                println!(
                                    "pixel intersection at ({}, {}) vs ({}, {})",
                                    ax, ay, bx, by
                                );
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
    }
}

#[derive(Debug)]
struct Chamber {
    rocks: RefCell<VecDeque<RefCell<Rock>>>,
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (buffer, width, height) = self.render();
        for y in 0..height {
            write!(f, "|")?;
            for x in 0..width {
                let idx = (y * width) + x;
                write!(f, "{}", buffer[idx as usize])?;
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "+-------+")?;

        Ok(())
    }
}

impl Chamber {
    fn new() -> Self {
        Self {
            rocks: RefCell::new(VecDeque::new()),
        }
    }

    // render the chamber to a buffer
    // returns the buffer and the width and height of the buffer
    fn render(&self) -> (Vec<char>, i64, i64) {
        let height = self.height() + 3;
        let mut buffer = vec!['.'; (height * CHAMBER_WIDTH) as usize];

        // render the rocks
        for rock in self.rocks.borrow().iter() {
            rock.borrow()
                .render(&mut buffer, CHAMBER_WIDTH, height, false);
        }

        (buffer, CHAMBER_WIDTH, height)
    }

    fn spawn_rock(&self, round: usize) {
        let height = self.height();
        self.rocks
            .borrow_mut()
            .push_front(RefCell::new(Rock::new(round, height)));
    }

    fn height(&self) -> i64 {
        let rocks = self.rocks.borrow();
        if rocks.len() == 0 {
            return 0;
        }

        rocks.iter().map(|x| x.borrow().top()).max().unwrap()
    }

    fn intersects_rock(&self, rock: &Rock) -> bool {
        for other in self.rocks.borrow().iter().skip(1) {
            if rock.intersects(&other.borrow()) {
                return true;
            }
        }
        false
    }
}

fn part1(jets: impl AsRef<[JetDirection]>) {
    let jets = jets.as_ref();

    let chamber = Chamber::new();

    let mut rock_counter = 0;
    let mut jet_counter = 0;

    loop {
        chamber.spawn_rock(rock_counter);

        let rocks = chamber.rocks.borrow();
        let rock = rocks.front().unwrap();
        println!("spawned rock: {:?}", rock);

        loop {
            let jet = jets.get(jet_counter % jets.len()).unwrap();
            jet_counter += 1;

            println!("push {:?}", jet);
            rock.borrow_mut().push(*jet, &chamber);

            if !rock.borrow_mut().fall(&chamber) {
                //println!("landed");
                break;
            }

            println!("{:?}", rock);
        }
        println!("final: {:?}", rock);

        rock_counter += 1;
        if rock_counter >= MAX_ROUNDS {
            break;
        }
    }

    println!("{}", chamber);

    let height = chamber.height();
    println!(
        "Final height: {} ({} rocks)",
        height,
        chamber.rocks.borrow().len()
    );
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .trim()
        .chars()
        .map(|x| match x {
            '<' => JetDirection::Left,
            '>' => JetDirection::Right,
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    part1(values);
}
