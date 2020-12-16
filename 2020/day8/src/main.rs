use derivative::Derivative;

#[derive(Debug, Derivative, Clone, Copy, PartialEq, Eq)]
#[derivative(Default)]
enum InstructionType {
    #[derivative(Default)]
    NoOp,
    Accumulate,
    Jump,
}

#[derive(Debug, Default, Clone)]
struct Instruction {
    pub r#type: InstructionType,
    pub value: i64,

    pub execute_count: usize,
}

#[derive(Debug, Default, Clone)]
struct CPU {
    pub accumulator: i64,

    pub counter: i64,

    pub stack: Vec<()>,
    pub stack_pointer: i64,

    pub program: Vec<Instruction>,
}

impl CPU {
    pub fn load<'a>(&mut self, program: impl AsRef<[&'a str]>) {
        for line in program.as_ref() {
            let scratch: Vec<&str> = line.split_whitespace().collect();
            let r#type = match *(scratch.get(0).unwrap()) {
                "nop" => InstructionType::NoOp,
                "acc" => InstructionType::Accumulate,
                "jmp" => InstructionType::Jump,
                _ => panic!("Invalid instruction"),
            };

            let value = scratch.get(1).unwrap().parse().unwrap();

            self.program.push(Instruction {
                r#type,
                value,
                ..Default::default()
            });
        }
    }

    pub fn run(&mut self) -> bool {
        loop {
            match self.program.get_mut(self.counter as usize) {
                None => return true,
                Some(instruction) => {
                    if instruction.execute_count > 0 {
                        return false;
                    }
                    instruction.execute_count += 1;

                    match instruction.r#type {
                        InstructionType::NoOp => self.counter += 1,
                        InstructionType::Accumulate => {
                            self.accumulator += instruction.value;
                            self.counter += 1;
                        }
                        InstructionType::Jump => self.counter += instruction.value,
                    }
                }
            }
        }
    }
}

fn part1<'a>(program: impl AsRef<[&'a str]>) {
    let mut cpu = CPU::default();
    cpu.load(program);

    if !cpu.run() {
        println!("infinite loop detected");
    }

    println!("accumulator: {}", cpu.accumulator);
}

fn part2_change(
    mut cpu: CPU,
    change: usize,
    from: InstructionType,
    to: InstructionType,
) -> Option<i64> {
    let mut idx = 0;
    let mut count = 0;
    loop {
        match cpu.program.get_mut(idx) {
            Some(instruction) => {
                idx += 1;

                if instruction.r#type != from {
                    continue;
                }

                if count == change {
                    instruction.r#type = to;
                    break;
                }
                count += 1;
            }
            None => return None,
        }
    }

    if cpu.run() {
        return Some(cpu.accumulator);
    }

    Some(-1)
}

fn part2<'a>(program: impl AsRef<[&'a str]>) {
    let mut cpu = CPU::default();
    cpu.load(program);

    let mut change = 0;
    loop {
        match part2_change(
            cpu.clone(),
            change,
            InstructionType::NoOp,
            InstructionType::Jump,
        ) {
            Some(acc) => {
                if acc >= 0 {
                    println!("success accumulator: {}", acc);
                    return;
                }
                change += 1;
            }
            None => break,
        }
    }

    let mut change = 0;
    loop {
        match part2_change(
            cpu.clone(),
            change,
            InstructionType::Jump,
            InstructionType::NoOp,
        ) {
            Some(acc) => {
                if acc >= 0 {
                    println!("success accumulator: {}", acc);
                    return;
                }
                change += 1;
            }
            None => break,
        }
    }

    panic!("total failure!");
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.lines().filter(|x| !x.is_empty()).collect();

    part1(&lines);
    part2(&lines);
}
