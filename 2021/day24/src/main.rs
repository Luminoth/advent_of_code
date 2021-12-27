use std::collections::HashMap;

// TODO: this needs reworked with caching

#[derive(Debug, Copy, Clone)]
enum Value {
    Value(isize),
    Variable(&'static str),
}

impl From<&'static str> for Value {
    fn from(input: &'static str) -> Self {
        match input.parse() {
            Ok(i) => Self::Value(i),
            Err(_) => Self::Variable(input),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Input(&'static str),
    Add(&'static str, Value),
    Multiply(&'static str, Value),
    Divide(&'static str, Value),
    Modulus(&'static str, Value),
    Equals(&'static str, Value),
}

impl From<&'static str> for Instruction {
    fn from(input: &'static str) -> Self {
        let v: Vec<&str> = input.split_whitespace().collect();
        match v[0] {
            "inp" => Self::Input(v[1]),
            "add" => Self::Add(v[1], v[2].into()),
            "mul" => Self::Multiply(v[1], v[2].into()),
            "div" => Self::Divide(v[1], v[2].into()),
            "mod" => Self::Modulus(v[1], v[2].into()),
            "eql" => Self::Equals(v[1], v[2].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct Alu {
    variables: HashMap<String, isize>,
}

impl Alu {
    fn z(&self) -> isize {
        self.variables.get("z").copied().unwrap_or_default()
    }

    fn execute(&mut self, instruction: Instruction, input: usize) -> usize {
        match instruction {
            Instruction::Input(var) => self.input(var, input),
            Instruction::Add(a, b) => {
                self.add(a, b);
                input
            }
            Instruction::Multiply(a, b) => {
                self.multiply(a, b);
                input
            }
            Instruction::Divide(a, b) => {
                self.divide(a, b);
                input
            }
            Instruction::Modulus(a, b) => {
                self.modulus(a, b);
                input
            }
            Instruction::Equals(a, b) => {
                self.equals(a, b);
                input
            }
        }
    }

    fn input(&mut self, var: impl Into<String>, input: usize) -> usize {
        let v = input % 10;
        assert!(v > 0 && v <= 9);

        self.variables.insert(var.into(), v as isize);

        input / 10_usize.pow(1)
    }

    fn add(&mut self, a: impl Into<String>, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => *self.variables.entry(v.into()).or_insert(0),
        };
        let av = self.variables.entry(a.into()).or_insert(0);
        *av += bv;
    }

    fn multiply(&mut self, a: impl Into<String>, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => *self.variables.entry(v.into()).or_insert(0),
        };
        let av = self.variables.entry(a.into()).or_insert(0);
        *av *= bv;
    }

    fn divide(&mut self, a: impl Into<String>, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => *self.variables.entry(v.into()).or_insert(0),
        };
        assert!(bv != 0);

        let av = self.variables.entry(a.into()).or_insert(0);

        *av /= bv;
    }

    fn modulus(&mut self, a: impl Into<String>, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => *self.variables.entry(v.into()).or_insert(0),
        };
        assert!(bv > 0);

        let av = self.variables.entry(a.into()).or_insert(0);
        assert!(*av >= 0);

        *av %= bv;
    }

    fn equals(&mut self, a: impl Into<String>, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => *self.variables.entry(v.into()).or_insert(0),
        };
        let av = self.variables.entry(a.into()).or_insert(0);
        *av = if *av == bv { 1 } else { 0 };
    }
}

fn part1(instructions: impl AsRef<[Instruction]>) {
    let start: usize = 11111111111111;
    let end: usize = 99999999999999;

    for x in (start..=end).rev() {
        // super hack, skip any number with a 0 in it
        let scratch = x.to_string();
        if scratch.contains('0') {
            continue;
        }

        let mut alu = Alu::default();

        let mut input = x;
        for instruction in instructions.as_ref() {
            input = alu.execute(*instruction, input);
        }

        if alu.z() == 0 {
            println!("Largest valid model number: {}", x);
            return;
        }
    }
}

fn main() {
    let input = include_str!("../input.txt").trim();

    let instructions: Vec<Instruction> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(x.into())
        })
        .collect();

    part1(instructions);
}
