use std::collections::HashSet;

use rayon::prelude::*;
use regex::Regex;

#[derive(Debug)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<isize> for Instruction {
    #[inline]
    fn from(opcode: isize) -> Self {
        match opcode {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
struct Registers {
    a: isize,
    b: isize,
    c: isize,
}

impl Registers {
    #[inline]
    fn reset(&mut self, a: isize) {
        self.a = a;
        self.b = 0;
        self.c = 0;
    }
}

#[derive(Debug, Default, Clone)]
struct Computer {
    registers: Registers,
    ip: usize,

    output: Vec<isize>,

    halt_checker: HashSet<(Registers, usize)>,
}

impl Computer {
    #[inline]
    fn reset(&mut self, a: isize) {
        if a % 10_000_000 == 0 {
            println!("resetting {}", a);
        }

        self.registers.reset(a);
        self.ip = 0;

        self.output.clear();
        self.halt_checker.clear();
    }

    #[inline]
    fn combo(&self, operand: isize) -> isize {
        match operand {
            0..=3 => operand,
            4 => self.registers.a,
            5 => self.registers.b,
            6 => self.registers.c,
            _ => unreachable!(),
        }
    }

    fn execute(&mut self, instruction: Instruction, operand: isize) {
        match instruction {
            Instruction::Adv => self.registers.a /= 2_isize.pow(self.combo(operand) as u32),
            Instruction::Bxl => self.registers.b ^= operand,
            Instruction::Bst => self.registers.b = self.combo(operand) % 8,
            Instruction::Jnz => {
                if self.registers.a != 0 {
                    self.ip = operand as usize;
                }
            }
            Instruction::Bxc => self.registers.b ^= self.registers.c,
            Instruction::Out => self.output.push(self.combo(operand) % 8),
            Instruction::Bdv => {
                self.registers.b = self.registers.a / 2_isize.pow(self.combo(operand) as u32)
            }
            Instruction::Cdv => {
                self.registers.c = self.registers.a / 2_isize.pow(self.combo(operand) as u32)
            }
        }
    }

    fn step(&mut self, program: &[isize]) -> anyhow::Result<bool> {
        if !self.halt_checker.insert((self.registers.clone(), self.ip)) {
            anyhow::bail!("will not halt");
        } else {
            //println!("checker: {:?}", self.halt_checker);
        }

        if self.ip >= program.len() {
            return Ok(false);
        }

        let instruction = program[self.ip].into();
        self.ip += 1;

        let operand = program[self.ip];
        self.ip += 1;

        self.execute(instruction, operand);

        Ok(true)
    }
}

fn part1(mut computer: Computer, program: &[isize]) {
    while computer.step(program).unwrap() {}

    let output = computer
        .output
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<_>>()
        .join(",");
    assert!(output == "1,5,0,3,7,3,0,3,1");
    println!("{}", output);
}

fn part2(program: &[isize]) {
    let start = 10_908_900_000;
    let end = 100_908_900_000;

    (start..end).into_par_iter().for_each(|initial| {
        let mut computer = Computer::default();
        computer.reset(initial);

        let halted = match computer.step(program) {
            Ok(result) => !result,
            Err(err) => {
                println!("{}", err);
                return;
            }
        };

        for i in 0..computer.output.len() {
            //println!("checking {}", i);
            if i >= program.len() || computer.output[i] != program[i] {
                /*if i >= program.len() {
                    println!("invalid output len: {} vs {}", i, program.len());
                } else {
                    println!("invalid output: {} vs {}", computer.output[i], program[i]);
                }*/
                return;
            }
        }
        //println!("out");

        if halted && computer.output.len() == program.len() {
            panic!("found it: {}", computer.registers.a);
        }
    });

    /*loop {
        let halted = match computer.step(program) {
            Ok(result) => !result,
            Err(err) => {
                println!("{}", err);

                initial += 1;
                computer.reset(initial);
                continue;
            }
        };

        for i in 0..computer.output.len() {
            //println!("checking {}", i);
            if i >= program.len() || computer.output[i] != program[i] {
                /*if i >= program.len() {
                    println!("invalid output len: {} vs {}", i, program.len());
                } else {
                    println!("invalid output: {} vs {}", computer.output[i], program[i]);
                }*/
                initial += 1;
                computer.reset(initial);
                break;
            }
        }
        //println!("out");

        if halted && computer.output.len() == program.len() {
            break;
        }
    }

    println!("found it: {}", computer.registers.a);*/
}

fn main() {
    let input = include_str!("../input.txt");

    let (registers, program) = input.split_once("\n\n").unwrap();

    let mut computer = Computer::default();
    let mut registers = registers.split('\n');

    let regex = Regex::new(r"Register A: (?P<v>\d+)").unwrap();
    let caps: regex::Captures<'_> = regex.captures(registers.next().unwrap()).unwrap();
    computer.registers.a = caps["v"].parse().unwrap();

    let regex = Regex::new(r"Register B: (?P<v>\d+)").unwrap();
    let caps: regex::Captures<'_> = regex.captures(registers.next().unwrap()).unwrap();
    computer.registers.b = caps["v"].parse().unwrap();

    let regex = Regex::new(r"Register C: (?P<v>\d+)").unwrap();
    let caps: regex::Captures<'_> = regex.captures(registers.next().unwrap()).unwrap();
    computer.registers.c = caps["v"].parse().unwrap();

    let program = program
        .trim()
        .split_once(' ')
        .unwrap()
        .1
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect::<Vec<_>>();

    part1(computer, &program);
    part2(&program);
}
