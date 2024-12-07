#[derive(Debug)]
struct Equation {
    result: usize,
    operands: Vec<usize>,
}

// I 100% had to google this ...
fn concat(a: usize, b: usize) -> usize {
    a * 10_usize.pow(b.ilog(10) + 1) + b
}

fn check(operations: &mut [usize], cur: usize, result: usize, part2: bool) -> bool {
    if operations.len() == 1 {
        let v = cur + operations[0];
        if v == result {
            return true;
        }

        let v = cur * operations[0];
        if v == result {
            return true;
        }

        if part2 {
            let v = concat(cur, operations[0]);
            return v == result;
        }

        return false;
    }

    let c = cur + operations[0];
    if check(&mut operations[1..], c, result, part2) {
        return true;
    }

    if part2 {
        let c = concat(cur, operations[0]);
        if check(&mut operations[1..], c, result, part2) {
            return true;
        }
    }

    let c = cur * operations[0];
    if check(&mut operations[1..], c, result, part2) {
        return true;
    }

    if part2 {
        let c = concat(cur, operations[0]);
        if check(&mut operations[1..], c, result, part2) {
            return true;
        }
    }

    false
}

fn part1(equations: &mut [Equation]) {
    let mut total = 0;
    for equation in equations {
        if check(&mut equation.operands, 0, equation.result, false) {
            total += equation.result;
        }
    }

    assert!(total == 1708857123053);
    println!("total: {}", total);
}

fn part2(equations: &mut [Equation]) {
    let mut total = 0;
    for equation in equations {
        if check(&mut equation.operands, 0, equation.result, true) {
            total += equation.result;
        }
    }

    assert!(total == 189207836795655);
    println!("total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let mut equations = input
        .lines()
        .map(|line| {
            let parts = line.split_once(':').unwrap();
            let result = parts.0.parse().unwrap();
            let operands = parts
                .1
                .split_ascii_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            Equation { result, operands }
        })
        .collect::<Vec<_>>();

    part1(&mut equations);
    part2(&mut equations);
}
