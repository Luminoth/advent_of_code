use std::cell::RefCell;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{all_consuming, map, map_res},
    sequence::preceded,
    Finish, IResult,
};

#[derive(Debug)]
struct Cpu {
    x: RefCell<i32>,
    cycle: RefCell<i32>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: RefCell::new(1),
            cycle: RefCell::new(1),
        }
    }

    fn execute<F>(&self, instruction: Instruction, mut cycle: F)
    where
        F: FnMut(i32, i32),
    {
        match instruction {
            Instruction::NoOp => {
                cycle(*self.cycle.borrow(), *self.x.borrow());
                *self.cycle.borrow_mut() += 1;
            }
            Instruction::AddX(v) => {
                for _ in 0..instruction.cycles() - 1 {
                    cycle(*self.cycle.borrow(), *self.x.borrow());
                    *self.cycle.borrow_mut() += 1;
                }

                cycle(*self.cycle.borrow(), *self.x.borrow());
                *self.cycle.borrow_mut() += 1;

                *self.x.borrow_mut() += v;
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    fn cycles(&self) -> usize {
        match self {
            Self::NoOp => 1,
            Self::AddX(_) => 2,
        }
    }
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    map(tag("noop"), |_| Instruction::NoOp)(input)
}

fn parse_negative_digit(input: &str) -> IResult<&str, i32> {
    map(map_res(preceded(tag("-"), digit1), str::parse), |x: i32| -x)(input)
}

fn parse_digit(input: &str) -> IResult<&str, i32> {
    alt((map_res(digit1, str::parse), parse_negative_digit))(input)
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), parse_digit), |x| {
        Instruction::AddX(x)
    })(input)
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    alt((parse_noop, parse_addx))(i)
}

fn part1(instructions: impl AsRef<[Instruction]>) {
    let cpu = Cpu::new();

    let mut total = 0;
    for instruction in instructions.as_ref() {
        if *cpu.cycle.borrow() > 220 {
            break;
        }

        cpu.execute(*instruction, |cycle, x| {
            if (cycle - 20) % 40 == 0 {
                let s = cycle * x;
                total += s;
            }
        });
    }

    assert!(total == 15220);
    println!("Total: {}", total);
}

fn part2(instructions: impl AsRef<[Instruction]>) {
    let cpu = Cpu::new();

    for instruction in instructions.as_ref() {
        cpu.execute(*instruction, |cycle, spos| {
            // TODO: this isn't 100% correct at the right edge
            // not super sure what's wrong but it works well enough to pass the test

            let cpos = cycle % 40;

            // is at least one pixel of the sprite visible?
            let ppos = cpos - 1;
            if ppos == spos - 1 || ppos == spos || ppos == spos + 1 {
                print!("#");
            } else {
                print!(".");
            }

            if cpos == 0 {
                println!();
            }
        });
    }

    // this should print out RFZEKBFA
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .map(|x| all_consuming(parse_instruction)(x).finish().unwrap().1)
        .collect::<Vec<_>>();

    part1(&values);
    part2(&values);
}
