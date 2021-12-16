#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum OperatorType {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl From<usize> for OperatorType {
    fn from(input: usize) -> Self {
        match input {
            0 => Self::Sum,
            1 => Self::Product,
            2 => Self::Minimum,
            3 => Self::Maximum,
            5 => Self::GreaterThan,
            6 => Self::LessThan,
            7 => Self::Equal,
            _ => panic!("invalid operator type"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum PacketType {
    Literal,
    Operator(OperatorType),
}

impl<T: AsRef<str>> From<T> for PacketType {
    fn from(input: T) -> Self {
        let op = usize::from_str_radix(input.as_ref(), 2).unwrap();
        match op {
            4 => PacketType::Literal,
            _ => PacketType::Operator(op.into()),
        }
    }
}

#[derive(Debug)]
enum OperatorLengthType {
    Length,
    Count,
}

#[derive(Debug)]
enum PacketPayload {
    Literal(usize, usize),
    Operator(OperatorLengthType, Vec<Packet>),
}

impl PacketPayload {
    fn new(r#type: PacketType, input: impl AsRef<str>) -> Self {
        match r#type {
            PacketType::Literal => Self::new_literal(input),
            PacketType::Operator(_) => Self::new_operator(input),
        }
    }

    fn new_literal(input: impl AsRef<str>) -> Self {
        let input = input.as_ref();

        let mut literal = String::new();

        let mut consumed = 0;
        loop {
            let start = consumed + 1;
            let end = start + 4;

            literal.push_str(&input[start..end]);
            if input.chars().nth(consumed).unwrap() == '0' {
                break;
            }
            consumed += 5;
        }
        consumed += 5;

        Self::Literal(consumed, usize::from_str_radix(&literal, 2).unwrap())
    }

    fn new_operator(input: impl AsRef<str>) -> Self {
        let input = input.as_ref();

        let mut packets = Vec::new();
        let r#type = match input.chars().next().unwrap() {
            '0' => {
                let length = usize::from_str_radix(&input[1..16], 2).unwrap();

                let mut consumed = 0;
                while consumed < length {
                    let start = 16 + consumed;

                    let packet: Packet = (&input[start..]).into();

                    consumed += packet.encoded_len();

                    packets.push(packet);
                }

                OperatorLengthType::Length
            }
            '1' => {
                let count = usize::from_str_radix(&input[1..12], 2).unwrap();

                let mut consumed = 0;
                while packets.len() < count {
                    let start = 12 + consumed;

                    let packet: Packet = (&input[start..]).into();

                    consumed += packet.encoded_len();

                    packets.push(packet);
                }

                OperatorLengthType::Count
            }
            _ => panic!("invalid packet length type ID"),
        };

        Self::Operator(r#type, packets)
    }

    fn encoded_len(&self) -> usize {
        match self {
            Self::Literal(size, _) => *size,
            Self::Operator(r#type, packets) => {
                let mut length = 1;
                length += match r#type {
                    OperatorLengthType::Length => 15,
                    OperatorLengthType::Count => 11,
                };
                length += packets.iter().map(|p| p.encoded_len()).sum::<usize>();

                length
            }
        }
    }

    fn version_total(&self) -> usize {
        match self {
            Self::Literal(_, _) => 0,
            Self::Operator(_, packets) => packets.iter().map(|p| p.version_total()).sum::<usize>(),
        }
    }

    fn value(&self, op: Option<OperatorType>) -> usize {
        match self {
            Self::Literal(_, value) => *value,
            Self::Operator(_, packets) => match op.unwrap() {
                OperatorType::Sum => packets.iter().map(|p| p.value()).sum(),
                OperatorType::Product => packets.iter().map(|p| p.value()).product(),
                OperatorType::Minimum => packets.iter().map(|p| p.value()).min().unwrap(),
                OperatorType::Maximum => packets.iter().map(|p| p.value()).max().unwrap(),
                OperatorType::GreaterThan => {
                    if packets[0].value() > packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::LessThan => {
                    if packets[0].value() < packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
                OperatorType::Equal => {
                    if packets[0].value() == packets[1].value() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

#[derive(Debug)]
struct Packet {
    version: usize,
    r#type: PacketType,
    payload: PacketPayload,
}

impl Packet {
    fn encoded_len(&self) -> usize {
        3 + 3 + self.payload.encoded_len()
    }

    fn version_total(&self) -> usize {
        self.version + self.payload.version_total()
    }

    fn value(&self) -> usize {
        match self.r#type {
            PacketType::Literal => self.payload.value(None),
            PacketType::Operator(op) => self.payload.value(Some(op)),
        }
    }
}

impl<T: AsRef<str>> From<T> for Packet {
    fn from(input: T) -> Self {
        let input = input.as_ref();

        let version = usize::from_str_radix(&input[..3], 2).unwrap();
        let r#type = input[3..6].into();
        let payload = PacketPayload::new(r#type, &input[6..]);

        Self {
            version,
            r#type,
            payload,
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let codes: String = input
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let code: String = x
                .chars()
                .map(|ch| ch.to_digit(16).unwrap() as u8)
                .map(|v| format!("{:04b}", v))
                .collect();
            Some(code)
        })
        .collect();

    let transmission: Packet = codes.into();

    let version_total = transmission.version_total();
    assert!(version_total == 821);
    println!("Transmission version total: {}", version_total);

    let value = transmission.value();
    assert!(value == 2056021084691);
    println!("Transmission value: {}", value);
}
