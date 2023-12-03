use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Symbol {
    symbol: char,
    coords: Coords,
}

impl Symbol {
    fn new(symbol: char, x: usize, y: usize) -> Self {
        Self {
            symbol,
            coords: Coords::new(x, y),
        }
    }

    fn is_gear(&self) -> bool {
        self.symbol == '*'
    }
}

fn part2(symbols: HashMap<Symbol, Vec<usize>>) {
    let mut sum = 0;

    for (symbol, parts) in symbols {
        // is this a real gear?
        if !symbol.is_gear() || parts.len() != 2 {
            continue;
        }

        let ratio = parts[0] * parts[1];

        //println!("gear at {:#?} = {}", coords, ratio);
        sum += ratio;
    }

    assert!(sum == 87449461);
    println!("Sum: {}", sum);
}

fn is_symbol(ch: char) -> bool {
    ch != '.' && !ch.is_ascii_digit()
}

// returns all of the symbols this coord touches
fn get_adjacent_symbols(grid: &Vec<Vec<char>>, x: usize, y: usize) -> HashSet<Symbol> {
    let mut symbols = HashSet::new();

    if x > 0 {
        // left
        if is_symbol(grid[y][x - 1]) {
            symbols.insert(Symbol::new(grid[y][x - 1], x - 1, y));
        }

        // upper left
        if y > 0 && is_symbol(grid[y - 1][x - 1]) {
            symbols.insert(Symbol::new(grid[y - 1][x - 1], x - 1, y - 1));
        }

        // lower left
        if y < grid.len() - 1 && is_symbol(grid[y + 1][x - 1]) {
            symbols.insert(Symbol::new(grid[y + 1][x - 1], x - 1, y + 1));
        }
    }

    if x < grid[y].len() - 1 {
        // right
        if is_symbol(grid[y][x + 1]) {
            symbols.insert(Symbol::new(grid[y][x + 1], x + 1, y));
        }

        // upper right
        if y > 0 && is_symbol(grid[y - 1][x + 1]) {
            symbols.insert(Symbol::new(grid[y - 1][x + 1], x + 1, y - 1));
        }

        // lower right
        if y < grid.len() - 1 && is_symbol(grid[y + 1][x + 1]) {
            symbols.insert(Symbol::new(grid[y + 1][x + 1], x + 1, y + 1));
        }
    }

    if y > 0 {
        // upper
        if is_symbol(grid[y - 1][x]) {
            symbols.insert(Symbol::new(grid[y - 1][x], x, y - 1));
        }
    }

    if y < grid.len() - 1 {
        // lower
        if is_symbol(grid[y + 1][x]) {
            symbols.insert(Symbol::new(grid[y + 1][x], x, y + 1));
        }
    }

    symbols
}

// returns all of the symbols and the parts they touch
fn part1(grid: &Vec<Vec<char>>) -> HashMap<Symbol, Vec<usize>> {
    let mut symbols: HashMap<Symbol, Vec<usize>> = HashMap::new();
    let mut parts = vec![];

    for y in 0..grid.len() {
        let line = &grid[y];

        let mut x = 0;
        loop {
            if x >= line.len() {
                break;
            }

            let mut ch = line[x];
            if ch.is_ascii_digit() {
                let mut part_symbols = HashSet::new();
                let mut part = String::new();
                loop {
                    part.push(ch);

                    part_symbols.extend(get_adjacent_symbols(grid, x, y));

                    x += 1;
                    if x >= line.len() {
                        break;
                    }

                    ch = line[x];
                    if !ch.is_ascii_digit() {
                        break;
                    }
                }

                if !part_symbols.is_empty() {
                    let part = part.parse::<usize>().unwrap();
                    //println!("found part {} with {} symbols", part, part_symbols.len());
                    parts.push(part);

                    for symbol in part_symbols {
                        symbols.entry(symbol).or_default().push(part);
                    }
                } else {
                    //println!("{} is not a part", part);
                }
            } else {
                x += 1;
            }
        }
    }

    let sum: usize = parts.iter().sum();

    assert!(sum == 546312);
    println!("Sum: {}", sum);

    symbols
}

fn main() {
    let input = include_str!("../input.txt");

    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let symbols = part1(&grid);
    part2(symbols);
}
