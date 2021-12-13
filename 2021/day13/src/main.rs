use std::fmt;

#[derive(Debug, Clone)]
struct Grid {
    grid: Vec<Vec<bool>>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for v in row {
                write!(f, "{}", if *v { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Vec<(usize, usize)>> for Grid {
    fn from(input: Vec<(usize, usize)>) -> Self {
        let (width, height) = input
            .iter()
            .fold((0, 0), |a, &v| (a.0.max(v.0), a.1.max(v.1)));
        assert!(width > 0 && height > 0);

        let mut grid = vec![vec![false; width + 1]; height + 1];
        for v in input {
            grid[v.1][v.0] = true;
        }

        Self { grid }
    }
}

impl Grid {
    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }

    fn dot_count(&self) -> usize {
        self.grid
            .iter()
            .map(|y| y.iter().filter(|&&x| x))
            .flatten()
            .count()
    }

    fn fold(&self, folds: impl AsRef<[Fold]>) -> Self {
        let fold = folds.as_ref()[0];

        let (width, height) = match fold {
            Fold::Horizontal(y) => (self.width(), self.height() - y - 1),
            Fold::Vertical(x) => (self.width() - x - 1, self.height()),
        };

        let grid: Vec<Vec<bool>> = self
            .grid
            .iter()
            .take(height)
            .cloned()
            .map(|mut x| {
                x.truncate(width);
                x
            })
            .collect();
        let mut folded = Self { grid };

        match fold {
            Fold::Horizontal(v) => {
                for y in self.height() - v..self.height() {
                    for x in 0..self.width() {
                        let yf = y - v - 1;
                        let ys = self.height() - yf - 1;
                        folded.grid[yf][x] |= self.grid[ys][x];
                    }
                }
            }
            Fold::Vertical(v) => {
                for y in 0..self.height() {
                    for x in self.width() - v..self.width() {
                        let xf = x - v - 1;
                        let xs = self.width() - xf - 1;
                        folded.grid[y][xf] |= self.grid[y][xs];
                    }
                }
            }
        };

        if folds.as_ref().len() > 1 {
            folded.fold(&folds.as_ref()[1..])
        } else {
            folded
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

fn main() {
    let input = include_str!("../input.txt");

    let (dots, folds) = input.split_once("\n\n").unwrap();
    let dots: Vec<(usize, usize)> = dots
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let (x, y) = x.split_once(',').unwrap();
            Some((x.parse().unwrap(), y.parse().unwrap()))
        })
        .collect();

    let folds: Vec<Fold> = folds
        .lines()
        .filter_map(|x| {
            let x = x.trim();
            if x.is_empty() {
                return None;
            }

            let (direction, v) = x.split_once('=').unwrap();
            Some(match direction {
                "fold along x" => Fold::Vertical(v.parse().unwrap()),
                "fold along y" => Fold::Horizontal(v.parse().unwrap()),
                _ => unreachable!(),
            })
        })
        .collect();

    let grid: Grid = dots.into();

    let folded = grid.fold(&folds[..1]);
    let dot_count = folded.dot_count();
    assert!(dot_count == 743);
    println!("There are {} dots visible after 1 fold", dot_count);

    let folded = grid.fold(&folds);
    let dot_count = folded.dot_count();
    assert!(dot_count == 94);
    println!("There are {} dots visible after all folds", dot_count);
    /*
    ###...##..###..#.....##..#..#.#..#.#....
    #..#.#..#.#..#.#....#..#.#.#..#..#.#....
    #..#.#....#..#.#....#..#.##...####.#....
    ###..#....###..#....####.#.#..#..#.#....
    #.#..#..#.#....#....#..#.#.#..#..#.#....
    #..#..##..#....####.#..#.#..#.#..#.####.
    */
    println!("{}", folded);
}
