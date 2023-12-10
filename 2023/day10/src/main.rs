use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, strum::EnumString)]
enum PipeType {
    #[strum(serialize = ".")]
    Ground,

    #[strum(serialize = "S")]
    Start,

    #[strum(serialize = "|")]
    Vertical,

    #[strum(serialize = "-")]
    Horizontal,

    #[strum(serialize = "L")]
    NEBend,

    #[strum(serialize = "J")]
    NWBend,

    #[strum(serialize = "7")]
    SWBend,

    #[strum(serialize = "F")]
    SEBend,
}

#[derive(Debug)]
struct Pipe {
    r#type: PipeType,
    coords: (usize, usize),
}

impl From<(usize, usize, char)> for Pipe {
    fn from(v: (usize, usize, char)) -> Self {
        Self {
            r#type: PipeType::from_str(&v.2.to_string()).unwrap(),
            coords: (v.0, v.1),
        }
    }
}

impl Pipe {
    fn resolve_type(&self, grid: &[Vec<Pipe>]) -> PipeType {
        if self.r#type != PipeType::Start {
            return self.r#type;
        }

        // TODO:

        PipeType::Start
    }

    fn is_start(&self) -> bool {
        self.r#type == PipeType::Start
    }

    fn prev(&self, grid: &[Vec<Pipe>]) -> &Pipe {
        let r#type = self.resolve_type(grid);

        // TODO:
        self
    }

    fn next(&self, grid: &[Vec<Pipe>]) -> &Pipe {
        let r#type = self.resolve_type(grid);

        // TODO:
        self
    }
}

fn part1(grid: &[Vec<Pipe>]) {
    let start = grid
        .iter()
        .find_map(|row| {
            row.iter().find_map(|pipe| {
                if pipe.r#type == PipeType::Start {
                    Some(pipe)
                } else {
                    None
                }
            })
        })
        .unwrap();

    let mut node = start;
    let mut distance = 0;
    let mut max_distance = 0;
    loop {
        node = node.next(grid);
        if node.is_start() {
            break;
        }

        distance += 1;
        max_distance = max_distance.max(distance);
    }

    let mut node = start;
    let mut distance = 0;
    let mut max_distance = 0;
    loop {
        node = node.prev(grid);
        if node.is_start() {
            break;
        }

        distance += 1;
        max_distance = max_distance.max(distance);
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let grid = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Pipe::from((x, y, c)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    part1(&grid);
}
