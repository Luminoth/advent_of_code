struct Parser<'a> {
    input: &'a str,
    cur: usize,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, cur: 0 }
    }

    pub fn peek(&self) -> Option<char> {
        if self.cur >= self.input.len() {
            return None;
        }

        Some(self.input.chars().nth(self.cur).unwrap())
    }

    fn advance(&mut self) -> Option<char> {
        if self.cur >= self.input.len() {
            return None;
        }

        let ch = self.input.chars().nth(self.cur).unwrap();
        self.cur += 1;
        Some(ch)
    }

    fn parse_mul(&mut self) -> Option<usize> {
        self.advance();

        if self.peek() == Some('u') {
            self.advance();
        } else {
            return None;
        }

        if self.peek() == Some('l') {
            self.advance();
        } else {
            return None;
        }

        if self.peek() == Some('(') {
            self.advance();
        } else {
            return None;
        }

        // TODO: allocating a string here is bad
        let mut a = String::with_capacity(3);
        while let Some(ch) = self.peek() {
            if !ch.is_ascii_digit() {
                break;
            }

            a.push(ch);
            self.advance();
        }
        let a = a.parse::<usize>().unwrap();

        if self.peek() == Some(',') {
            self.advance();
        } else {
            return None;
        }

        // TODO: allocating a string here is bad
        let mut b = String::with_capacity(3);
        while let Some(ch) = self.peek() {
            if !ch.is_ascii_digit() {
                break;
            }

            b.push(ch);
            self.advance();
        }
        let b = b.parse::<usize>().unwrap();

        if self.peek() == Some(')') {
            self.advance();
        } else {
            return None;
        }

        Some(a * b)
    }

    fn parse_conditional(&mut self) -> Option<bool> {
        self.advance();

        if self.peek() == Some('o') {
            self.advance();
        } else {
            return None;
        }

        if self.peek() == Some('(') {
            self.advance();

            if self.peek() == Some(')') {
                self.advance();
                return Some(true);
            } else {
                return None;
            }
        } else if self.peek() == Some('n') {
            self.advance();

            if self.peek() == Some('\'') {
                self.advance();
            } else {
                return None;
            }

            if self.peek() == Some('t') {
                self.advance();
            } else {
                return None;
            }

            if self.peek() == Some('(') {
                self.advance();
            } else {
                return None;
            }

            if self.peek() == Some(')') {
                return Some(false);
            } else {
                return None;
            }
        }

        None
    }
}

fn part1(input: impl AsRef<str>) {
    let mut parser = Parser::new(input.as_ref());

    let mut sum = 0;
    while let Some(ch) = parser.peek() {
        if ch == 'm' {
            if let Some(v) = parser.parse_mul() {
                sum += v;
            } else {
                parser.advance();
            }
        } else {
            parser.advance();
        }
    }

    assert!(sum == 164730528);
    println!("Total: {}", sum);
}

fn part2(input: impl AsRef<str>) {
    let mut parser = Parser::new(input.as_ref());

    let mut enabled = true;
    let mut sum = 0;
    while let Some(ch) = parser.peek() {
        match ch {
            'm' => {
                if !enabled {
                    parser.advance();
                    continue;
                }

                if let Some(v) = parser.parse_mul() {
                    sum += v;
                } else {
                    parser.advance();
                }
            }
            'd' => {
                if let Some(v) = parser.parse_conditional() {
                    enabled = v;
                } else {
                    parser.advance();
                }
            }
            _ => {
                parser.advance();
            }
        }
    }

    assert!(sum == 70478672);
    println!("Total: {}", sum);
}

fn main() {
    let input = include_str!("../input.txt");

    part1(input);
    part2(input);
}
