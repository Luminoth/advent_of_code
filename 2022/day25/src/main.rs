use std::fmt;

#[derive(Debug)]
struct Snafu {
    snafu: String,
    decimal: i64,
}

impl From<&str> for Snafu {
    fn from(v: &str) -> Self {
        let mut decimal = 0;
        for (i, ch) in v.chars().enumerate() {
            let p = v.len() - 1 - i;
            let n = 5_i64.pow(p as u32);

            if ch.is_ascii_digit() {
                let d = ch.to_digit(10).unwrap() as i64;
                decimal += d * n;
            } else if ch == '-' {
                decimal -= n;
            } else if ch == '=' {
                decimal -= 2 * n;
            }
        }

        Self {
            snafu: v.to_owned(),
            decimal,
        }
    }
}

impl From<i64> for Snafu {
    fn from(v: i64) -> Self {
        let snafu = String::new();
        // TODO:

        Self { snafu, decimal: v }
    }
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.snafu, self.decimal)
    }
}

fn part1(values: impl AsRef<[Snafu]>) {
    /*for value in values.as_ref() {
        println!("{}", value);
    }*/

    let total: i64 = values.as_ref().iter().map(|x| x.decimal).sum();
    println!("Total: {}", Snafu::from(total));
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            Some(Snafu::from(x))
        })
        .collect::<Vec<_>>();

    part1(values);
}
