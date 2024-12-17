use regex::Regex;

#[derive(Debug, Default, Clone)]
struct Computer {
    a: isize,
    b: isize,
    c: isize,

    ip: usize,

    output: Vec<String>,
}

impl Computer {
    fn combo(&self, operand: isize) -> isize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn execute(&mut self, program: &[isize]) -> bool {
        if self.ip >= program.len() {
            return false;
        }

        let instruction = program[self.ip].into();
        self.ip += 1;

        let operand = program[self.ip];
        self.ip += 1;

        match instruction {
            Instruction::Adv => self.a /= 2_isize.pow(self.combo(operand) as u32),
            Instruction::Bxl => self.b ^= operand,
            Instruction::Bst => self.b = self.combo(operand) % 8,
            Instruction::Jnz => {
                if self.a != 0 {
                    self.ip = operand as usize;
                }
            }
            Instruction::Bxc => self.b ^= self.c,
            Instruction::Out => self.output.push((self.combo(operand) % 8).to_string()),
            Instruction::Bdv => self.b = self.a / 2_isize.pow(self.combo(operand) as u32),
            Instruction::Cdv => self.c = self.a / 2_isize.pow(self.combo(operand) as u32),
        }

        true
    }
}

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

fn part1(mut computer: Computer, program: &[isize]) {
    while computer.execute(program) {}

    let output = computer.output.join(",");
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
    computer.a = caps["v"].parse().unwrap();

    let regex = Regex::new(r"Register B: (?P<v>\d+)").unwrap();
    let caps: regex::Captures<'_> = regex.captures(registers.next().unwrap()).unwrap();
    computer.b = caps["v"].parse().unwrap();

    let regex = Regex::new(r"Register C: (?P<v>\d+)").unwrap();
    let caps: regex::Captures<'_> = regex.captures(registers.next().unwrap()).unwrap();
    computer.c = caps["v"].parse().unwrap();

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
