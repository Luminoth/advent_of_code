#[derive(Debug, Default, Clone)]
struct Problem {
    operands: Vec<usize>,
    operator: char,
}

impl Problem {
    fn new(size: usize, operator: char) -> Self {
        Self {
            operands: Vec::with_capacity(size),
            operator,
        }
    }

    fn add_operand(&mut self, operand: usize) {
        self.operands.push(operand);
    }

    fn solve(&self) -> usize {
        match self.operator {
            '+' => self.operands.iter().fold(0, |a, &v| a + v),
            '*' => self.operands.iter().fold(1, |a, &v| a * v),
            _ => unreachable!(),
        }
    }
}

fn part1(matrix: impl AsRef<[Vec<&'static str>]>) {
    let matrix = matrix.as_ref();

    let mut problems = Vec::with_capacity(matrix[0].len());
    let size = matrix.len() - 1;
    for y in 0..matrix[0].len() {
        let operand = matrix[matrix.len() - 1][y].chars().nth(0).unwrap();

        let mut problem = Problem::new(size, operand);
        for x in 0..size {
            problem.add_operand(matrix[x][y].parse::<usize>().unwrap());
        }
        problems.push(problem);
    }

    let total: usize = problems
        .iter()
        .map(|problem| {
            let v = problem.solve();
            //println!("v={v}");
            v
        })
        .sum();

    assert!(total == 4878670269096);
    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let matrix = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    part1(&matrix);
}
