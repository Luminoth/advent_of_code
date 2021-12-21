use std::collections::VecDeque;
use std::fmt;

// I suspect this could be easily solved with a doubly linked list
// [((left, right), depth), ...]
// I believe that would greatly simplify explosion propogation
// while allowing splitting to still be done in-place

#[derive(Debug, Clone)]
enum SnailfishNumberType {
    Number(isize),
    Pair(Box<SnailfishNumber>),
}

impl SnailfishNumberType {
    fn magnitude(&self) -> isize {
        match self {
            Self::Number(number) => *number,
            Self::Pair(pair) => pair.magnitude(),
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
            _ => panic!("invalid split!"),
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
    Exploded,
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

#[derive(Debug, Clone)]
struct SnailfishNumber {
    number: [SnailfishNumberType; 2],
}

impl SnailfishNumber {
    fn magnitude(&self) -> isize {
        3 * self.number[0].magnitude() + 2 * self.number[1].magnitude()
    }

    fn check_explodes(&self, depth: usize) -> Option<(isize, isize)> {
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

    fn reduce_explode(&mut self, depth: usize) -> ExplodeType {
        // explode left?
        if let SnailfishNumberType::Pair(pair) = &mut self.number[0] {
            if let Some((left, right)) = pair.check_explodes(depth + 1) {
                self.number[1].explode(ExplodeType::Right(right));

                let ret = ExplodeType::Left(left);
                self.number[0] = SnailfishNumberType::Number(0);
                return ret;
            }

            // continue down the tree
            let res = pair.reduce_explode(depth + 1);
            match res {
                ExplodeType::Right(_) => {
                    self.number[1].explode(res);
                    return ExplodeType::Exploded;
                }
                ExplodeType::None => (),
                _ => return res,
            }
        }

        // explode right?
        if let SnailfishNumberType::Pair(pair) = &mut self.number[1] {
            if let Some((left, right)) = pair.check_explodes(depth + 1) {
                self.number[0].explode(ExplodeType::Left(left));

                let ret = ExplodeType::Right(right);
                self.number[1] = SnailfishNumberType::Number(0);
                return ret;
            }

            // continue down the tree
            let res = pair.reduce_explode(depth + 1);
            match res {
                ExplodeType::Left(_) => {
                    self.number[0].explode(res);
                    return ExplodeType::Exploded;
                }
                ExplodeType::None => (),
                _ => return res,
            }
        }

        ExplodeType::None
    }

    fn reduce_split(&mut self, depth: usize) -> bool {
        // split left?
        match &mut self.number[0] {
            &mut SnailfishNumberType::Number(number) => {
                if number >= 10 {
                    let number = self.number[0].split();
                    self.number[0] = SnailfishNumberType::Pair(Box::new(Self { number }));
                    return true;
                }
            }
            SnailfishNumberType::Pair(pair) => {
                // continue down the tree
                if pair.reduce_split(depth + 1) {
                    return true;
                }
            }
        }

        // split right?
        match &mut self.number[1] {
            &mut SnailfishNumberType::Number(number) => {
                if number >= 10 {
                    let number = self.number[1].split();
                    self.number[1] = SnailfishNumberType::Pair(Box::new(Self { number }));
                    return true;
                }
            }
            SnailfishNumberType::Pair(pair) => {
                // continue down the tree
                if pair.reduce_split(depth + 1) {
                    return true;
                }
            }
        }

        false
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

fn reduce(number: &mut SnailfishNumber) {
    loop {
        if matches!(number.reduce_explode(0), ExplodeType::None) {
            if !number.reduce_split(0) {
                break;
            }
        }
    }
}

fn part1(mut numbers: VecDeque<SnailfishNumber>) {
    let mut sum = numbers.pop_front().unwrap();
    for number in numbers {
        sum = sum.add(number);
        reduce(&mut sum);
    }

    let magnitude = sum.magnitude();
    assert!(magnitude == 3665);
    println!("Sum magnitude: {}", magnitude);
}

fn part2(numbers: impl AsRef<[SnailfishNumber]>) {
    let numbers = numbers.as_ref();

    let mut max = isize::MIN;

    for i in 0..numbers.len() - 1 {
        let v = numbers[i].clone();
        for number in numbers[i + 1..].iter().cloned() {
            let mut sum = v.clone().add(number);
            reduce(&mut sum);

            let magnitude = sum.magnitude();
            max = max.max(magnitude);
        }
    }

    // addition is not commutative
    // so we have to test the other direction as well
    for i in (1..numbers.len()).rev() {
        let v = numbers[i].clone();
        for number in numbers[..i - 1].iter().cloned() {
            let mut sum = v.clone().add(number);
            reduce(&mut sum);

            let magnitude = sum.magnitude();
            max = max.max(magnitude);
        }
    }

    assert!(max == 4775);
    println!("Max magnitude: {}", max);
}

fn main() {
    let input = include_str!("../input.txt");

    let numbers: Vec<SnailfishNumber> = input
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

    part1(numbers.clone().into());
    part2(numbers);
}
