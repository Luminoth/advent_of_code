use std::collections::HashSet;

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
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
}
