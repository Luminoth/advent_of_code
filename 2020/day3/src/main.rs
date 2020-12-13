const TREE: char = '#';

#[derive(Debug, Copy, Clone)]
struct Slope {
    rows: usize,
    cols: usize,
}

impl Slope {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self { rows, cols }
    }
}

fn check_slope<'a>(lines: impl AsRef<[&'a str]>, slope: Slope) -> usize {
    let lines = lines.as_ref();

    let mut row = 0;
    let mut col = 0;

    let mut trees = 0;
    loop {
        row += slope.rows;
        if row >= lines.len() {
            break;
        }

        let line = lines[row];

        col = (col + slope.cols) % line.len();

        if line.chars().nth(col).unwrap() == TREE {
            trees += 1;
        }
    }

    println!("Ran into {} trees", trees);
    return trees;
}

fn part1<'a>(lines: impl AsRef<[&'a str]>) {
    check_slope(lines, Slope::new(1, 3));
}

fn part2<'a>(lines: impl AsRef<[&'a str]>) {
    let slopes = vec![
        Slope::new(1, 1),
        Slope::new(1, 3),
        Slope::new(1, 5),
        Slope::new(1, 7),
        Slope::new(2, 1),
    ];

    let total = slopes
        .iter()
        .fold(1, |acc, s| acc * check_slope(lines.as_ref(), *s));

    println!("Total: {}", total);
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<&str> = input.split('\n').filter(|x| x.len() > 0).collect();

    part1(&lines);
    part2(&lines);
}
