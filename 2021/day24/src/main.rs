use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Value {
    Value(isize),
    Variable(char),
}

impl From<&'static str> for Value {
    fn from(input: &'static str) -> Self {
        match input.parse() {
            Ok(i) => Self::Value(i),
            Err(_) => Self::Variable(input.trim().chars().next().unwrap()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Input(char),
    Add(char, Value),
    Multiply(char, Value),
    Divide(char, Value),
    Modulus(char, Value),
    Equals(char, Value),
}

impl From<&'static str> for Instruction {
    fn from(input: &'static str) -> Self {
        let v: Vec<&str> = input.split_whitespace().collect();
        let instruction = v[0];
        let var = v[1].chars().next().unwrap();

        match instruction {
            "inp" => Self::Input(var),
            "add" => Self::Add(var, v[2].into()),
            "mul" => Self::Multiply(var, v[2].into()),
            "div" => Self::Divide(var, v[2].into()),
            "mod" => Self::Modulus(var, v[2].into()),
            "eql" => Self::Equals(var, v[2].into()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct AluState {
    w: isize,
    x: isize,
    y: isize,
    z: isize,
}

impl AluState {
    fn get(&self, var: char) -> isize {
        match var {
            'w' => self.w,
            'x' => self.x,
            'y' => self.y,
            'z' => self.z,
            _ => unreachable!(),
        }
    }

    fn set(&mut self, var: char, input: isize) {
        match var {
            'w' => self.w = input,
            'x' => self.x = input,
            'y' => self.y = input,
            'z' => self.z = input,
            _ => unreachable!(),
        }
    }

    fn execute(&mut self, instruction: Instruction, input: usize) -> bool {
        match instruction {
            Instruction::Input(var) => {
                self.input(var, input);
                true
            }
            Instruction::Add(a, b) => {
                self.add(a, b);
                false
            }
            Instruction::Multiply(a, b) => {
                self.multiply(a, b);
                false
            }
            Instruction::Divide(a, b) => {
                self.divide(a, b);
                false
            }
            Instruction::Modulus(a, b) => {
                self.modulus(a, b);
                false
            }
            Instruction::Equals(a, b) => {
                self.equals(a, b);
                false
            }
        }
    }

    fn input(&mut self, var: char, input: usize) {
        assert!(input > 0 && input <= 9);
        self.set(var, input as isize);
    }

    fn add(&mut self, a: char, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => self.get(v),
        };
        let av = self.get(a);
        self.set(a, av + bv);
    }

    fn multiply(&mut self, a: char, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => self.get(v),
        };
        let av = self.get(a);
        self.set(a, av * bv);
    }

    fn divide(&mut self, a: char, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => self.get(v),
        };
        assert!(bv != 0);

        let av = self.get(a);
        self.set(a, av / bv);
    }

    fn modulus(&mut self, a: char, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => self.get(v),
        };
        assert!(bv > 0);

        let av = self.get(a);
        assert!(av >= 0);

        self.set(a, av % bv);
    }

    fn equals(&mut self, a: char, b: Value) {
        let bv = match b {
            Value::Value(v) => v,
            Value::Variable(v) => self.get(v),
        };
        let av = self.get(a);
        self.set(a, isize::from(av == bv));
    }
}

#[derive(Debug, Default)]
struct Alu {
    state: AluState,
}

impl Alu {
    fn z(&self) -> isize {
        self.state.z
    }

    fn execute(&mut self, instruction: Instruction, input: usize) -> bool {
        self.state.execute(instruction, input)
    }
}

fn part1(instructions: impl AsRef<[Instruction]>) {
    let start: usize = 11111111111111;
    let end: usize = 99999999999999;

    let mut cache = HashMap::new();
    let mut states = vec![];

    let largest = (start..end).rev().find(|&x| {
        // super hack, skip any number with a 0 in it
        let scratch = x.to_string();
        if scratch.contains('0') {
            return false;
        }

        let mut alu = Alu::default();

        let mut input = x;
        let mut v = input % 10;

        states.clear();
        for instruction in instructions.as_ref() {
            // if we can go from this state to the end result, do it
            let key = (alu.state, instruction, v);
            if let Some(result) = cache.get(&key) {
                alu.state = *result;
                break;
            }

            let state = alu.state;
            let was_input = alu.execute(*instruction, v);
            states.push((state, instruction, v));

            if was_input {
                input /= 10_usize.pow(1);
                v = input % 10;
            }
        }

        for (state, instruction, input) in &states {
            let key = (*state, *instruction, *input);
            cache.insert(key, alu.state);
        }

        if alu.z() == 0 {
            return true;
        }

        false
    });

    println!("Largest valid model number: {:?}", largest);
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
