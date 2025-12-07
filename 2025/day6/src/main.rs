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
            '+' => self.operands.iter().sum::<usize>(),
            '*' => self.operands.iter().product::<usize>(),
            _ => unreachable!(),
        }
    }
}

fn part1(lines: impl AsRef<[&'static str]>) {
    let matrix = lines
        .as_ref()
        .iter()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut problems = Vec::with_capacity(matrix[0].len());

    let size = matrix.len() - 1;
    for y in 0..matrix[0].len() {
        let operand = matrix[size][y].chars().next().unwrap();

        let mut problem = Problem::new(size, operand);
        for v in matrix.iter().take(size) {
            problem.add_operand(v[y].parse::<usize>().unwrap());
        }
        problems.push(problem);
    }

    let total: usize = problems
        .iter()
        .map(|problem| {
            #[allow(clippy::let_and_return)]
            let v = problem.solve();
            //println!("v={v}");
            v
        })
        .sum();

    assert!(total == 4878670269096);
    println!("Total: {}", total);
}

fn part2(_lines: impl AsRef<[&'static str]>) {
    // TODO: the struggle here is that we need to maintain all of the whitespace
    // that would be lost when splitting on whitespace
    // because each value's exactly column matters
    // of note, the operand line tells us where the boundaries are so that may be useful
}

fn main() {
    let input = include_str!("../input.txt");

    let matrix = input.lines().collect::<Vec<_>>();

    part1(&matrix);
    part2(&matrix);
}
