use std::cmp::Ordering;
use std::fmt;

use itertools::{
    EitherOrBoth::{Both, Left, Right},
    Itertools,
};
use nom::{
    branch::alt,
    character::complete::{char, digit1, newline},
    combinator::{all_consuming, map, map_res},
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketValue {
    Integer(usize),
    List(Vec<PacketValue>),
}

impl From<usize> for PacketValue {
    fn from(v: usize) -> Self {
        Self::Integer(v)
    }
}

impl From<Vec<PacketValue>> for PacketValue {
    fn from(v: Vec<PacketValue>) -> Self {
        Self::List(v)
    }
}

impl fmt::Display for PacketValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Integer(v) => write!(f, "{}", v),
            Self::List(v) => write!(f, "[{}]", v.iter().map(|x| x.to_string()).join(",")),
        }
    }
}

impl PacketValue {
    fn compare(&self, other: &PacketValue, _depth: usize) -> Ordering {
        #[cfg(feature = "debugvis")]
        let pad = _depth * 2;
        #[cfg(feature = "debugvis")]
        println!("{:pad$}- Compare {} vs {}", ' ', self, other);

        match self {
            Self::Integer(a) => match other {
                Self::Integer(b) => {
                    if a < b {
                        #[cfg(feature = "debugvis")]
                        println!(
                            "{:pad$}- Left side is smaller, so inputs are in the right order",
                            ' '
                        );
                        return Ordering::Less;
                    }

                    if b < a {
                        #[cfg(feature = "debugvis")]
                        println!(
                            "{:pad$}- Right side is smaller, so inputs are not in the right order",
                            ' '
                        );
                        return Ordering::Greater;
                    }

                    Ordering::Equal
                }
                Self::List(_) => PacketValue::List(vec![(*a).into()]).compare(other, _depth + 1),
            },
            Self::List(a) => match other {
                Self::Integer(b) => self.compare(&vec![(*b).into()].into(), _depth + 1),
                Self::List(b) => {
                    let mut it = a.iter().zip_longest(b.iter());
                    loop {
                        let v = it.next();
                        if v.is_none() {
                            return Ordering::Equal;
                        }

                        match v.unwrap() {
                            Both(a, b) => {
                                let r = a.compare(b, _depth + 1);
                                if r != Ordering::Equal {
                                    return r;
                                }
                            }
                            Left(_) => {
                                #[cfg(feature = "debugvis")]
                                println!(
                                    "{:pad$}- Right side ran out of items, so inputs are not in the right order", ' '
                                );
                                return Ordering::Greater;
                            }
                            Right(_) => {
                                #[cfg(feature = "debugvis")]
                                println!("{:pad$}- Left side ran out of items, so inputs are in the right order", ' ');
                                return Ordering::Less;
                            }
                        }
                    }
                }
            },
        }
    }
}

fn parse_packet_value_integer(input: &str) -> IResult<&str, PacketValue> {
    map(map_res(digit1, str::parse::<usize>), Into::into)(input)
}

fn parse_packet_values(input: &str) -> IResult<&str, Vec<PacketValue>> {
    separated_list0(char(','), parse_packet_value)(input)
}

fn parse_packet_value_list(input: &str) -> IResult<&str, PacketValue> {
    map(
        delimited(char('['), parse_packet_values, char(']')),
        Into::into,
    )(input)
}

fn parse_packet_value(input: &str) -> IResult<&str, PacketValue> {
    alt((parse_packet_value_integer, parse_packet_value_list))(input)
}

fn parse_packets(input: &str) -> IResult<&str, (PacketValue, PacketValue)> {
    separated_pair(parse_packet_value_list, newline, parse_packet_value_list)(input)
}

fn part1(packets: impl AsRef<[(PacketValue, PacketValue)]>) {
    let mut total = 0;
    for (i, (a, b)) in packets.as_ref().iter().enumerate() {
        #[cfg(feature = "debugvis")]
        println!("== Pair {} ==", i + 1);

        if a.compare(b, 0) != Ordering::Greater {
            total += i + 1;
        }
    }

    assert!(total == 5588);
    println!("Correctly ordered pairs total: {}", total);
}

fn part2(mut packets: Vec<PacketValue>) {
    packets.push(vec![vec![2.into()].into()].into());
    packets.push(vec![vec![6.into()].into()].into());

    packets.sort_by(|a, b| a.compare(b, 0));

    #[cfg(feature = "debugvis")]
    {
        let message = packets.iter().map(|x| x.to_string()).join("\n");
        println!("Message:\n{}", message);
    }

    let ai = packets
        .iter()
        .position(|x| match x {
            PacketValue::List(x) => {
                if x.is_empty() {
                    false
                } else {
                    match &x[0] {
                        PacketValue::List(y) => {
                            if y.is_empty() {
                                false
                            } else {
                                match y[0] {
                                    PacketValue::Integer(v) => v == 2,
                                    _ => false,
                                }
                            }
                        }
                        _ => false,
                    }
                }
            }
            PacketValue::Integer(_) => false,
        })
        .unwrap()
        + 1;

    let bi = packets
        .iter()
        .position(|x| match x {
            PacketValue::List(x) => {
                if x.is_empty() {
                    false
                } else {
                    match &x[0] {
                        PacketValue::List(y) => {
                            if y.is_empty() {
                                false
                            } else {
                                match y[0] {
                                    PacketValue::Integer(v) => v == 6,
                                    _ => false,
                                }
                            }
                        }
                        _ => false,
                    }
                }
            }
            PacketValue::Integer(_) => false,
        })
        .unwrap()
        + 1;

    let total = ai * bi;
    assert!(total == 23958);
    println!("Decoder key ({}, {}): {}", ai, bi, total);
}

fn main() {
    let input = include_str!("../input.txt");

    let values = input
        .trim()
        .split("\n\n")
        .map(|x| all_consuming(parse_packets)(x).finish().unwrap().1)
        .collect::<Vec<_>>();

    part1(&values);
    part2(
        // this sucks lol
        values
            .iter()
            .cloned()
            .flat_map(|(a, b)| vec![a, b])
            .collect::<Vec<_>>(),
    );
}
