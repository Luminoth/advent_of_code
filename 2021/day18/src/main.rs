use std::collections::VecDeque;
use std::fmt;

/*
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]

should produce [[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]
*/

#[derive(Debug)]
enum SnailfishNumberType {
    Number(isize),
    Pair(Box<SnailfishNumber>),
}

impl SnailfishNumberType {
    fn magnitude(&self) -> isize {
        match self {
            Self::Number(number) => *number,
            Self::Pair(number) => number.magnitude(),
        }
    }

    fn explode(&mut self, explosion: ExplodeType) {
        match self {
            Self::Number(number) => *number += explosion.value(),
            Self::Pair(pair) => pair.propagate_explosion(explosion),
        }
    }

    fn split(&self) -> [SnailfishNumberType; 2] {
        match self {
            SnailfishNumberType::Number(number) => [
                SnailfishNumberType::Number((*number as f64 / 2.0).floor() as isize),
                SnailfishNumberType::Number((*number as f64 / 2.0).ceil() as isize),
            ],
            _ => panic!("invalid left split!"),
        }
    }
}

impl fmt::Display for SnailfishNumberType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(number) => write!(f, "{}", number)?,
            Self::Pair(pair) => write!(f, "{}", pair)?,
        }
        Ok(())
    }
}

impl<T: AsRef<str>> From<T> for SnailfishNumberType {
    fn from(input: T) -> Self {
        let input = input.as_ref();

        let ch = input.chars().next().unwrap();
        if ch == '[' {
            Self::Pair(Box::new(input.into()))
        } else if ch.is_ascii_digit() {
            Self::Number(ch.to_digit(10).unwrap() as isize)
        } else {
            panic!("invalid snailfish number: {}", ch);
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ExplodeType {
    Left(isize),
    Right(isize),
    None,
}

impl ExplodeType {
    fn value(&self) -> isize {
        match self {
            Self::Left(value) => *value,
            Self::Right(value) => *value,
            _ => panic!("invalid explosion"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum ReduceAction {
    None,
    Explode(ExplodeType),
    Split,
}

#[derive(Debug)]
struct SnailfishNumber {
    number: [SnailfishNumberType; 2],
}

impl SnailfishNumber {
    fn magnitude(&self) -> isize {
        3 * self.number[0].magnitude() + 2 * self.number[1].magnitude()
    }

    fn check_explode(&self, depth: usize) -> Option<(isize, isize)> {
        if depth >= 4 {
            if let SnailfishNumberType::Number(left) = self.number[0] {
                if let SnailfishNumberType::Number(right) = self.number[1] {
                    return Some((left, right));
                }
            }
        }
        None
    }

    fn propagate_explosion(&mut self, explosion: ExplodeType) {
        match explosion {
            ExplodeType::Left(_) => self.number[1].explode(explosion),
            ExplodeType::Right(_) => self.number[0].explode(explosion),
            _ => panic!("invalid explosion propagation"),
        }
    }

    fn add(self, rhs: SnailfishNumber) -> Self {
        Self {
            number: [
                SnailfishNumberType::Pair(Box::new(self)),
                SnailfishNumberType::Pair(Box::new(rhs)),
            ],
        }
    }

    // TODO: this has a bug in it somewhere that is producing incorrect results
    // on the right half of the tree
    fn reduce(&mut self, depth: usize) -> ReduceAction {
        // explode left?
        if let SnailfishNumberType::Pair(pair) = &self.number[0] {
            if let Some((left, right)) = pair.check_explode(depth + 1) {
                self.number[1].explode(ExplodeType::Right(right));

                let ret = ReduceAction::Explode(ExplodeType::Left(left));
                self.number[0] = SnailfishNumberType::Number(0);
                return ret;
            }
        }

        // explode right?
        if let SnailfishNumberType::Pair(pair) = &self.number[1] {
            if let Some((left, right)) = pair.check_explode(depth + 1) {
                self.number[0].explode(ExplodeType::Left(left));

                let ret = ReduceAction::Explode(ExplodeType::Right(right));
                self.number[1] = SnailfishNumberType::Number(0);
                return ret;
            }
        }

        // reduce left
        if let SnailfishNumberType::Pair(pair) = &mut self.number[0] {
            let res = pair.reduce(depth + 1);
            match res {
                ReduceAction::Split => return ReduceAction::Split,
                ReduceAction::Explode(explosion) => {
                    if matches!(explosion, ExplodeType::Right(_)) {
                        self.number[1].explode(explosion);
                        return ReduceAction::Explode(ExplodeType::None);
                    }
                    return res;
                }
                ReduceAction::None => (),
            }
        }

        // reduce right
        if let SnailfishNumberType::Pair(pair) = &mut self.number[1] {
            let res = pair.reduce(depth + 1);
            match res {
                ReduceAction::Split => return ReduceAction::Split,
                ReduceAction::Explode(explosion) => {
                    if matches!(explosion, ExplodeType::Left(_)) {
                        self.number[0].explode(explosion);
                        return ReduceAction::Explode(ExplodeType::None);
                    }
                    return res;
                }
                ReduceAction::None => (),
            }
        }

        // split left
        if let SnailfishNumberType::Number(number) = &self.number[0] {
            if *number >= 10 {
                let number = self.number[0].split();
                self.number[0] = SnailfishNumberType::Pair(Box::new(Self { number }));
                return ReduceAction::Split;
            }
        }

        // split right
        if let SnailfishNumberType::Number(number) = &self.number[1] {
            if *number >= 10 {
                let number = self.number[1].split();
                self.number[1] = SnailfishNumberType::Pair(Box::new(Self { number }));
                return ReduceAction::Split;
            }
        }

        ReduceAction::None
    }
}

impl fmt::Display for SnailfishNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]", self.number[0], self.number[1])?;
        Ok(())
    }
}

impl<T: AsRef<str>> From<T> for SnailfishNumber {
    fn from(input: T) -> Self {
        let input = input.as_ref();

        let left = input[1..].into();

        let mut level = 0;
        let mut idx = 0;
        for ch in input.chars() {
            if ch == '[' {
                level += 1;
            } else if ch == ']' {
                level -= 1;
            } else if ch == ',' && level == 1 {
                break;
            }
            idx += 1;
        }
        idx += 1;

        let right = input[idx..].into();

        Self {
            number: [left, right],
        }
    }
}

fn part1(mut numbers: VecDeque<SnailfishNumber>) {
    let mut sum = numbers.pop_front().unwrap();
    for number in numbers {
        sum = sum.add(number);
        println!("sum: {}", sum);

        loop {
            if matches!(sum.reduce(0), ReduceAction::None) {
                break;
            }
            println!("reduced: {}", sum);
        }
        println!("step: {}", sum);
    }

    println!("Final sum: {}", sum);
    println!("Sum magnitude: {}", sum.magnitude());
}

fn main() {
    let input = include_str!("../sample.txt");

    let numbers: VecDeque<SnailfishNumber> = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let number = x.into();
            Some(number)
        })
        .collect();

    part1(numbers);
}
